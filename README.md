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

To access the underlying function, use `real!(existing_function)`. Then
compile your library as a `dylib`.

There are a few [examples](examples) included, as separate crates.

At the moment, redhook only supports `LD_PRELOAD` on glibc, but support
for other platforms is planned.

redhook is named after the [Red Hook](http://en.wikipedia.org/wiki/Red_Hook,_Brooklyn)
neighborhood in Brooklyn, New York.
