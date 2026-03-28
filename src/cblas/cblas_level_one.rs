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

    /// The ?axpy routine performs a vector-vector operation defined as
    ///
    /// $$y := a \cdot x + y$$
    ///
    /// where $a$ is a scalar, $x$ and $y$ are vectors each with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `a` - Specifies the scalar a.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_saxpy(
        n: CBlasInt,
        a: CBlasFloat,
        x: *const CBlasFloat,
        incx: CBlasInt,
        y: *mut CBlasFloat,
        incy: CBlasInt,
    );

    /// The ?axpy routine performs a vector-vector operation defined as
    ///
    /// $$y := a \cdot x + y$$
    ///
    /// where $a$ is a scalar, $x$ and $y$ are vectors each with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `a` - Specifies the scalar a.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_daxpy(
        n: CBlasInt,
        a: CBlasDouble,
        x: *const CBlasDouble,
        incx: CBlasInt,
        y: *mut CBlasDouble,
        incy: CBlasInt,
    );

    /// The ?axpy routine performs a vector-vector operation defined as
    ///
    /// $$y := a \cdot x + y$$
    ///
    /// where $a$ is a scalar, $x$ and $y$ are vectors each with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `a` - Specifies the scalar a (complex).
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_caxpy(
        n: CBlasInt,
        a: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?axpy routine performs a vector-vector operation defined as
    ///
    /// $$y := a \cdot x + y$$
    ///
    /// where $a$ is a scalar, $x$ and $y$ are vectors each with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `a` - Specifies the scalar a (complex double).
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_zaxpy(
        n: CBlasInt,
        a: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?copy routine performs a vector-vector operation defined as
    ///
    /// $$y := x$$
    ///
    /// where $x$ and $y$ are vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_scopy(
        n: CBlasInt,
        x: *const CBlasFloat,
        incx: CBlasInt,
        y: *mut CBlasFloat,
        incy: CBlasInt,
    );

    /// The ?copy routine performs a vector-vector operation defined as
    ///
    /// $$y := x$$
    ///
    /// where $x$ and $y$ are vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_dcopy(
        n: CBlasInt,
        x: *const CBlasDouble,
        incx: CBlasInt,
        y: *mut CBlasDouble,
        incy: CBlasInt,
    );

    /// The ?copy routine performs a vector-vector operation defined as
    ///
    /// $$y := x$$
    ///
    /// where $x$ and $y$ are vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_ccopy(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?copy routine performs a vector-vector operation defined as
    ///
    /// $$y := x$$
    ///
    /// where $x$ and $y$ are vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_zcopy(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?dot routine performs a vector-vector reduction operation defined as
    ///
    /// $$\mathrm{res} = \sum_{i=1}^{n} x_i \cdot y_i$$
    ///
    /// where $x$ and $y$ are vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    ///
    /// # Returns
    /// Returns the dot product of vectors x and y.
    pub fn cblas_sdot(
        n: CBlasInt,
        x: *const CBlasFloat,
        incx: CBlasInt,
        y: *const CBlasFloat,
        incy: CBlasInt,
    ) -> CBlasFloat;

    /// The ?dot routine performs a vector-vector reduction operation defined as
    ///
    /// $$\mathrm{res} = \sum_{i=1}^{n} x_i \cdot y_i$$
    ///
    /// where $x$ and $y$ are vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    ///
    /// # Returns
    /// Returns the dot product of vectors x and y.
    pub fn cblas_ddot(
        n: CBlasInt,
        x: *const CBlasDouble,
        incx: CBlasInt,
        y: *const CBlasDouble,
        incy: CBlasInt,
    ) -> CBlasDouble;

    /// The sdsdot routine performs a vector-vector operation defined as
    ///
    /// $$\mathrm{res} = sb + \sum_{i=1}^{n} sx_i \cdot sy_i$$
    ///
    /// where $sb$ is a scalar, $sx$ and $sy$ are single-precision vectors with $n$ elements.
    /// The computation is performed in double precision.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors sx and sy.
    /// * `sb` - Specifies the scalar sb to be added to the dot product.
    /// * `sx` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector sx.
    /// * `sy` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector sy.
    ///
    /// # Returns
    /// Returns the result of sb plus the dot product of sx and sy.
    pub fn cblas_sdsdot(
        n: CBlasInt,
        sb: CBlasFloat,
        sx: *const CBlasFloat,
        incx: CBlasInt,
        sy: *const CBlasFloat,
        incy: CBlasInt,
    ) -> CBlasFloat;

    /// The dsdot routine performs a vector-vector operation defined as
    ///
    /// $$\mathrm{res} = \sum_{i=1}^{n} sx_i \cdot sy_i$$
    ///
    /// where $sx$ and $sy$ are single-precision vectors with $n$ elements.
    /// The computation is performed in double precision.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors sx and sy.
    /// * `sx` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector sx.
    /// * `sy` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector sy.
    ///
    /// # Returns
    /// Returns the dot product of vectors sx and sy in double precision.
    pub fn cblas_dsdot(
        n: CBlasInt,
        sx: *const CBlasFloat,
        incx: CBlasInt,
        sy: *const CBlasFloat,
        incy: CBlasInt,
    ) -> CBlasDouble;

    /// The ?dotc routine performs a vector-vector operation defined as
    ///
    /// $$\mathrm{res} = \sum_{i=1}^{n} \overline{x_i} \cdot y_i$$
    ///
    /// where $x$ and $y$ are complex vectors of $n$ elements. The conjugate of $x$ is used.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `dotc` - Contains the result of the conjugate dot product.
    pub fn cblas_cdotc_sub(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        dotc: *mut CBlasVoid,
    );

    /// The ?dotc routine performs a vector-vector operation defined as
    ///
    /// $$\mathrm{res} = \sum_{i=1}^{n} \overline{x_i} \cdot y_i$$
    ///
    /// where $x$ and $y$ are complex vectors of $n$ elements. The conjugate of $x$ is used.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `dotc` - Contains the result of the conjugate dot product.
    pub fn cblas_zdotc_sub(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        dotc: *mut CBlasVoid,
    );

    /// The ?dotu routine performs a vector-vector operation defined as
    ///
    /// $$\mathrm{res} = \sum_{i=1}^{n} x_i \cdot y_i$$
    ///
    /// where $x$ and $y$ are complex vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `dotu` - Contains the result of the dot product.
    pub fn cblas_cdotu_sub(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        dotu: *mut CBlasVoid,
    );

    /// The ?dotu routine performs a vector-vector operation defined as
    ///
    /// $$\mathrm{res} = \sum_{i=1}^{n} x_i \cdot y_i$$
    ///
    /// where $x$ and $y$ are complex vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)).
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `dotu` - Contains the result of the dot product.
    pub fn cblas_zdotu_sub(
        n: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        dotu: *mut CBlasVoid,
    );

    /// The ?nrm2 routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{res} = \|x\|_2 = \sqrt{\sum_{i=1}^{n} |x_i|^2}$$
    ///
    /// where $x$ is a vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the Euclidean norm of vector x.
    pub fn cblas_snrm2(n: CBlasInt, x: *const CBlasFloat, incx: CBlasInt) -> CBlasFloat;

    /// The ?nrm2 routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{res} = \|x\|_2 = \sqrt{\sum_{i=1}^{n} |x_i|^2}$$
    ///
    /// where $x$ is a vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the Euclidean norm of vector x.
    pub fn cblas_dnrm2(n: CBlasInt, x: *const CBlasDouble, incx: CBlasInt) -> CBlasDouble;

    /// The ?nrm2 routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{res} = \|x\|_2 = \sqrt{\sum_{i=1}^{n} |x_i|^2}$$
    ///
    /// where $x$ is a complex vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the Euclidean norm of complex vector x.
    pub fn cblas_scnrm2(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasFloat;

    /// The ?nrm2 routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{res} = \|x\|_2 = \sqrt{\sum_{i=1}^{n} |x_i|^2}$$
    ///
    /// where $x$ is a complex double vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the Euclidean norm of complex vector x.
    pub fn cblas_dznrm2(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasDouble;

    /// The ?rot routine performs a vector-vector operation defined as
    ///
    /// $$\begin{pmatrix} x_i \\ y_i \end{pmatrix} := \begin{pmatrix} c & s \\ -s & c \end{pmatrix} \begin{pmatrix} x_i \\ y_i \end{pmatrix}$$
    ///
    /// where $c$ and $s$ form a plane rotation matrix. For complex versions, the operation uses appropriate complex arithmetic.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). On entry, contains the vector x. On exit, overwritten by the rotated vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). On entry, contains the vector y. On exit, overwritten by the rotated vector.
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `c` - Specifies the scalar c (cosine of rotation).
    /// * `s` - Specifies the scalar s (sine of rotation).
    pub fn cblas_srot(
        n: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
        y: *mut CBlasFloat,
        incy: CBlasInt,
        c: CBlasFloat,
        s: CBlasFloat,
    );

    /// The ?rot routine performs a vector-vector operation defined as
    ///
    /// $$\begin{pmatrix} x_i \\ y_i \end{pmatrix} := \begin{pmatrix} c & s \\ -s & c \end{pmatrix} \begin{pmatrix} x_i \\ y_i \end{pmatrix}$$
    ///
    /// where $c$ and $s$ form a plane rotation matrix. For complex versions, the operation uses appropriate complex arithmetic.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). On entry, contains the vector x. On exit, overwritten by the rotated vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). On entry, contains the vector y. On exit, overwritten by the rotated vector.
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `c` - Specifies the scalar c (cosine of rotation).
    /// * `s` - Specifies the scalar s (sine of rotation).
    pub fn cblas_drot(
        n: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
        y: *mut CBlasDouble,
        incy: CBlasInt,
        c: CBlasDouble,
        s: CBlasDouble,
    );

    /// The ?rot routine performs a vector-vector operation for complex vectors defined as
    ///
    /// $$\begin{pmatrix} x_i \\ y_i \end{pmatrix} := \begin{pmatrix} c & s \\ -\overline{s} & c \end{pmatrix} \begin{pmatrix} x_i \\ y_i \end{pmatrix}$$
    ///
    /// where $c$ is real and $s$ is complex, forming a Givens rotation matrix.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). On entry, contains the complex vector x. On exit, overwritten by the rotated vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). On entry, contains the complex vector y. On exit, overwritten by the rotated vector.
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `c` - Specifies the real scalar c (cosine of rotation).
    /// * `s` - Specifies the complex scalar s (sine of rotation).
    pub fn cblas_crot(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
        c: CBlasFloat,
        s: *const CBlasVoid,
    );

    /// The ?rot routine performs a vector-vector operation for complex vectors defined as
    ///
    /// $$\begin{pmatrix} x_i \\ y_i \end{pmatrix} := \begin{pmatrix} c & s \\ -\overline{s} & c \end{pmatrix} \begin{pmatrix} x_i \\ y_i \end{pmatrix}$$
    ///
    /// where $c$ is real and $s$ is complex, forming a Givens rotation matrix.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). On entry, contains the complex vector x. On exit, overwritten by the rotated vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). On entry, contains the complex vector y. On exit, overwritten by the rotated vector.
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `c` - Specifies the real scalar c (cosine of rotation).
    /// * `s` - Specifies the complex scalar s (sine of rotation).
    pub fn cblas_zrot(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
        c: CBlasDouble,
        s: *const CBlasVoid,
    );

    /// The csrot routine performs a vector-vector operation defined as
    ///
    /// $$\begin{pmatrix} x_i \\ y_i \end{pmatrix} := \begin{pmatrix} c & s \\ -s & c \end{pmatrix} \begin{pmatrix} x_i \\ y_i \end{pmatrix}$$
    ///
    /// where $c$ and $s$ are real scalars, and $x$ and $y$ are complex single-precision vectors.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). Complex single-precision vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). Complex single-precision vector.
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `c` - Specifies the real scalar c (cosine of rotation).
    /// * `s` - Specifies the real scalar s (sine of rotation).
    pub fn cblas_csrot(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
        c: CBlasFloat,
        s: CBlasFloat,
    );

    /// The zdrot routine performs a vector-vector operation defined as
    ///
    /// $$\begin{pmatrix} x_i \\ y_i \end{pmatrix} := \begin{pmatrix} c & s \\ -s & c \end{pmatrix} \begin{pmatrix} x_i \\ y_i \end{pmatrix}$$
    ///
    /// where $c$ and $s$ are real scalars, and $x$ and $y$ are complex double-precision vectors.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). Complex double-precision vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). Complex double-precision vector.
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `c` - Specifies the real scalar c (cosine of rotation).
    /// * `s` - Specifies the real scalar s (sine of rotation).
    pub fn cblas_zdrot(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
        c: CBlasDouble,
        s: CBlasDouble,
    );

    /// The ?rotg routine performs a Givens rotation of a vector with two elements.
    /// Given the Cartesian coordinates $(a, b)$ of a point, these routines compute the parameters $c$ and $s$ for a Givens rotation matrix $G$:
    ///
    /// $$G = \begin{pmatrix} c & s \\ -s & c \end{pmatrix}$$
    ///
    /// such that
    ///
    /// $$G \begin{pmatrix} a \\ b \end{pmatrix} = \begin{pmatrix} r \\ 0 \end{pmatrix}$$
    ///
    /// where $r = \sqrt{|a|^2 + |b|^2}$.
    ///
    /// # Arguments
    /// * `a` - On entry, specifies the first element of the vector. On exit, overwritten by the rotated first element (r).
    /// * `b` - On entry, specifies the second element of the vector. On exit, overwritten by the rotation parameter z (used to recover s).
    /// * `c` - On exit, contains the cosine of the rotation.
    /// * `s` - On exit, contains the sine of the rotation.
    pub fn cblas_srotg(
        a: *mut CBlasFloat,
        b: *mut CBlasFloat,
        c: *mut CBlasFloat,
        s: *mut CBlasFloat,
    );

    /// The ?rotg routine performs a Givens rotation of a vector with two elements.
    /// Given the Cartesian coordinates $(a, b)$ of a point, these routines compute the parameters $c$ and $s$ for a Givens rotation matrix $G$:
    ///
    /// $$G = \begin{pmatrix} c & s \\ -s & c \end{pmatrix}$$
    ///
    /// such that
    ///
    /// $$G \begin{pmatrix} a \\ b \end{pmatrix} = \begin{pmatrix} r \\ 0 \end{pmatrix}$$
    ///
    /// where $r = \sqrt{|a|^2 + |b|^2}$.
    ///
    /// # Arguments
    /// * `a` - On entry, specifies the first element of the vector. On exit, overwritten by the rotated first element (r).
    /// * `b` - On entry, specifies the second element of the vector. On exit, overwritten by the rotation parameter z (used to recover s).
    /// * `c` - On exit, contains the cosine of the rotation.
    /// * `s` - On exit, contains the sine of the rotation.
    pub fn cblas_drotg(
        a: *mut CBlasDouble,
        b: *mut CBlasDouble,
        c: *mut CBlasDouble,
        s: *mut CBlasDouble,
    );

    /// The ?rotg routine performs a Givens rotation for complex vectors.
    /// Given the Cartesian coordinates $(a, b)$ of a point, these routines compute the parameters $c$ and $s$ for a Givens rotation.
    ///
    /// # Arguments
    /// * `a` - On entry, specifies the first complex element of the vector. On exit, overwritten by the rotated first element.
    /// * `b` - On entry, specifies the second complex element of the vector. On exit, overwritten by the rotation parameter.
    /// * `c` - On exit, contains the real cosine of the rotation.
    /// * `s` - On exit, contains the complex sine of the rotation.
    pub fn cblas_crotg(
        a: *mut CBlasVoid,
        b: *const CBlasVoid,
        c: *mut CBlasFloat,
        s: *mut CBlasVoid,
    );

    /// The ?rotg routine performs a Givens rotation for complex vectors.
    /// Given the Cartesian coordinates $(a, b)$ of a point, these routines compute the parameters $c$ and $s$ for a Givens rotation.
    ///
    /// # Arguments
    /// * `a` - On entry, specifies the first complex element of the vector. On exit, overwritten by the rotated first element.
    /// * `b` - On entry, specifies the second complex element of the vector. On exit, overwritten by the rotation parameter.
    /// * `c` - On exit, contains the real cosine of the rotation.
    /// * `s` - On exit, contains the complex sine of the rotation.
    pub fn cblas_zrotg(
        a: *mut CBlasVoid,
        b: *const CBlasVoid,
        c: *mut CBlasDouble,
        s: *mut CBlasVoid,
    );

    /// The ?rotm routine performs a modified Givens rotation of a pair of vectors.
    /// The operation is defined as:
    ///
    /// $$\begin{pmatrix} x_i \\ y_i \end{pmatrix} := H \begin{pmatrix} x_i \\ y_i \end{pmatrix}$$
    ///
    /// where $H$ is a modified Givens transformation matrix defined by the param array.
    /// The param array contains $h_{11}, h_{21}, h_{12}, h_{22}$ and a flag defining the form of $H$.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). On exit, overwritten by the rotated vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). On exit, overwritten by the rotated vector.
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `param` - Array, size 5. Contains the modified Givens rotation parameters: flag, $h_{11}$, $h_{21}$, $h_{12}$, $h_{22}$.
    pub fn cblas_srotm(
        n: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
        y: *mut CBlasFloat,
        incy: CBlasInt,
        param: *const CBlasFloat,
    );

    /// The ?rotm routine performs a modified Givens rotation of a pair of vectors.
    /// The operation is defined as:
    ///
    /// $$\begin{pmatrix} x_i \\ y_i \end{pmatrix} := H \begin{pmatrix} x_i \\ y_i \end{pmatrix}$$
    ///
    /// where $H$ is a modified Givens transformation matrix defined by the param array.
    /// The param array contains $h_{11}, h_{21}, h_{12}, h_{22}$ and a flag defining the form of $H$.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). On exit, overwritten by the rotated vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). On exit, overwritten by the rotated vector.
    /// * `incy` - Specifies the increment for indexing vector y.
    /// * `param` - Array, size 5. Contains the modified Givens rotation parameters: flag, $h_{11}$, $h_{21}$, $h_{12}$, $h_{22}$.
    pub fn cblas_drotm(
        n: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
        y: *mut CBlasDouble,
        incy: CBlasInt,
        param: *const CBlasDouble,
    );

    /// The ?rotmg routine computes the parameters for a modified Givens rotation.
    /// Given input matrices $D_1$, $D_2$ and vectors, this routine computes the modified Givens transformation matrix $H$ that zeroes the $y$-component of the vector:
    ///
    /// $$\begin{pmatrix} x_1 \\ y_1 \end{pmatrix} := H \begin{pmatrix} \sqrt{d_1} x_1 \\ \sqrt{d_2} y_1 \end{pmatrix}$$
    ///
    /// # Arguments
    /// * `d1` - On entry, specifies the first diagonal element. On exit, contains the updated value.
    /// * `d2` - On entry, specifies the second diagonal element. On exit, contains the updated value.
    /// * `x1` - On entry, specifies the first vector component. On exit, contains the updated value.
    /// * `y1` - On entry, specifies the second vector component (scalar input).
    /// * `param` - Array, size 5. On exit, contains the modified Givens rotation parameters: flag, $h_{11}$, $h_{21}$, $h_{12}$, $h_{22}$.
    pub fn cblas_srotmg(
        d1: *mut CBlasFloat,
        d2: *mut CBlasFloat,
        x1: *mut CBlasFloat,
        y1: CBlasFloat,
        param: *mut CBlasFloat,
    );

    /// The ?rotmg routine computes the parameters for a modified Givens rotation.
    /// Given input matrices $D_1$, $D_2$ and vectors, this routine computes the modified Givens transformation matrix $H$ that zeroes the $y$-component of the vector:
    ///
    /// $$\begin{pmatrix} x_1 \\ y_1 \end{pmatrix} := H \begin{pmatrix} \sqrt{d_1} x_1 \\ \sqrt{d_2} y_1 \end{pmatrix}$$
    ///
    /// # Arguments
    /// * `d1` - On entry, specifies the first diagonal element. On exit, contains the updated value.
    /// * `d2` - On entry, specifies the second diagonal element. On exit, contains the updated value.
    /// * `x1` - On entry, specifies the first vector component. On exit, contains the updated value.
    /// * `y1` - On entry, specifies the second vector component (scalar input).
    /// * `param` - Array, size 5. On exit, contains the modified Givens rotation parameters: flag, $h_{11}$, $h_{21}$, $h_{12}$, $h_{22}$.
    pub fn cblas_drotmg(
        d1: *mut CBlasDouble,
        d2: *mut CBlasDouble,
        x1: *mut CBlasDouble,
        y1: CBlasDouble,
        param: *mut CBlasDouble,
    );

    /// The ?scal routine performs a vector operation defined as
    ///
    /// $$x := a \cdot x$$
    ///
    /// where $a$ is a scalar and $x$ is a vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `a` - Specifies the scalar a.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). On exit, overwritten by the scaled vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    pub fn cblas_sscal(n: CBlasInt, a: CBlasFloat, x: *mut CBlasFloat, incx: CBlasInt);

    /// The ?scal routine performs a vector operation defined as
    ///
    /// $$x := a \cdot x$$
    ///
    /// where $a$ is a scalar and $x$ is a vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `a` - Specifies the scalar a.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). On exit, overwritten by the scaled vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    pub fn cblas_dscal(n: CBlasInt, a: CBlasDouble, x: *mut CBlasDouble, incx: CBlasInt);

    /// The ?scal routine performs a vector operation defined as
    ///
    /// $$x := a \cdot x$$
    ///
    /// where $a$ is a complex scalar and $x$ is a complex vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `a` - Specifies the complex scalar a.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). Complex vector. On exit, overwritten by the scaled vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    pub fn cblas_cscal(n: CBlasInt, a: *const CBlasVoid, x: *mut CBlasVoid, incx: CBlasInt);

    /// The ?scal routine performs a vector operation defined as
    ///
    /// $$x := a \cdot x$$
    ///
    /// where $a$ is a complex scalar and $x$ is a complex double-precision vector with $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `a` - Specifies the complex scalar a.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). Complex double-precision vector. On exit, overwritten by the scaled vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    pub fn cblas_zscal(n: CBlasInt, a: *const CBlasVoid, x: *mut CBlasVoid, incx: CBlasInt);

    /// The csscal routine performs a vector operation defined as
    ///
    /// $$x := a \cdot x$$
    ///
    /// where $a$ is a real scalar and $x$ is a complex single-precision vector with $n$ elements.
    /// This is a mixed-precision version that scales a complex vector by a real scalar.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `a` - Specifies the real scalar a.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). Complex vector. On exit, overwritten by the scaled vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    pub fn cblas_csscal(n: CBlasInt, a: CBlasFloat, x: *mut CBlasVoid, incx: CBlasInt);

    /// The zdscal routine performs a vector operation defined as
    ///
    /// $$x := a \cdot x$$
    ///
    /// where $a$ is a real scalar and $x$ is a complex double-precision vector with $n$ elements.
    /// This is a mixed-precision version that scales a complex vector by a real scalar.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `a` - Specifies the real scalar a.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). Complex double-precision vector. On exit, overwritten by the scaled vector.
    /// * `incx` - Specifies the increment for indexing vector x.
    pub fn cblas_zdscal(n: CBlasInt, a: CBlasDouble, x: *mut CBlasVoid, incx: CBlasInt);

    /// The ?swap routine performs a vector-vector operation defined as
    ///
    /// $$\mathrm{swap}(x, y)$$
    ///
    /// where $x$ and $y$ are vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). On exit, contains the elements of y.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). On exit, contains the elements of x.
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_sswap(
        n: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
        y: *mut CBlasFloat,
        incy: CBlasInt,
    );

    /// The ?swap routine performs a vector-vector operation defined as
    ///
    /// $$\mathrm{swap}(x, y)$$
    ///
    /// where $x$ and $y$ are vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). On exit, contains the elements of y.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). On exit, contains the elements of x.
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_dswap(
        n: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
        y: *mut CBlasDouble,
        incy: CBlasInt,
    );

    /// The ?swap routine performs a vector-vector operation defined as
    ///
    /// $$\mathrm{swap}(x, y)$$
    ///
    /// where $x$ and $y$ are complex vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). Complex vector. On exit, contains the elements of y.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). Complex vector. On exit, contains the elements of x.
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_cswap(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?swap routine performs a vector-vector operation defined as
    ///
    /// $$\mathrm{swap}(x, y)$$
    ///
    /// where $x$ and $y$ are complex double-precision vectors of $n$ elements.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vectors x and y.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)). Complex double-precision vector. On exit, contains the elements of y.
    /// * `incx` - Specifies the increment for indexing vector x.
    /// * `y` - Array, size at least (1 + (n-1)*abs(incy)). Complex double-precision vector. On exit, contains the elements of x.
    /// * `incy` - Specifies the increment for indexing vector y.
    pub fn cblas_zswap(
        n: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The i?amax routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{result} = \arg\max_{i=1,\ldots,n} |x_i|$$
    ///
    /// where $x$ is a vector with $n$ elements. Returns the index of the element with the largest absolute value.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the index (0-based) of the element with the largest absolute value. If n ≤ 0, returns 0.
    pub fn cblas_isamax(n: CBlasInt, x: *const CBlasFloat, incx: CBlasInt) -> CBlasIndex;

    /// The i?amax routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{result} = \arg\max_{i=1,\ldots,n} |x_i|$$
    ///
    /// where $x$ is a vector with $n$ elements. Returns the index of the element with the largest absolute value.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the index (0-based) of the element with the largest absolute value. If n ≤ 0, returns 0.
    pub fn cblas_idamax(n: CBlasInt, x: *const CBlasDouble, incx: CBlasInt) -> CBlasIndex;

    /// The i?amax routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{result} = \arg\max_{i=1,\ldots,n} |x_i|$$
    ///
    /// where $x$ is a complex vector with $n$ elements. Returns the index of the element with the largest absolute value.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the index (0-based) of the element with the largest absolute value. If n ≤ 0, returns 0.
    pub fn cblas_icamax(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasIndex;

    /// The i?amax routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{result} = \arg\max_{i=1,\ldots,n} |x_i|$$
    ///
    /// where $x$ is a complex double-precision vector with $n$ elements. Returns the index of the element with the largest absolute value.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the index (0-based) of the element with the largest absolute value. If n ≤ 0, returns 0.
    pub fn cblas_izamax(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasIndex;

    /// The i?amin routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{result} = \arg\min_{i=1,\ldots,n} |x_i|$$
    ///
    /// where $x$ is a vector with $n$ elements. Returns the index of the element with the smallest absolute value.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the index (0-based) of the element with the smallest absolute value. If n ≤ 0, returns 0.
    pub fn cblas_isamin(n: CBlasInt, x: *const CBlasFloat, incx: CBlasInt) -> CBlasIndex;

    /// The i?amin routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{result} = \arg\min_{i=1,\ldots,n} |x_i|$$
    ///
    /// where $x$ is a vector with $n$ elements. Returns the index of the element with the smallest absolute value.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the index (0-based) of the element with the smallest absolute value. If n ≤ 0, returns 0.
    pub fn cblas_idamin(n: CBlasInt, x: *const CBlasDouble, incx: CBlasInt) -> CBlasIndex;

    /// The i?amin routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{result} = \arg\min_{i=1,\ldots,n} |x_i|$$
    ///
    /// where $x$ is a complex vector with $n$ elements. Returns the index of the element with the smallest absolute value.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the index (0-based) of the element with the smallest absolute value. If n ≤ 0, returns 0.
    pub fn cblas_icamin(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasIndex;

    /// The i?amin routine performs a vector reduction operation defined as
    ///
    /// $$\mathrm{result} = \arg\min_{i=1,\ldots,n} |x_i|$$
    ///
    /// where $x$ is a complex double-precision vector with $n$ elements. Returns the index of the element with the smallest absolute value.
    ///
    /// # Arguments
    /// * `n` - Specifies the number of elements in vector x.
    /// * `x` - Array, size at least (1 + (n-1)*abs(incx)).
    /// * `incx` - Specifies the increment for indexing vector x.
    ///
    /// # Returns
    /// Returns the index (0-based) of the element with the smallest absolute value. If n ≤ 0, returns 0.
    pub fn cblas_izamin(n: CBlasInt, x: *const CBlasVoid, incx: CBlasInt) -> CBlasIndex;

    /// The scabs1 routine computes the absolute value of a complex number defined as
    ///
    /// $$|z| = |\mathrm{Re}(z)| + |\mathrm{Im}(z)|$$
    ///
    /// # Arguments
    /// * `z` - Pointer to a complex single-precision number.
    ///
    /// # Returns
    /// Returns the sum of the absolute values of the real and imaginary parts.
    pub fn cblas_scabs1(z: *const CBlasVoid) -> CBlasFloat;

    /// The dcabs1 routine computes the absolute value of a complex number defined as
    ///
    /// $$|z| = |\mathrm{Re}(z)| + |\mathrm{Im}(z)|$$
    ///
    /// # Arguments
    /// * `z` - Pointer to a complex double-precision number.
    ///
    /// # Returns
    /// Returns the sum of the absolute values of the real and imaginary parts.
    pub fn cblas_dcabs1(z: *const CBlasVoid) -> CBlasDouble;
}