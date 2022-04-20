# lett

一个用 RUST 实现的简易博客系统

## 特性

* [X] rust 实现，性能优秀
* [X] 支持 markdown 渲染
* [X] 可自定义展示模板
* [X] 使用 mysql 数据库保存数据
* [X] 后台管理页面，快速发布或修改文章
* [X] 静态文件路由
* [X] 配置文件
* [X] tls 支持
* [X] 评论


## 使用

安装依赖   

```bash
# Debian/Ubuntu
apt install libmysqlclient-dev
```

创建数据库,在 `MySQL` 中执行sql语句
```SQL
CREATE DATABASE database_name;
```

配置文件模板
```toml
[application]
# The socket address to bind
listen = "0.0.0.0:3000"
# TimeZone
timezone = "+08:00"
# enable/disable tls
tls = true
# certificate file path
certs = "sample.pem"
# key file path
key = "sample.rsa"

[database]
# Database url
url = "mysql://username:password@localhost/database_name"

[site]
# The site name
name = "Example Name"
# The site URL
url = "http://example.com"
# The site description
description = "Example Description"
```

运行
```bash
lett -c config.toml
```

## License
GNU General Public License v3.0
