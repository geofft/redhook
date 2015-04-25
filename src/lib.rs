#![feature(libc)]

extern crate libc;

use libc::{c_char, c_void};
use std::sync::atomic;

#[link(name="dl")]
extern {
        fn dlsym(handle: *const c_void, symbol: *const c_char) -> *const c_void;
}

const RTLD_NEXT: *const c_void = -1isize as *const c_void;

pub unsafe fn dlsym_next(symbol: &'static str) -> *const u8 {
    let ptr = dlsym(RTLD_NEXT, symbol.as_ptr() as *const c_char);
    if ptr.is_null() {
        panic!("redhook: Unable to find underlying function for {}", symbol);
    }
    ptr as *const u8
}

/* Some Rust library functionality (e.g., jemalloc) initializes
 * lazily, after the hooking library has inserted itself into the call
 * path. If the initialization uses any hooked functions, this will lead
 * to an infinite loop. Work around this by running some initialization
 * code in a static constructor, and bypassing all hooks until it has
 * completed. */

static INIT_STATE: atomic::AtomicBool = atomic::ATOMIC_BOOL_INIT;

pub fn initialized() -> bool {
    INIT_STATE.load(atomic::Ordering::SeqCst)
}

extern fn initialize() {
    Box::new(0u8);
    INIT_STATE.store(true, atomic::Ordering::SeqCst);
}

/* Rust doesn't directly expose __attribute__((constructor)), but this
 * is how GNU implements it. */
#[link_section=".init_array"]
pub static INITIALIZE_CTOR: extern fn() = initialize;

#[macro_export]
macro_rules! hook {
    (fn $real_fn:ident ( $($v:ident : $t:ty),* ) -> $r:ty => $hook_fn:ident $body:block) => {
        #[allow(non_camel_case_types)]
        pub struct $real_fn {__private_field: ()}
        #[allow(non_upper_case_globals)]
        static $real_fn: $real_fn = $real_fn {__private_field: ()};

        impl $real_fn {
            fn get(&self) -> unsafe extern fn ( $($v : $t),* ) -> $r {
                use std::sync::{Once, ONCE_INIT};

                static mut REAL: *const u8 = 0 as *const u8;
                static mut ONCE: Once = ONCE_INIT;

                unsafe {
                    ONCE.call_once(|| {
                        REAL = $crate::dlsym_next(concat!(stringify!($real_fn), "\0"));
                    });
                    std::mem::transmute(REAL)
                }
            }

            #[no_mangle]
            pub unsafe fn $real_fn ( $($v : $t),* ) -> $r {
                if $crate::initialized() {
                    $hook_fn ( $($v),* )
                } else {
                    $real_fn.get() ( $($v),* )
                }
            }
        }

        pub fn $hook_fn ( $($v : $t),* ) -> $r {
            $body
        }
    };

    (fn $real_fn:ident ( $($v:ident : $t:ty),* ) => $hook_fn:ident $body:block) => {
        hook! { fn $real_fn ( $($v : $t),* ) -> () => $hook_fn $body }
    };
}

#[macro_export]
macro_rules! real {
    ($real_fn:ident) => {$real_fn.get()}
}
