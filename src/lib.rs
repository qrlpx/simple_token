#![no_std]

#[doc(hidden)]
pub mod _core {
    pub use core::*;
}

#[macro_export]
macro_rules! new_token_type {
    (@impl $name:ident) => {
        impl $name {
            pub fn new() -> Self {
                use $crate::_core::sync::atomic::{AtomicUsize, Ordering};

                //FIXME(integer_atomics) use AtomicU64
                static mut COUNT: Option<AtomicUsize> = None;

                //FIXME(const_fn) racey, remove when const_fn is stable
                match unsafe { &mut COUNT } {
                    &mut Some(ref mut count) => {
                        let val = count.fetch_add(1, Ordering::Relaxed);
                        assert!(val != !0, 
                            "Token type '{}' experienced overflow: too many tokens generated", stringify!($name)
                        ); 
                        $name(val)
                    }
                    count => {
                        unsafe { *(count as *mut _) = Some(AtomicUsize::new(0)); }
                        Self::new()
                    }
                }
            }
        }
    };
    (pub $name:ident) => {
        #[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(usize);
        
        new_token_type!(@impl $name);
    };
    ($name:ident) => {
        #[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        struct $name(usize);
        
        new_token_type!(@impl $name);
    };
}

new_token_type!(pub Token);

