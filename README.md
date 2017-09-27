# readfile1-rs

This is just an example of a rust program that uses the standard IO
module to read a file into a vector data structure.  It demonstrates
the complexity to properly handle errors.

Once the file is read, the program prints out how many bytes were
read and a classic hex dump of the file's bytes with the printable
ASCII representation on the right.

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
$ cargo run ./Cargo.toml
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/readfile1-rs ./Cargo.toml`
reading file name: ./Cargo.toml.. 112 bytes read.

simple hexdump (dot <.> denotes unprintable char)

87654321 00 11 22 33 44 55 66 77 88 99 aa bb cc dd ee ff  |0123456789abcdef|
-------- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --  |----------------|
00000000 5b 70 61 63 6b 61 67 65 5d 0a 6e 61 6d 65 20 3d  |[package].name =|
00000010 20 22 72 65 61 64 66 69 6c 65 31 2d 72 73 22 0a  | "readfile1-rs".|
00000020 76 65 72 73 69 6f 6e 20 3d 20 22 30 2e 31 2e 30  |version = "0.1.0|
00000030 22 0a 61 75 74 68 6f 72 73 20 3d 20 5b 22 44 61  |".authors = ["Da|
00000040 6e 20 4d 61 63 6b 20 3c 6d 61 63 6b 40 6d 61 63  |n Mack <mack@mac|
00000050 6b 74 72 6f 6e 69 63 73 2e 63 6f 6d 3e 22 5d 0a  |ktronics.com>"].|
00000060 0a 5b 64 65 70 65 6e 64 65 6e 63 69 65 73 5d 0a  |.[dependencies].|

hexdump of 112 bytes completed.
```

Pretty neat, huh?

This demonstration shows how to:
* handle a function which returns Option type like std::env::args().nth()
  * Option types return Some() / None
* handle a function which returns a Result type like fs::File::open()
  * Result types return Ok() / Err()
* use low level file IO from rust
* populate a buffer safely leveraging the vector (Vec) built in type
* how to use match with error handling

Hope this is useful.  I think points out how syntactically complex
rust can be to do some very simple things as compared to C and higher
level languages.

This worked with rust version 1.20.


