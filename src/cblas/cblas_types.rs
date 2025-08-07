#[cfg(feature = "ilp64")]
pub type CBlasInt = ::std::os::raw::c_longlong;
#[cfg(feature = "lp64")]
pub type CBlasInt = ::std::os::raw::c_int;

pub type CBlasIndex = ::std::os::raw::c_uint;
pub type CBlasFloat = ::std::os::raw::c_float;
pub type CBlasDouble = ::std::os::raw::c_double;
pub type CBlasVoid = ::std::os::raw::c_void;
pub type CBlasF16 = ::std::os::raw::c_ushort;

#[repr(C)]
pub enum CBlasLayout {
    CBlasRowMajor = 101,
    CBlasColMajor = 102,
}
#[repr(C)]
pub enum CBlasTranspose {
    CBlasNoTrans = 111,
    CBlasTrans = 112,
    CBlasConjTrans = 113,
}

#[repr(C)]
pub enum CBlasUplo {
    CblasUpper = 121,
    CblasLower = 122,
}

#[repr(C)]
pub enum CblasDiag {
    CblasNonUnit = 131,
    CblasUnit = 132,
}

#[repr(C)]
pub enum CblasSide {
    CblasLeft = 141,
    CblasRight = 142,
}
