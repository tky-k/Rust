# Rust
tutorial for rust

# Install
install rustup acording to below site.  
- [Install Rust](https://www.rust-lang.org/tools/install)

recomend extention
- [Rust-analyzer](https://rust-analyzer.github.io/manual.html)

# Refferenct
- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/title-page.html)


# Use Docker

docker build
```
$ docker build -t rustup:1.0 .
```

docker rum

```
$ docker run -td --rm -v `pwd`:/tmp --name rust_exsample rustup:1.0
$ docker exec -it rust_exsample /bin/bash
```

docker stop
```
$ docker stop rust_exsample
```

# Official Document
- https://doc.rust-jp.rs/book-ja/title-page.html