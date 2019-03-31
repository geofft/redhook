redhook::hook! {
    // Can't have use-after-free vulnerabilities... if you never free anything
    unsafe fn free(_ptr: *const ()) => my_free { }
}
