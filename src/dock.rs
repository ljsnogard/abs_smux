use abs_buff::x_deps::funty;

use crate::conn::TrDock;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Dock<T>(T)
where
    T: Clone + Copy + Eq + Ord + PartialEq + PartialOrd;

impl<T> Dock<T>
where
    T: Clone + Copy + Eq + Ord + PartialEq + PartialOrd,
{
    pub const fn new(t: T) -> Self {
        Dock(t)
    }

    /// 取回 dock 的内部数值。
    ///
    /// 上层协议需要把 dock 编成字节（例如复用帧的 `LocalDock` / `RemoteDock`
    /// 字段）时，必须能把它还原成数值；反向构造见 [`Dock::new`]。
    ///
    /// # Examples
    ///
    /// ```
    /// use abs_smux::dock::Dock;
    ///
    /// assert_eq!(Dock::new(7u32).value(), 7u32);
    /// assert_eq!(Dock::<u32>::wildcard().value(), u32::MAX);
    /// ```
    pub const fn value(&self) -> T {
        self.0
    }
}

impl<T> Dock<T>
where
    T: funty::Integral + Clone + Copy
        + Eq + Ord + PartialEq + PartialOrd,
{
    pub const fn unspecified() -> Self {
        Dock(T::ZERO)
    }

    pub const fn wildcard() -> Self {
        Dock(T::MAX)
    }
}

impl TrDock for Dock<u8> {
    fn unspecified() -> Self {
        Dock::unspecified()
    }

    fn wildcard() -> Self {
        Dock::wildcard()
    }
}

impl TrDock for Dock<u16> {
    fn unspecified() -> Self {
        Dock::unspecified()
    }

    fn wildcard() -> Self {
        Dock::wildcard()
    }
}

impl TrDock for Dock<u32> {
    fn unspecified() -> Self {
        Dock::unspecified()
    }

    fn wildcard() -> Self {
        Dock::wildcard()
    }
}

impl TrDock for Dock<u64> {
    fn unspecified() -> Self {
        Dock::unspecified()
    }

    fn wildcard() -> Self {
        Dock::wildcard()
    }
}

impl TrDock for Dock<usize> {
    fn unspecified() -> Self {
        Dock::unspecified()
    }

    fn wildcard() -> Self {
        Dock::wildcard()
    }
}

impl TrDock for Dock<i8> {
    fn unspecified() -> Self {
        Dock::unspecified()
    }

    fn wildcard() -> Self {
        Dock::wildcard()
    }
}

impl TrDock for Dock<i16> {
    fn unspecified() -> Self {
        Dock::unspecified()
    }

    fn wildcard() -> Self {
        Dock::wildcard()
    }
}

impl TrDock for Dock<i32> {
    fn unspecified() -> Self {
        Dock::unspecified()
    }

    fn wildcard() -> Self {
        Dock::wildcard()
    }
}

impl TrDock for Dock<i64> {
    fn unspecified() -> Self {
        Dock::unspecified()
    }

    fn wildcard() -> Self {
        Dock::wildcard()
    }
}

impl TrDock for Dock<isize> {
    fn unspecified() -> Self {
        Dock::unspecified()
    }

    fn wildcard() -> Self {
        Dock::wildcard()
    }
}
