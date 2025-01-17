#![no_std]

pub mod port;
pub mod tunnel;

pub mod x_deps {
    pub use abs_buff;
    pub use abs_buff::x_deps::abs_sync;
}
