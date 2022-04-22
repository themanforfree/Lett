# lett

一个用 Rust 实现的简易博客系统

## 特性

* [X] Rust 实现，性能优秀
* [X] 支持 markdown 渲染
* [X] 支持 jinja2 模板
* [X] 支持 tls 
* [X] 支持配置文件
* [X] 支持静态文件路由
* [X] 使用 diesel 抽象化包装 MySQL 接口以抵御 SQL 注入。
* [X] 后台管理页面，快速发布或修改文章


## 使用

安装依赖   

```bash
# Arch
pacman -S libmysqlclient

# Debian/Ubuntu
apt install libmysqlclient-dev
```

创建数据库,在 `MySQL` 中执行sql语句
```SQL
CREATE DATABASE database_name;
```

快速创建数据表
```bash
lett --install mysql://username:password@localhost/database_name
```


运行
```bash
lett -c config.toml
```

配置文件模板
```toml
[application]
# The socket address to bind
listen = "0.0.0.0:3000"
# TimeZone
timezone = "+08:00"
# Time format
time_format = "[year]-[month]-[day] [hour]:[minute]:[second]"
# Template path
template_path = "templates"
# enable/disable tls
tls = false
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

## License
GNU General Public License v3.0
