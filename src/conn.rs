use abs_buff::{
    TrBuffRead, TrBuffWrite,
    x_deps::{abs_cancel, anylr},
};
use abs_cancel::TrMayCancel;
use anylr::SomeOf;

use crate::{
    chan::{TrChannelHandle, TrChannelRx, TrChannelTx},
    dock::TrDock,
};

/// Similar to UDP in TCP/IP, a telegrpah can send or receive packets without
/// any handshake to establish a short-living channel. But not like in TCP/IP,
/// a channel and a telegraph sharing a same dock is not allowed.
pub trait TrTelegraph {
    type Data;
    type Dock: TrDock;
    type Err: core::error::Error;

    fn local_dock(&self) -> Self::Dock;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type SendAsync<'f, R>: TrMayCancel<'f, MayCancelOutput =
        SomeOf<usize, Self::Err>>
    where
        Self: 'f,
        R: 'f + TrBuffRead<Self::Data>;

    fn send_async<'f, R>(
        &'f mut self,
        remote_dock: Self::Dock,
        packet: &'f mut R,
    ) -> Self::SendAsync<'f, R>
    where
        R: TrBuffRead<Self::Data>;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type RecvAsync<'f, W>: TrMayCancel<'f, MayCancelOutput = SomeOf<usize, Self::Err>>
    where
        Self: 'f,
        W: 'f + TrBuffWrite<Self::Data>;

    fn recv_async<'f, W>(
        &'f mut self,
        remote_dock: Self::Dock,
        buffer: &'f mut W,
    ) -> Self::RecvAsync<'f, W>
    where
        W: TrBuffWrite<Self::Data>;
}


pub trait TrChannelListener {
    type Data;
    type Dock: TrDock;
    type Err: core::error::Error;

    fn local_dock(&self) -> &Self::Dock;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type ChannelHandle: TrChannelHandle<
        Data = Self::Data,
        Dock = Self::Dock,
    >;

    type IncomeAsync<'f>: TrMayCancel<'f, MayCancelOutput = Result<Self::ChannelHandle, Self::Err>>
    where
        Self: 'f;

    fn income_async(&mut self) -> Self::IncomeAsync<'_>;
}

pub trait TrConnection {
    type DockBinding: TrDockBinding<Data = Self::Data, Dock = Self::Dock>;

    type Data;
    type Dock: TrDock;
    type Err: core::error::Error;

    type BindAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::DockBinding, Self::Err>>
    where
        Self: 'f;

    fn bind_async<'f>(
        &'f self,
        local_dock: Self::Dock,
    ) -> Self::BindAsync<'f>;
}

pub trait TrDockBinding {
    type Data;
    type Dock: TrDock;
    type Err: core::error::Error;

    fn local_dock(&self) -> &Self::Dock;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // Listener section
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type ChannelHandle: TrChannelHandle;

    type Listener: TrChannelListener<
        Data = Self::Data,
        Dock = Self::Dock,
        ChannelHandle = Self::ChannelHandle,
    >;

    type ListenAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::Listener, Self::Err>>
    where
        Self: 'f;

    /// Listen at the dock owned by this operator.
    fn listen_async(&mut self) -> Self::ListenAsync<'_>;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // Telegraph section
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type Telegraph: TrTelegraph<Data = Self::Data, Dock = Self::Dock>;

    type OpenTelegraphAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::Telegraph, Self::Err>>
    where
        Self: 'f;

    /// Create a telegraph for sending and receiving packets.
    fn open_telegraph_async(&mut self) -> Self::OpenTelegraphAsync<'_>;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type Tx: TrChannelTx<Data = Self::Data, Dock = Self::Dock>;
    type Rx: TrChannelRx<Data = Self::Data, Dock = Self::Dock>;

    type OpenChannelAsync<'f, R>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::ChannelHandle, Self::Err>>
    where
        Self: 'f,
        R: 'f + TrBuffRead<Self::Data>;

    /// Initiate a channel to the remote dock
    fn open_channel_async<'f, R>(
        &'f mut self,
        remote_dock: Self::Dock,
        message: &'f mut R,
    ) -> Self::OpenChannelAsync<'f, R>
    where
        R: TrBuffRead<Self::Data>;
}
