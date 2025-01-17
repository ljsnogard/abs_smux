#![no_std]

#![feature(try_trait_v2)]

pub mod tunnel;

pub mod x_deps {
    pub use abs_buff;
    pub use abs_buff::x_deps::abs_sync;
}
