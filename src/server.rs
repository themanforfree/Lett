use crate::{config::Config, router};
use anyhow::Result;
use core::task::{Context, Poll};
use futures_util::ready;
use hyper::{
    server::{
        accept::Accept,
        conn::{AddrIncoming, AddrStream},
    },
    service::{make_service_fn, service_fn},
    Server,
};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::Item;
use std::{
    convert::Infallible,
    fs::{self, File},
    io::{self, BufReader},
    pin::Pin,
    sync,
    sync::Arc,
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use std::{future::Future, vec::Vec};

pub async fn run(cfg: &Config) -> Result<()> {
    let addr = cfg.application.listen;
    if !cfg.application.tls {
        let service =
            make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router::handle)) });
        let server = Server::bind(&addr).serve(service);
        log::debug!("Listening on http://{}", addr);
        return server.await.map_err(Into::into);
    }
    let tls_cfg = {
        let certs = load_certs(&cfg.application.certs)?;

        let key = load_private_key(&cfg.application.key)?;
        let mut cfg = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)?;
        cfg.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        sync::Arc::new(cfg)
    };

    let incoming = AddrIncoming::bind(&addr)?;
    let service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router::handle)) });
    let server = Server::builder(TlsAcceptor::new(tls_cfg, incoming)).serve(service);

    log::debug!("Listening on https://{}", addr);
    server.await?;
    Ok(())
}

enum State {
    Handshaking(tokio_rustls::Accept<AddrStream>),
    Streaming(tokio_rustls::server::TlsStream<AddrStream>),
}

pub struct TlsStream {
    state: State,
}

impl TlsStream {
    fn new(stream: AddrStream, config: Arc<ServerConfig>) -> TlsStream {
        let accept = tokio_rustls::TlsAcceptor::from(config).accept(stream);
        TlsStream {
            state: State::Handshaking(accept),
        }
    }
}

impl AsyncRead for TlsStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut ReadBuf,
    ) -> Poll<io::Result<()>> {
        let pin = self.get_mut();
        match pin.state {
            State::Handshaking(ref mut accept) => match ready!(Pin::new(accept).poll(cx)) {
                Ok(mut stream) => {
                    let result = Pin::new(&mut stream).poll_read(cx, buf);
                    pin.state = State::Streaming(stream);
                    result
                }
                Err(err) => Poll::Ready(Err(err)),
            },
            State::Streaming(ref mut stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for TlsStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let pin = self.get_mut();
        match pin.state {
            State::Handshaking(ref mut accept) => match ready!(Pin::new(accept).poll(cx)) {
                Ok(mut stream) => {
                    let result = Pin::new(&mut stream).poll_write(cx, buf);
                    pin.state = State::Streaming(stream);
                    result
                }
                Err(err) => Poll::Ready(Err(err)),
            },
            State::Streaming(ref mut stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.state {
            State::Handshaking(_) => Poll::Ready(Ok(())),
            State::Streaming(ref mut stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.state {
            State::Handshaking(_) => Poll::Ready(Ok(())),
            State::Streaming(ref mut stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}

pub struct TlsAcceptor {
    config: Arc<ServerConfig>,
    incoming: AddrIncoming,
}

impl TlsAcceptor {
    pub fn new(config: Arc<ServerConfig>, incoming: AddrIncoming) -> TlsAcceptor {
        TlsAcceptor { config, incoming }
    }
}

impl Accept for TlsAcceptor {
    type Conn = TlsStream;
    type Error = io::Error;
    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        let pin = self.get_mut();
        match ready!(Pin::new(&mut pin.incoming).poll_accept(cx)) {
            Some(Ok(sock)) => Poll::Ready(Some(Ok(TlsStream::new(sock, pin.config.clone())))),
            Some(Err(e)) => Poll::Ready(Some(Err(e))),
            None => Poll::Ready(None),
        }
    }
}

pub fn load_certs(path: &str) -> Result<Vec<Certificate>> {
    let mut file = BufReader::new(File::open(path)?);
    let mut certs = Vec::new();
    while let Ok(Some(item)) = rustls_pemfile::read_one(&mut file) {
        if let Item::X509Certificate(cert) = item {
            certs.push(Certificate(cert));
        }
    }
    if certs.is_empty() {
        certs = vec![Certificate(fs::read(path)?)];
    }
    Ok(certs)
}

pub fn load_private_key(path: &str) -> Result<PrivateKey> {
    let mut file = BufReader::new(File::open(path)?);
    let mut priv_key = None;
    while let Ok(Some(item)) = rustls_pemfile::read_one(&mut file) {
        if let Item::RSAKey(key) | Item::PKCS8Key(key) | Item::ECKey(key) = item {
            priv_key = Some(key);
        }
    }
    priv_key
        .map(Ok)
        .unwrap_or_else(|| fs::read(path))
        .map(PrivateKey)
        .map_err(Into::into)
}
