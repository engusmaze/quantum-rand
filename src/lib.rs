use std::{mem, time::Instant};

pub struct QRNG {
    last_ns: u32,
    sum: u32,
}
impl QRNG {
    pub const fn new() -> Self {
        Self { last_ns: 0, sum: 0 }
    }
    fn bit(&mut self) -> u32 {
        // Loop til we get a new time value
        loop {
            let ns = unsafe { *(&Instant::now() as *const _ as *const u32).wrapping_add(2) };
            self.sum = self.sum.wrapping_add(ns);
            if ns != self.last_ns {
                self.last_ns = ns;
                return (self.sum * 1210758371) >> 31; // Apply tiny bit of mixing and take last bit
            }
        }
    }
    pub fn bool(&mut self) -> bool {
        self.bit() != 0
    }
}
macro_rules! impl_integers {
    ($($i: ident $u: ident)*) => {
        impl QRNG {
            $(
                pub fn $u(&mut self) -> $u {
                    let mut value = 0;
                    for i in 0..mem::size_of::<$u>() * 8 {
                        value |= (self.bit() as $u) << i;
                    }
                    value
                }
                #[inline(always)]
                pub fn $i(&mut self) -> $i {
                    self.$u() as $i
                }
            )*
        }
    };
}
impl_integers! {
    i8 u8
    i16 u16
    i32 u32
    i64 u64
    i128 u128
}
