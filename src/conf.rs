use core::{
    borrow::BorrowMut,
    mem::MaybeUninit,
};

use crate::dock::TrDock;

/// The shared types used across the smux connection.
pub trait TrMuxConfig {
    /// The type of data flows in channels through the network. e.g., `u8`;
    type Data: 'static + Sized;

    /// The dock type used to identify channel source and target;
    type Dock: 'static + TrDock;

    /// The buffer type that will be used to construct the ring buffer for
    /// channel IO.
    ///
    /// This is not the buffer type that managed by the mux connection, the
    /// caller is responsible to manage the memory offered to the mux
    /// connection.
    type Buff: 'static + BorrowMut<[MaybeUninit<Self::Data>]>;
}
