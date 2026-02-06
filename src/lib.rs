#![no_std]
#![feature(macro_derive)]

//! bytemuck alternative for casting struct as slice of bytes

/// Declarative derive! macro that adds an `as_u8_slice` method to the struct.
#[macro_export]
macro_rules! AsU8Slice {
    derive() (
        $(#[$_attr:meta])*
        $_vis:vis
        struct
        $name:ident
        $($_the_rest:tt)*
    ) => {
        impl $name {
            #[inline]
            pub fn as_u8_slice(&self) -> &[u8] {
                unsafe {
                    ::core::slice::from_raw_parts(
                        (self as *const $name) as *const u8,
                        ::core::mem::size_of::<$name>(),
                    )
                }
            }
            pub fn as_u8_slice_mut(&mut self) -> &mut [u8] {
                unsafe {
                    ::core::slice::from_raw_parts_mut(
                        (self as *mut $name) as *mut u8,
                        ::core::mem::size_of::<$name>(),
                    )
                }
            }
        }
    };
}
