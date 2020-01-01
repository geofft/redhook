#[macro_export]
macro_rules! hook {
    (unsafe fn $real_fn:ident ( $($v:ident : $t:ty),* ) -> $r:ty => $hook_fn:ident $body:block) => {
        pub mod $real_fn {
            #[allow(non_camel_case_types)]
            pub struct $real_fn {
                _new: *const (),
                _old: *const (),
            }

            #[allow(dead_code)]
            #[allow(non_upper_case_globals)]
            #[link_section="__DATA,__interpose"]
            pub static mut $real_fn: $real_fn = $real_fn {
                _new: super::$hook_fn as *const (),
                _old: super::$real_fn as *const (),
            };
        }

        extern {
            pub fn $real_fn ( $($v : $t),* ) -> $r;
        }

        pub unsafe extern fn $hook_fn ( $($v : $t),* ) -> $r {
            ::std::panic::catch_unwind(|| $body ).unwrap_or_else(|_| $real_fn ( $($v),* ))
        }
    };

    (unsafe fn $real_fn:ident ( $($v:ident : $t:ty),* ) => $hook_fn:ident $body:block) => {
        $crate::hook! { unsafe fn $real_fn ( $($v : $t),* ) -> () => $hook_fn $body }
    };
}

#[macro_export]
macro_rules! real {
    ($real_fn:ident) => {
        $real_fn
    };
}
