pub trait TrPort: Copy + Eq + Ord {
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

impl TrPort for u8 {
    fn unspecified() -> Self { 0u8 }

    fn wildcard() -> Self { u8::MAX }
}

impl TrPort for u16 {
    fn unspecified() -> Self { 0u16 }

    fn wildcard() -> Self { u16::MAX }
}

impl TrPort for u32 {
    fn unspecified() -> Self { 0u32 }

    fn wildcard() -> Self { u32::MAX }
}

impl TrPort for u64 {
    fn unspecified() -> Self { 0u64 }

    fn wildcard() -> Self { u64::MAX }
}

impl TrPort for usize {
    fn unspecified() -> Self { 0usize }

    fn wildcard() -> Self { usize::MAX }
}

impl TrPort for i8 {
    fn unspecified() -> Self { 0i8 }

    fn wildcard() -> Self { i8::MIN }
}

impl TrPort for i16 {
    fn unspecified() -> Self { 0i16 }

    fn wildcard() -> Self { i16::MIN }
}

impl TrPort for i32 {
    fn unspecified() -> Self { 0i32 }

    fn wildcard() -> Self { i32::MIN }
}

impl TrPort for i64 {
    fn unspecified() -> Self { 0i64 }

    fn wildcard() -> Self { i64::MIN }
}

impl TrPort for isize {
    fn unspecified() -> Self { 0isize }

    fn wildcard() -> Self { isize::MIN }
}
