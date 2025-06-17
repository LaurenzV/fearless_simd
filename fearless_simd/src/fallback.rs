use crate::{Fallback, WithSimd};

/// The level for architectures without direct SIMD support.
#[derive(Clone, Copy, Debug)]
pub enum Level {
    Fallback(Fallback),
}

impl Level {
    pub fn new() -> Self {
        Self::Fallback(Fallback::new())
    }

    #[inline]
    pub fn fallback() -> Self {
        Self::Fallback(Fallback::new())
    }

    #[inline]
    pub fn dispatch<W: WithSimd>(self, f: W) -> W::Output {
        f.with_simd(Fallback::new())
    }
}
