#![feature(allocator_ext)]

#![no_std]

pub mod chan;
pub mod conf;
pub mod conn;
pub mod dock;
pub mod telegraph;

pub mod x_deps {
    pub use abs_mm;
}
