use abs_buff::{
    TrBuffRead,
    x_deps::abs_cancel,
};
use abs_cancel::TrMayCancel;

use crate::{
    chan::{TrChannelHandle, TrChannelRx, TrChannelTx},
    conf::TrMuxConfig,
};

pub trait TrChannelListener<C>
where
    C: TrMuxConfig,
{
    type Err: core::error::Error;

    fn local_dock(&self) -> &C::Dock;

    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----
    // Listener income
    // -- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ---- ----

    type ChannelHandle: TrChannelHandle<C>;

    type IncomeAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::ChannelHandle, Self::Err>>
    where
        Self: 'f;

    fn income_async(&mut self) -> Self::IncomeAsync<'_>;
}

pub trait TrConnection {
    type Config: TrMuxConfig;
    type DockBinding: TrDockBinding<Self::Config>;
    type Err: core::error::Error;

    type BindAsync<'f>: TrMayCancel<'f, MayCancelOutput =
        Result<Self::DockBinding, Self::Err>>
    where
        Self: 'f;

    /// 在某个 `local_dock` 上派生一个会话（binding）。
    ///
    /// # `unspecified`：由实现自行安排一个可用垛口
    ///
    /// `local_dock` 取 [`TrDock::unspecified`](crate::dock::TrDock::unspecified)
    /// （数值 `0`）时，它的含义是**由实现自行安排一个空闲 dock**，而不是参数错误：
    /// 实现必须挑一个当前未被自己占用的 dock，并在其上完成同样的独占绑定。
    /// 这与 `bind(2)` 传入端口 `0`、由内核挑一个临时端口同形。调用方从返回的
    /// [`TrDockBinding::local_dock`] 读回**实际**绑定的值。
    ///
    /// 「挑一个空闲的」与「占住它」对**并发调用者必须是原子的**：两个同时发起、
    /// 都取 `unspecified` 的绑定不允许选中同一个 dock。实现因此要么在同一个临界区
    /// 里完成两件事，要么在占用失败时换一个候选重试。
    ///
    /// 候选范围由实现决定（可以留出一段不给自动分配）；所有的候选都被占满时，
    /// 实现按「绑定失败」返回错误即可。`wildcard` 则**不是**这个意思：它是
    /// 「任意 remote」的哨兵，不能充当本端身份，实现必须拒绝。
    ///
    /// # Errors
    ///
    /// `local_dock` 是 `wildcard`、或该 dock 已被占用（含自动分配时候选耗尽）时
    /// 返回错误。
    fn bind_async<'f>(
        &'f self,
        local_dock: <Self::Config as TrMuxConfig>::Dock,
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
    // Channel definition and open_channel_async
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
