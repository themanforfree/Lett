# lett

一个用 RUST 实现的简易博客系统

## 特性

* [X] rust 实现，性能优秀
* [X] 支持 markdown 渲染
* [X] 可自定义展示模板
* [X] 使用 mysql 数据库保存数据
* [X] 后台管理页面，快速发布或修改文章
* [X] 静态文件路由


## 使用

初始化数据库    

可以手动创建或使用 `diesel` 工具创建

SQL语句均保存于 [migrations](migrations) 目录下  
```bash
diesel setup
diesel migration run
```

安装依赖   

```bash
# Debian/Ubuntu
apt install libmysqlclient-dev
```

程序同级目录创建 .env 文件

```
DATABASE_URL=mysql://[username]:[password]@locakhost/database_name
```
或直接添加到环境变量

```bash
export DATABASE_URL=mysql://[username]:[password]@locakhost/database_name
```

运行
```bash
lett
```

## TODO

* [ ] 配置文件
* [ ] 评论
* [ ] tls 支持

## License
GNU General Public License v3.0
