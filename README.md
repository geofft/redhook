redhook
=======

redhook is a helper crate for writing interposition libraries
(`LD_PRELOAD`, `DYLD_INSERT_LIBRARIES`, etc.) in Rust.

To use redhook, edit your `Cargo.toml` to add redhook as a dependency
and configure your library to build as a `dylib`:

```toml
[dependencies]
redhook = "1.0"

[lib]
name = "mylib"
crate_type = ["dylib"]
```

Then use the `hook!` macro to declare the function you want to hook, and
the name you want to give to your hook function:

```rust
redhook::hook! {
    unsafe fn existing_function(x: i32) -> i32 => my_function {
        42
    }
}
```

To access the underlying function, use `redhook::real!(existing_function)`.

Build your library as usual, find the resulting `.so` file (or `.dylib`,
for macOS) inside the `target` directory, and set the `LD_PRELOAD` (or
`DYLD_INSERT_LIBRARIES`) environment variable to that.

(You can also use redhook without Cargo, with `rustc --crate-type=dylib`.)

There are a few [examples](examples) included, as separate crates:

```
geofft@cactuar:~/src/rust/redhook/examples/readlinkspy$ cargo build
   Compiling redhook v0.0.1 (file:///home/geofft/src/rust/redhook/examples/readlinkspy)
   Compiling redhook_ex_readlinkspy v0.0.1 (file:///home/geofft/src/rust/redhook/examples/readlinkspy)
geofft@cactuar:~/src/rust/redhook/examples/readlinkspy$ LD_PRELOAD=target/debug/libreadlinkspy.so ls -l /bin/sh
readlink("/bin/sh")
lrwxrwxrwx 1 root root 4 Feb 19  2014 /bin/sh -> dash
```

```
geofft@cactuar:~/src/rust/redhook/examples/fakeroot$ cargo build
geofft@cactuar:~/src/rust/redhook/examples/fakeroot$ LD_PRELOAD=target/debug/libfakeroot.so id
uid=0(root) gid=1001(geofft) euid=1001(geofft) groups=1001(geofft),27(sudo),111(sbuild)
```

redhook currently supports building interposition libraries for
`LD_PRELOAD` on glibc (GNU/Linux) and `DYLD_INSERT_LIBRARIES` on Apple's
libc (macOS) from the same source code. If you're interested in
support for other platforms, please file an issue or pull request.

redhook is named after the [Red Hook](http://en.wikipedia.org/wiki/Red_Hook,_Brooklyn)
neighborhood in Brooklyn, New York. Portions of the implementation
borrow heavily from concepts in @Kimundi's
[`lazy_static!`](https://github.com/Kimundi/lazy-static.rs) macro.

redhook is free software, available under the terms of the
[2-clause BSD license](COPYING).

[![Build Status](https://travis-ci.org/geofft/redhook.svg?branch=master)](https://travis-ci.org/geofft/redhook)
