use abs_buff::{
    TrBuffRead, TrBuffWrite, TrBuffTryRead, TrBuffTryWrite,
    x_deps::{abs_cancel, anylr},
};
use abs_cancel::TrMayCancel;
use anylr::SomeOf;

/// Similar to port in TCP/IP, a tuple of dock defines the packet source and destination.
pub trait TrDock
where
    Self: Sized + Clone + Eq + Ord + PartialEq + PartialOrd,
{
    fn unspecified() -> Self;

    fn wildcard() -> Self;

    fn is_unspecified(&self) -> bool {
        *self == Self::unspecified()
    }

    fn is_wildcard(&self) -> bool {
        *self == Self::wildcard()
    }

    fn is_special(&self) -> bool {
        self.is_unspecified() || self.is_wildcard()
    }
}

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

    type RecvAsync<'f, W>: TrMayCancel<'f, MayCancelOutput =
        SomeOf<usize, Self::Err>>
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

pub trait TrChannelHalf {
    type Data;
    type Dock: TrDock;

    fn local_dock(&self) -> Self::Dock;

    fn remote_dock(&self) -> Self::Dock;

    fn is_tx_closed(&self) -> bool;

    fn is_rx_closed(&self) -> bool;
}

pub trait TrChannelTx
where
    Self: TrBuffTryWrite<Self::Data> + TrChannelHalf,
{}

pub trait TrChannelRx
where
    Self: TrBuffTryRead<Self::Data> + TrChannelHalf,
{}

pub trait TrChannelHandle
where
    Self: TrChannelHalf,
{
    type Err: core::error::Error;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type Tx: TrChannelTx<Data = Self::Data, Dock = Self::Dock>;
    type Rx: TrChannelRx<Data = Self::Data, Dock = Self::Dock>;

    type AcceptAsync<'f, W>: TrMayCancel<'f, MayCancelOutput =
        Result<(Self::Tx, Self::Rx), Self::Err>>
    where
        Self: 'f,
        W: 'f + TrBuffWrite;

    /// 向请求端发送同意建立 channel 的消息及欢迎信息
    fn accept_async<'f, W>(
        &'f mut self,
        welcome: &'f mut W,
    ) -> Self::AcceptAsync<'f, W>
    where
        W: TrBuffWrite;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type RejectAsync<'f, R>: TrMayCancel<'f, MayCancelOutput =
        Result<usize, Self::Err>>
    where
        Self: 'f,
        R: 'f + TrBuffRead;

    /// 向请求端发送拒绝建立 channel 的消息及理由
    fn reject_async<'f, R>(
        &'f mut self,
        reason: &'f mut R,
    ) -> Self::RejectAsync<'f, R>
    where
        R: TrBuffRead;
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

    type IncomeAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::ChannelHandle, Self::Err>>
    where
        Self: 'f;

    fn income_async(&mut self) -> Self::IncomeAsync<'_>;
}

pub trait TrConnection {
    type DockBinding<'f>: TrDockBinding<Data = Self::Data, Dock = Self::Dock>
    where
        Self: 'f;

    type Data;
    type Dock: TrDock;
    type Err: core::error::Error;

    type BindAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::DockBinding<'f>, Self::Err>>
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
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type Listener<'f>: TrChannelListener<Data = Self::Data, Dock = Self::Dock>
    where
        Self: 'f;

    type ListenAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::Listener<'f>, Self::Err>>
    where
        Self: 'f;

    /// Listen at the dock owned by this operator.
    fn listen_async(&mut self) -> Self::ListenAsync<'_>;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type Telegraph<'f>: TrTelegraph<Data = Self::Data, Dock = Self::Dock>
    where
        Self: 'f;

    type OpenTelegraphAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::Telegraph<'f>, Self::Err>>
    where
        Self: 'f;

    /// Create a telegraph for sending and receiving packets.
    fn open_telegraph_async(&mut self) -> Self::OpenTelegraphAsync<'_>;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type Tx: TrChannelTx<Data = Self::Data, Dock = Self::Dock>;
    type Rx: TrChannelRx<Data = Self::Data, Dock = Self::Dock>;

    type OpenChannelAsync<'f, R>: TrMayCancel<'f, MayCancelOutput =
        Result<(Self::Tx, Self::Rx), Self::Err>>
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
