use core::{
    alloc::AllocatorClone,
    marker::PhantomData,
    mem::MaybeUninit,
};

use abs_buff::{
    TrBuffRead, TrBuffWrite,
    x_deps::abs_cancel,
};
use abs_cancel::TrMayCancel;
use abs_mm::res_man::TrUnique;

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

    /// 最终裁决：把**调用方当场交出的**那块 ring 内存接成两条环。
    ///
    /// ring 存储的智能指针类型 `B` 是**方法级泛型**，不再由
    /// [`TrMuxConfig`](crate::conf::TrMuxConfig) 规定——本条子流用什么智能指针、
    /// 从哪来、多大，完全由 `prepare` 的实参决定。
    type AcceptAsync<'f, W, B, P>: TrMayCancel<'f, MayCancelOutput =
        Result<(Self::Tx, Self::Rx), Self::Err>>
    where
        Self: 'f,
        W: 'f + TrBuffWrite<C::Data>,
        B: 'static + Send + Sync + TrUnique<Item = [MaybeUninit<C::Data>], Alloc: AllocatorClone>,
        P: TrPrepareChannelRing<B, C::Data>;

    /// 发送同意建立 channel 的消息及欢迎信息，以及
    fn accept_async<'f, W, B, P>(
        &'f mut self,
        welcome: &'f mut W,
        prepare: P,
    ) -> Self::AcceptAsync<'f, W, B, P>
    where
        W: 'f + TrBuffWrite<C::Data>,
        B: 'static + Send + Sync + TrUnique<Item = [MaybeUninit<C::Data>], Alloc: AllocatorClone>,
        P: TrPrepareChannelRing<B, C::Data>;

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

/// 一条子流两个方向的 ring 内存（各自的智能指针 `B`）。
///
/// `B::Alloc: AllocatorClone` 是连接侧释放这两块内存所需——连接会把 `B` 连同它的
/// ring 一起搬进自己的记账结构，之后**只能**靠 `B` 自己报出的分配器来归还。
#[derive(Clone, Debug, Default)]
pub struct ChannelBuffAlloc<B, T>
where
    B: 'static + TrUnique<Item = [MaybeUninit<T>]>,
    B::Alloc: AllocatorClone,
    T: 'static + Sized,
{
    pub tx_buff: B,
    pub rx_buff: B,
    _use_t_: PhantomData<fn() -> T>,
}

impl<B, T> ChannelBuffAlloc<B, T>
where
    B: 'static + TrUnique<Item = [MaybeUninit<T>]>,
    B::Alloc: AllocatorClone,
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
    B: 'static + TrUnique<Item = [MaybeUninit<T>]>,
    B::Alloc: AllocatorClone,
    T: 'static + Sized,
{
    fn prepare(self) -> ChannelBuffAlloc<B, T>;
}
