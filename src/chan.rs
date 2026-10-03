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

use crate::dock::TrDock;

/// 远端将要建立 channel 但仍未完成时的半建立 channel。用于传递额外
/// 信息（如协商结果）以及供本端裁决是否最终建立 channel
pub trait TrChannelHandle
where
    Self: TrChannelHalf,
{
    type Err: core::error::Error;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // Acceptance
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type Tx: TrChannelTx<Data = Self::Data, Dock = Self::Dock>;
    type Rx: TrChannelRx<Data = Self::Data, Dock = Self::Dock>;

    type AcceptAsync<'f, W, P, B>: TrMayCancel<'f, MayCancelOutput =
        Result<(Self::Tx, Self::Rx), Self::Err>>
    where
        Self: 'f + TrAcceptBuff<B, Self::Data>,
        W: 'f + TrBuffWrite<Self::Data>,
        P: 'f + TrPrepareChannelRing<B, Self::Data>,
        B: 'f + BorrowMut<[MaybeUninit<Self::Data>]>;

    /// 发送同意建立 channel 的消息及欢迎信息
    fn accept_async<'f, W, P, B>(
        &'f mut self,
        welcome: &'f mut W,
        prepare: P,
    ) -> Self::AcceptAsync<'f, W, P, B>
    where
        Self: TrAcceptBuff<B, Self::Data>,
        W: TrBuffWrite<Self::Data>,
        P: TrPrepareChannelRing<B, Self::Data>,
        B: BorrowMut<[MaybeUninit<Self::Data>]> + 'f;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type RejectAsync<'f, R>: TrMayCancel<'f, MayCancelOutput =
        Result<usize, Self::Err>>
    where
        Self: 'f,
        R: 'f + TrBuffRead<u8>;

    /// 发送拒绝建立 channel 的消息及理由
    fn reject_async<'f, R>(
        &'f mut self,
        reason: &'f mut R,
    ) -> Self::RejectAsync<'f, R>
    where
        R: TrBuffRead<u8>;
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
    Self: TrBuffWrite<Self::Data> + TrChannelHalf,
{}

pub trait TrChannelRx
where
    Self: TrBuffRead<Self::Data> + TrChannelHalf,
{}

#[derive(Clone, Debug, Default)]
pub struct ChannelBuff<B, T>
where
    B: BorrowMut<[MaybeUninit<T>]>,
{
    pub tx_buff: B,
    pub rx_buff: B,
    _use_b_: PhantomData<fn () -> T>,
}

impl<B, T> ChannelBuff<B, T>
where
    B: BorrowMut<[MaybeUninit<T>]>,
{
    pub const fn new(tx_buff: B, rx_buff: B) -> Self {
        ChannelBuff {
            tx_buff,
            rx_buff,
            _use_b_: PhantomData,
        }
    }
}

/// 专用于描述接受建立 channel 时，如何为 channel 配备用于构造环形缓冲器的内存
pub trait TrPrepareChannelRing<B, T>
where
    B: BorrowMut<[MaybeUninit<T>]>,
{
    /// Tuple 的0号位用于 Tx, 1号位用于 Rx
    fn prepare(self) -> ChannelBuff<B, T>;
}

impl<F, B, T> TrPrepareChannelRing<B, T> for F
where
    F: FnOnce() -> (B, B),
    B: BorrowMut<[MaybeUninit<T>]>,
{
    #[inline]
    fn prepare(self) -> ChannelBuff<B, T> {
        let (tx_buff, rx_buff) = self();
        ChannelBuff::new(tx_buff, rx_buff)
    }
}

/// 连接侧声明：「**这一种**缓冲智能指针，我能用起来」。
///
/// 这是抽象层与实现之间的**分工点**：
///
/// - 抽象层（[`TrChannelHandle::accept_async`]）只要求
///   `Self: TrAcceptBuff<P::Buff, Accepted = Result<(Self::Tx, Self::Rx), Self::Err>>`
///   ——「这次调用给的缓冲，本连接能接受」，并说明接受之后得到什么；
/// - **具体能接受哪几种 `B`，由每个实现自己声明**：零运行时开销的实现只为「正好是自己
///   要的那一种」实现它（于是传错类型在**调用点**编译不过）；而能同时应付多种智能指针
///   的实现可以为多种 `B` 实现它，抽象层不为它设限。
///
/// 这样，「一条连接只能用一种环存储」这个约束落在**实现**身上，而不是被写进抽象层的
/// 类型定义里——写进后者会把所有复用实现一起框死。
pub trait TrAcceptBuff<B, T>
where
    B: BorrowMut<[MaybeUninit<T>]>,
{}
