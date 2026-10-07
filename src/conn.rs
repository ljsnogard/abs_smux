use abs_buff::{
    TrBuffRead, TrBuffWrite,
    x_deps::{abs_cancel, anylr},
};
use abs_cancel::TrMayCancel;
use anylr::SomeOf;

use crate::{
    chan::{TrChannelHandle, TrChannelRx, TrChannelTx},
    conf::TrMuxConfig,
};

/// Similar to UDP in TCP/IP, a telegrpah can send or receive packets without
/// any handshake to establish a short-living channel. But not like in TCP/IP,
/// a channel and a telegraph sharing a same dock is not allowed.
pub trait TrTelegraph<C>
where
    C: TrMuxConfig,
{
    type Err: core::error::Error;

    fn local_dock(&self) -> C::Dock;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type SendAsync<'f, R>: TrMayCancel<'f, MayCancelOutput =
        SomeOf<usize, Self::Err>>
    where
        Self: 'f,
        R: 'f + TrBuffRead<C::Data>;

    fn send_async<'f, R>(
        &'f mut self,
        remote_dock: C::Dock,
        packet: &'f mut R,
    ) -> Self::SendAsync<'f, R>
    where
        R: TrBuffRead<C::Data>;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type RecvAsync<'f, W>: TrMayCancel<'f, MayCancelOutput =
        SomeOf<usize, Self::Err>>
    where
        Self: 'f,
        W: 'f + TrBuffWrite<C::Data>;

    fn recv_async<'f, W>(
        &'f mut self,
        remote_dock: C::Dock,
        buffer: &'f mut W,
    ) -> Self::RecvAsync<'f, W>
    where
        W: TrBuffWrite<C::Data>;
}


pub trait TrChannelListener<C>
where
    C: TrMuxConfig,
{
    type Err: core::error::Error;

    fn local_dock(&self) -> &C::Dock;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type ChannelHandle: TrChannelHandle<C>;

    type IncomeAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::ChannelHandle, Self::Err>>
    where
        Self: 'f;

    fn income_async(&mut self) -> Self::IncomeAsync<'_>;
}

pub trait TrConnection<C>
where
    C: TrMuxConfig,
{
    type DockBinding: TrDockBinding<C>;
    type Err: core::error::Error;

    type BindAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::DockBinding, Self::Err>>
    where
        Self: 'f;

    fn bind_async<'f>(
        &'f self,
        local_dock: C::Dock,
    ) -> Self::BindAsync<'f>;
}

pub trait TrDockBinding<C>
where
    C: TrMuxConfig,
{
    type Err: core::error::Error;

    fn local_dock(&self) -> &C::Dock;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // Listener section
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type ChannelHandle: TrChannelHandle<C>;

    type Listener: TrChannelListener<C>;

    type ListenAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::Listener, Self::Err>>
    where
        Self: 'f;

    /// Listen at the dock owned by this operator, specifying the max reserve
    /// count of the incoming invitations.
    fn listen_async(
        &mut self,
        reserve: usize,
    ) -> Self::ListenAsync<'_>;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // Telegraph section
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type Telegraph: TrTelegraph<C>;

    type OpenTelegraphAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::Telegraph, Self::Err>>
    where
        Self: 'f;

    /// Create a telegraph for sending and receiving packets.
    fn open_telegraph_async(&mut self) -> Self::OpenTelegraphAsync<'_>;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type Tx: TrChannelTx<C>;
    type Rx: TrChannelRx<C>;

    type OpenChannelAsync<'f, R>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::ChannelHandle, Self::Err>>
    where
        Self: 'f,
        R: 'f + TrBuffRead<C::Data>;

    /// Initiate a channel to the remote dock
    fn open_channel_async<'f, R>(
        &'f mut self,
        remote_dock: C::Dock,
        message: &'f mut R,
    ) -> Self::OpenChannelAsync<'f, R>
    where
        R: TrBuffRead<C::Data>;
}
