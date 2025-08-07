use crate::cblas::cblas_types::*;

unsafe extern "C" {

    /// The ?asum routine computes the sum of the magnitudes of elements of a real vector, or the sum of magnitudes of the real and imaginary parts of elements of a complex vector:
    ///
    /// $$\mathrm{res} =  | \mathrm{Re} x_1| + |\mathrm{Im} x_1| + | \mathrm{Re} x_2| + |\mathrm{Im} x_2|+ \cdots + | \mathrm{Re} x_n| + |\mathrm{Im} x_n|$$
    ///
    /// where $x$ is a vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    ///
    /// Contains the sum of magnitudes of real and imaginary parts of all elements of the vector.
    pub fn cblas_sasum(n: CBlasInt, x: *const CBlasFloat, incx: CBlasInt) -> CBlasFloat;

    /// The ?asum routine computes the sum of the magnitudes of elements of a real vector, or the sum of magnitudes of the real and imaginary parts of elements of a complex vector:
    ///
    /// $$\mathrm{res} =  | \mathrm{Re} x_1| + |\mathrm{Im} x_1| + | \mathrm{Re} x_2| + |\mathrm{Im} x_2|+ \cdots + | \mathrm{Re} x_n| + |\mathrm{Im} x_n|$$
    ///
    /// where $x$ is a vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    ///
    /// Contains the sum of magnitudes of real and imaginary parts of all elements of the vector.
    pub fn cblas_scasum(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasFloat;

    /// The ?asum routine computes the sum of the magnitudes of elements of a real vector, or the sum of magnitudes of the real and imaginary parts of elements of a complex vector:
    ///
    /// $$\mathrm{res} =  | \mathrm{Re} x_1| + |\mathrm{Im} x_1| + | \mathrm{Re} x_2| + |\mathrm{Im} x_2|+ \cdots + | \mathrm{Re} x_n| + |\mathrm{Im} x_n|$$
    ///
    /// where $x$ is a vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    ///
    /// Contains the sum of magnitudes of real and imaginary parts of all elements of the vector.
    pub fn cblas_dasum(n: CBlasInt, x: *const CBlasDouble, incx: CBlasInt) -> CBlasDouble;

    /// The ?asum routine computes the sum of the magnitudes of elements of a real vector, or the sum of magnitudes of the real and imaginary parts of elements of a complex vector:
    ///
    /// $$\mathrm{res} =  | \mathrm{Re} x_1| + |\mathrm{Im} x_1| + | \mathrm{Re} x_2| + |\mathrm{Im} x_2|+ \cdots + | \mathrm{Re} x_n| + |\mathrm{Im} x_n|$$
    ///
    /// where $x$ is a vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    ///
    /// Contains the sum of magnitudes of real and imaginary parts of all elements of the vector.
    pub fn cblas_dzasum(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasDouble;

    pub fn cblas_saxpy(
        n: CBlasInt,
        a: CBlasFloat,
        x: *const CBlasFloat,
        incx: CBlasInt,
        y: *mut CBlasFloat,
        incy: CBlasInt,
    );

    pub fn cblas_daxpy(
        n: CBlasInt,
        a: CBlasDouble,
        x: *const CBlasDouble,
        incx: CBlasInt,
        y: *mut CBlasDouble,
        incy: CBlasInt,
    );

    pub fn cblas_caxpy(
        n: CBlasInt,
        a: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    pub fn cblas_zaxpy(
        n: CBlasInt,
        a: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    pub fn cblas_scopy(
        n: CBlasInt,
        x: *const CBlasFloat,
        incx: CBlasInt,
        y: *mut CBlasFloat,
        incy: CBlasInt,
    );

    pub fn cblas_dcopy(
        n: CBlasInt,
        x: *const CBlasDouble,
        incx: CBlasInt,
        y: *mut CBlasDouble,
        incy: CBlasInt,
    );

    pub fn cblas_ccopy(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    pub fn cblas_zcopy(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    pub fn cblas_sdot(
        n: CBlasInt,
        x: *const CBlasFloat,
        incx: CBlasInt,
        y: *const CBlasFloat,
        incy: CBlasInt,
    ) -> CBlasFloat;

    pub fn cblas_ddot(
        n: CBlasInt,
        x: *const CBlasDouble,
        incx: CBlasInt,
        y: *const CBlasDouble,
        incy: CBlasInt,
    ) -> CBlasDouble;

    pub fn cblas_sdsdot(
        n: CBlasInt,
        sb: CBlasFloat,
        sx: *const CBlasFloat,
        incx: CBlasInt,
        sy: *const CBlasFloat,
        incy: CBlasInt,
    ) -> CBlasFloat;

    pub fn cblas_dsdot(
        n: CBlasInt,
        sx: *const CBlasFloat,
        incx: CBlasInt,
        sy: *const CBlasFloat,
        incy: CBlasInt,
    ) -> CBlasDouble;

    pub fn cblas_cdotc_sub(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        dotc: *mut CBlasVoid,
    );

    pub fn cblas_zdotc_sub(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        dotc: *mut CBlasVoid,
    );

    pub fn cblas_cdotu_sub(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        dotu: *mut CBlasVoid,
    );

    pub fn cblas_zdotu_sub(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        dotu: *mut CBlasVoid,
    );

    pub fn cblas_snrm2(n: CBlasInt, x: *const CBlasFloat, incx: CBlasInt) -> CBlasFloat;

    pub fn cblas_dnrm2(n: CBlasInt, x: *const CBlasDouble, incx: CBlasInt) -> CBlasDouble;

    pub fn cblas_scnrm2(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasFloat;

    pub fn cblas_dznrm2(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasDouble;

    pub fn cblas_srot(
        n: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
        y: *mut CBlasFloat,
        incy: CBlasInt,
        c: CBlasFloat,
        s: CBlasFloat,
    );

    pub fn cblas_drot(
        n: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
        y: *mut CBlasDouble,
        incy: CBlasInt,
        c: CBlasDouble,
        s: CBlasDouble,
    );

    pub fn cblas_crot(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
        c: CBlasFloat,
        s: *const CBlasVoid,
    );

    pub fn cblas_zrot(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
        c: CBlasDouble,
        s: *const CBlasVoid,
    );

    pub fn cblas_csrot(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
        c: CBlasFloat,
        s: CBlasFloat,
    );

    pub fn cblas_zdrot(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
        c: CBlasDouble,
        s: CBlasDouble,
    );

    pub fn cblas_srotg(
        a: *mut CBlasFloat,
        b: *mut CBlasFloat,
        c: *mut CBlasFloat,
        s: *mut CBlasFloat,
    );

    pub fn cblas_drotg(
        a: *mut CBlasDouble,
        b: *mut CBlasDouble,
        c: *mut CBlasDouble,
        s: *mut CBlasDouble,
    );

    pub fn cblas_crotg(
        a: *mut CBlasVoid,
        b: *const CBlasVoid,
        c: *mut CBlasFloat,
        s: *mut CBlasVoid,
    );

    pub fn cblas_zrotg(
        a: *mut CBlasVoid,
        b: *const CBlasVoid,
        c: *mut CBlasDouble,
        s: *mut CBlasVoid,
    );

    pub fn cblas_srotm(
        n: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
        y: *mut CBlasFloat,
        incy: CBlasInt,
        param: *const CBlasFloat,
    );

    pub fn cblas_drotm(
        n: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
        y: *mut CBlasDouble,
        incy: CBlasInt,
        param: *const CBlasDouble,
    );

    pub fn cblas_srotmg(
        d1: *mut CBlasFloat,
        d2: *mut CBlasFloat,
        x1: *mut CBlasFloat,
        y1: CBlasFloat,
        param: *mut CBlasFloat,
    );

    pub fn cblas_drotmg(
        d1: *mut CBlasDouble,
        d2: *mut CBlasDouble,
        x1: *mut CBlasDouble,
        y1: CBlasDouble,
        param: *mut CBlasDouble,
    );

    pub fn cblas_sscal(n: CBlasInt, a: CBlasFloat, x: *mut CBlasFloat, incx: CBlasInt);

    pub fn cblas_dscal(n: CBlasInt, a: CBlasDouble, x: *mut CBlasDouble, incx: CBlasInt);

    pub fn cblas_cscal(n: CBlasInt, a: *const CBlasVoid, x: *mut CBlasVoid, incx: CBlasInt);

    pub fn cblas_zscal(n: CBlasInt, a: *const CBlasVoid, x: *mut CBlasVoid, incx: CBlasInt);

    pub fn cblas_csscal(n: CBlasInt, a: CBlasFloat, x: *mut CBlasVoid, incx: CBlasInt);

    pub fn cblas_zdscal(n: CBlasInt, a: CBlasDouble, x: *mut CBlasVoid, incx: CBlasInt);

    pub fn cblas_sswap(
        n: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
        y: *mut CBlasFloat,
        incy: CBlasInt,
    );

    pub fn cblas_dswap(
        n: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
        y: *mut CBlasDouble,
        incy: CBlasInt,
    );

    pub fn cblas_cswap(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    pub fn cblas_zswap(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    pub fn cblas_isamax(n: CBlasInt, x: *const CBlasFloat, incx: CBlasInt) -> CBlasIndex;

    pub fn cblas_idamax(n: CBlasInt, x: *const CBlasDouble, incx: CBlasInt) -> CBlasIndex;

    pub fn cblas_icamax(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasIndex;

    pub fn cblas_izamax(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasIndex;

    pub fn cblas_isamin(n: CBlasInt, x: *const CBlasFloat, incx: CBlasInt) -> CBlasIndex;

    pub fn cblas_idamin(n: CBlasInt, x: *const CBlasDouble, incx: CBlasInt) -> CBlasIndex;

    pub fn cblas_icamin(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasIndex;

    pub fn cblas_izamin(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasIndex;

    pub fn cblas_scabs1(z: *const CBlasVoid) -> CBlasFloat;

    pub fn cblas_dcabs1(z: *const CBlasVoid) -> CBlasDouble;
}
