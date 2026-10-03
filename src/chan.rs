use core::{
    borrow::BorrowMut,
    marker::PhantomData,
    mem::MaybeUninit,
};

use abs_buff::{
    TrBuffRead, TrBuffWrite,
    x_deps::abs_cancel,
};
use abs_cancel::TrMayCancel;

use crate::conf::TrMuxConfig;

/// 远端将要建立 channel 但仍未完成时的半建立 channel。用于传递额外
/// 信息（如协商结果）以及供本端裁决是否最终建立 channel
pub trait TrChannelHandle<C>
where
    C: TrMuxConfig,
{
    type Err: core::error::Error;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // Acceptance
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type Tx: TrChannelTx<C>;
    type Rx: TrChannelRx<C>;

    type AcceptAsync<'f, W, P>: TrMayCancel<'f, MayCancelOutput =
        Result<(Self::Tx, Self::Rx), Self::Err>>
    where
        Self: 'f,
        W: 'f + TrBuffWrite<C::Data>,
        P: TrPrepareChannelRing<C::Buff, C::Data>;

    /// 发送同意建立 channel 的消息及欢迎信息，以及
    fn accept_async<'f, W, P>(
        &'f mut self,
        welcome: &'f mut W,
        prepare: P,
    ) -> Self::AcceptAsync<'f, W, P>
    where
        W: 'f + TrBuffWrite<C::Data>,
        P: TrPrepareChannelRing<C::Buff, C::Data>;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type RejectAsync<'f, R>: TrMayCancel<'f, MayCancelOutput =
        Result<usize, Self::Err>>
    where
        Self: 'f,
        R: 'f + TrBuffRead<C::Data>;

    /// 发送拒绝建立 channel 的消息及理由
    fn reject_async<'f, R>(
        &'f mut self,
        reason: &'f mut R,
    ) -> Self::RejectAsync<'f, R>
    where
        R: TrBuffRead<C::Data>;
}

pub trait TrChannelHalf<C>
where
    C: TrMuxConfig,
{
    fn local_dock(&self) -> C::Dock;

    fn remote_dock(&self) -> C::Dock;

    fn is_tx_closed(&self) -> bool;

    fn is_rx_closed(&self) -> bool;
}

pub trait TrChannelTx<C>
where
    C: TrMuxConfig,
    Self: TrBuffWrite<C::Data> + TrChannelHalf<C>,
{}

pub trait TrChannelRx<C>
where
    C: TrMuxConfig,
    Self: TrBuffRead<C::Data> + TrChannelHalf<C>,
{}

#[derive(Clone, Debug, Default)]
pub struct ChannelBuffAlloc<B, T>
where
    B: 'static + BorrowMut<[MaybeUninit<T>]>,
    T: 'static + Sized,
{
    pub tx_buff: B,
    pub rx_buff: B,
    _use_t_: PhantomData<fn() -> T>,
}

impl<B, T> ChannelBuffAlloc<B, T>
where
    B: 'static + BorrowMut<[MaybeUninit<T>]>,
    T: 'static + Sized,
{
    pub const fn new(tx_buff: B, rx_buff: B) -> Self {
        ChannelBuffAlloc {
            tx_buff,
            rx_buff,
            _use_t_: PhantomData,
        }
    }

    pub fn destruct(self) -> (B, B) {
        (self.tx_buff, self.rx_buff)
    }
}

/// 专用于描述接受建立 channel 时，如何为 channel 配备用于构造环形缓冲器的内存
pub trait TrPrepareChannelRing<B, T>
where
    B: 'static + BorrowMut<[MaybeUninit<T>]>,
    T: 'static + Sized,
{
    fn prepare(self) -> ChannelBuffAlloc<B, T>;
}
