# How to Use Cargo
### create project with cargo
type below command
```
$ cargo new $PROJECT_DIR --bin
```

### build source with cargo
move to target directory and build them.
```
$ cd $PROJECT_DIR
$ cargo build
```

cargo outputs execute file to target/debug/rust_tutorial.  
and then execute compiled program.
```
$ ./target/debug/rust_tutorial
```

### build source for release with cargo
move to target directory and build them with --release option.
```
$ cd $PROJECT_DIR
$ cargo build --release
```

cargo outputs release file to target/release/rust_tutorial.
what's difference between debug file and release file is performance.  
release file is optimaized by rustc compilser.  
release file performance is better than debug one.  
but you need more time for compiling release file than debug one.