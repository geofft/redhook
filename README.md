redhook
=======

redhook is a helper crate for writing interposition libraries
(`LD_PRELOAD`, `DYLD_INSERT_LIBRARIES`, etc.) in Rust.

To use redhook, add the lines

```rust
#[macro_use]
extern crate redhook;
```

to your library, then use the `hook!` macro to declare the function you
want to hook, and the name you want to give to your hook function:

```rust
hook! {
    fn existing_function(x: i32) -> i32 => my_function {
        42
    }
}
```

To access the underlying function, use `real!(existing_function)`.

Compile your library as a `dylib`, which can be done via
`rustc --crate-type=dylib`, or by adding

```
[lib]
name = "mylib"
crate_type = ["dylib"]
```

to your `Cargo.toml`. Then find the resulting `.so` file (`./mylib.so`
for rustc, or inside `target` for Cargo) and set the `LD_PRELOAD`
environment variable to that.

There are a few [examples](examples) included, as separate crates:

```
geofft@cactuar:~/src/rust/redhook/examples/readlinkspy$ cargo build
   Compiling redhook v0.0.1 (file:///home/geofft/src/rust/redhook/examples/readlinkspy)
   Compiling redhook_ex_readlinkspy v0.0.1 (file:///home/geofft/src/rust/redhook/examples/readlinkspy)
geofft@cactuar:~/src/rust/redhook/examples/readlinkspy$ LD_PRELOAD=target/debug/libreadlinkspy-7cb7c673fbb62878.so ls -l /bin/sh
readlink("/bin/sh")
lrwxrwxrwx 1 root root 4 Feb 19  2014 /bin/sh -> dash
```

```
geofft@cactuar:~/src/rust/redhook/examples/fakeroot$ cargo build
geofft@cactuar:~/src/rust/redhook/examples/fakeroot$ LD_PRELOAD=target/debug/libfakeroot-1be6565b67838e2e.so id
uid=0(root) gid=1001(geofft) euid=1001(geofft) groups=1001(geofft),27(sudo),111(sbuild)
```

At the moment, redhook only supports `LD_PRELOAD` on glibc, but support
for other platforms is planned.

redhook is named after the [Red Hook](http://en.wikipedia.org/wiki/Red_Hook,_Brooklyn)
neighborhood in Brooklyn, New York. Portions of the implementation
borrow heavily from concepts in @Kimundi's
[`lazy_static!`](https://github.com/Kimundi/lazy-static.rs) macro.

