#[cfg(feature = "ilp64")]
pub type BlasInt = i64;
#[cfg(feature = "lp64")]
pub type BlasInt = i32;

pub type BlasIndex = usize;
pub type BlasFloat = f32;
pub type BlasDouble = f64;
pub type BlasF16 = u16;
