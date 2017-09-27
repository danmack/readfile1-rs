# readfile1-rs

This is just an example of a rust program that uses the standard IO
module to read a file into a vector data structure.  It demonstrates
the complexity to properly handle errors.

Once the file is read, the program prints out how many bytes were
read and a basic hex dump of the file.

You can run the program using cargo like this:

```shell
$ cargo run <filename>
```

Examples of error and ok results:

1. no argument given specifying the file name to read:

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/readfile1-rs`
thread 'main' panicked at 'no file argument present', main.rs:28:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

2. argument given, but file cannot be opened because it is missing or
unreadable:

```shell
$ cargo run bad_fname
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/readfile1-rs bad_fname`
thread 'main' panicked at 'failed to open file: No such file or directory (os error 2)', main.rs:23:20
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

3. successful file read (I just passed in this project's Cargo.toml
file):

```shell
$ cargo run Cargo.toml
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/readfile1-rs Cargo.toml`
reading file name: Cargo.toml.. 112 bytes read.
5B 70 61 63 6B 61 67 65 5D 0A 6E 61 6D 65 20 3D
20 22 72 65 61 64 66 69 6C 65 31 2D 72 73 22 0A
76 65 72 73 69 6F 6E 20 3D 20 22 30 2E 31 2E 30
22 0A 61 75 74 68 6F 72 73 20 3D 20 5B 22 44 61
6E 20 4D 61 63 6B 20 3C 6D 61 63 6B 40 6D 61 63
6B 74 72 6F 6E 69 63 73 2E 63 6F 6D 3E 22 5D 0A
0A 5B 64 65 70 65 6E 64 65 6E 63 69 65 73 5D 0A
```

Pretty neat, huh?

This demostration shows how to:
* handle a function which returns Option type like std::env::args().nth()
  * Option types return Some() / None
* handle a function which returns a Result type like fs::File::open()
  * Result types return Ok() / Err()
* use low level file IO from rust
* populate a buffer safely leveraging the vector (Vec) built in type
* how to use match with error handling

Hope this is useful.  I think points out how syntaxtically complex
rust can be to do some very simple things as compared to C and higher
level languages.

This worked with rust version 1.20.


