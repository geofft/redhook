#[macro_use]
extern crate redhook;

hook! {
    fn getuid() -> u64 => i_am_root {
        0
    }
}
