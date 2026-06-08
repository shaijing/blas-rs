use crate::cblas::cblas_types::*;

unsafe extern "C" {
    /// The ?gbmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^T \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^H \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are scalars, $x$ and $y$ are vectors,
    /// and $A$ is an $m \\times n$ band matrix with $k_l$ sub-diagonals and $k_u$ super-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `kl` - Specifies the number of sub-diagonals of the matrix $A$.
    /// * `ku` - Specifies the number of super-diagonals of the matrix $A$.
    /// * `alpha` - Specifies the scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k_l + k_u + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$ when trans = CblasNoTrans, otherwise $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (m-1) \cdot |\\text{incy}|)$ when trans = CblasNoTrans, otherwise $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_sgbmv(
        layout: CBlasLayout,
        trans: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        kl: CBlasInt,
        ku: CBlasInt,
        alpha: CBlasFloat,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *const CBlasFloat,
        incx: CBlasInt,
        beta: CBlasFloat,
        y: *mut CBlasFloat,
        incy: CBlasInt,
    );

    /// The ?gbmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^T \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^H \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are scalars, $x$ and $y$ are vectors,
    /// and $A$ is an $m \\times n$ band matrix with $k_l$ sub-diagonals and $k_u$ super-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `kl` - Specifies the number of sub-diagonals of the matrix $A$.
    /// * `ku` - Specifies the number of super-diagonals of the matrix $A$.
    /// * `alpha` - Specifies the scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k_l + k_u + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$ when trans = CblasNoTrans, otherwise $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (m-1) \cdot |\\text{incy}|)$ when trans = CblasNoTrans, otherwise $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_dgbmv(
        layout: CBlasLayout,
        trans: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        kl: CBlasInt,
        ku: CBlasInt,
        alpha: CBlasDouble,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *const CBlasDouble,
        incx: CBlasInt,
        beta: CBlasDouble,
        y: *mut CBlasDouble,
        incy: CBlasInt,
    );

    /// The ?gbmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^T \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^H \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are complex scalars, $x$ and $y$ are complex vectors,
    /// and $A$ is an $m \\times n$ complex band matrix with $k_l$ sub-diagonals and $k_u$ super-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `kl` - Specifies the number of sub-diagonals of the matrix $A$.
    /// * `ku` - Specifies the number of super-diagonals of the matrix $A$.
    /// * `alpha` - Specifies the complex scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the complex band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k_l + k_u + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$ when trans = CblasNoTrans, otherwise $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the complex scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (m-1) \cdot |\\text{incy}|)$ when trans = CblasNoTrans, otherwise $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_cgbmv(
        layout: CBlasLayout,
        trans: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        kl: CBlasInt,
        ku: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        beta: *const CBlasVoid,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?gbmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^T \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^H \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are complex double-precision scalars, $x$ and $y$ are complex double-precision vectors,
    /// and $A$ is an $m \\times n$ complex double-precision band matrix with $k_l$ sub-diagonals and $k_u$ super-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `kl` - Specifies the number of sub-diagonals of the matrix $A$.
    /// * `ku` - Specifies the number of super-diagonals of the matrix $A$.
    /// * `alpha` - Specifies the complex double-precision scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the complex double-precision band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k_l + k_u + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$ when trans = CblasNoTrans, otherwise $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the complex double-precision scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (m-1) \cdot |\\text{incy}|)$ when trans = CblasNoTrans, otherwise $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_zgbmv(
        layout: CBlasLayout,
        trans: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        kl: CBlasInt,
        ku: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        beta: *const CBlasVoid,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?gemv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^T \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are scalars, $x$ and $y$ are vectors,
    /// and $A$ is an $m \\times n$ general matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `alpha` - Specifies the scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the $m \\times n$ matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, m)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$ when trans = CblasNoTrans, otherwise $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (m-1) \cdot |\\text{incy}|)$ when trans = CblasNoTrans, otherwise $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_sgemv(
        layout: CBlasLayout,
        trans: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        alpha: CBlasFloat,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *const CBlasFloat,
        incx: CBlasInt,
        beta: CBlasFloat,
        y: *mut CBlasFloat,
        incy: CBlasInt,
    );

    /// The ?gemv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^T \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are double-precision scalars, $x$ and $y$ are double-precision vectors,
    /// and $A$ is an $m \\times n$ general double-precision matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `alpha` - Specifies the double-precision scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the $m \\times n$ double-precision matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, m)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$ when trans = CblasNoTrans, otherwise $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the double-precision scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (m-1) \cdot |\\text{incy}|)$ when trans = CblasNoTrans, otherwise $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_dgemv(
        layout: CBlasLayout,
        trans: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        alpha: CBlasDouble,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *const CBlasDouble,
        incx: CBlasInt,
        beta: CBlasDouble,
        y: *mut CBlasDouble,
        incy: CBlasInt,
    );

    /// The ?gemv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^T \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^H \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are complex scalars, $x$ and $y$ are complex vectors,
    /// and $A$ is an $m \\times n$ complex general matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `alpha` - Specifies the complex scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the $m \\times n$ complex matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, m)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$ when trans = CblasNoTrans, otherwise $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the complex scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (m-1) \cdot |\\text{incy}|)$ when trans = CblasNoTrans, otherwise $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_cgemv(
        layout: CBlasLayout,
        trans: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        beta: *const CBlasVoid,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?gemv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^T \\cdot x + \\beta \\cdot y$$
    ///
    /// or
    ///
    /// $$y := \\alpha \\cdot A^H \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are complex double-precision scalars, $x$ and $y$ are complex double-precision vectors,
    /// and $A$ is an $m \\times n$ complex double-precision general matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `alpha` - Specifies the complex double-precision scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the $m \\times n$ complex double-precision matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, m)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$ when trans = CblasNoTrans, otherwise $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the complex double-precision scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (m-1) \cdot |\\text{incy}|)$ when trans = CblasNoTrans, otherwise $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_zgemv(
        layout: CBlasLayout,
        trans: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        beta: *const CBlasVoid,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?ger routine performs a rank-1 update of a general matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^T + A$$
    ///
    /// where $\\alpha$ is a scalar, $x$ is an $m$-element vector, $y$ is an $n$-element vector,
    /// and $A$ is an $m \\times n$ general matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `alpha` - Specifies the scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `a` - Array, size `lda * n`. On entry, the $m \\times n$ matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, m)$.
    pub fn cblas_sger(
        layout: CBlasLayout,
        m: CBlasInt,
        n: CBlasInt,
        alpha: CBlasFloat,
        x: *const CBlasFloat,
        incx: CBlasInt,
        y: *const CBlasFloat,
        incy: CBlasInt,
        a: *mut CBlasFloat,
        lda: CBlasInt,
    );

    /// The ?ger routine performs a rank-1 update of a general matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^T + A$$
    ///
    /// where $\\alpha$ is a double-precision scalar, $x$ is an $m$-element double-precision vector,
    /// $y$ is an $n$-element double-precision vector, and $A$ is an $m \\times n$ double-precision general matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `alpha` - Specifies the double-precision scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `a` - Array, size `lda * n`. On entry, the $m \\times n$ double-precision matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, m)$.
    pub fn cblas_dger(
        layout: CBlasLayout,
        m: CBlasInt,
        n: CBlasInt,
        alpha: CBlasDouble,
        x: *const CBlasDouble,
        incx: CBlasInt,
        y: *const CBlasDouble,
        incy: CBlasInt,
        a: *mut CBlasDouble,
        lda: CBlasInt,
    );

    /// The ?gerc routine performs a conjugated rank-1 update of a complex general matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^H + A$$
    ///
    /// where $\\alpha$ is a complex scalar, $x$ is an $m$-element complex vector,
    /// $y$ is an $n$-element complex vector, and $A$ is an $m \\times n$ complex general matrix.
    /// The conjugate of vector $y$ is used in the computation.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `alpha` - Specifies the complex scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `a` - Array, size `lda * n`. On entry, the $m \\times n$ complex matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, m)$.
    pub fn cblas_cgerc(
        layout: CBlasLayout,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        a: *mut CBlasVoid,
        lda: CBlasInt,
    );

    /// The ?gerc routine performs a conjugated rank-1 update of a complex double-precision general matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^H + A$$
    ///
    /// where $\\alpha$ is a complex double-precision scalar, $x$ is an $m$-element complex double-precision vector,
    /// $y$ is an $n$-element complex double-precision vector, and $A$ is an $m \\times n$ complex double-precision general matrix.
    /// The conjugate of vector $y$ is used in the computation.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `alpha` - Specifies the complex double-precision scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `a` - Array, size `lda * n`. On entry, the $m \\times n$ complex double-precision matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, m)$.
    pub fn cblas_zgerc(
        layout: CBlasLayout,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        a: *mut CBlasVoid,
        lda: CBlasInt,
    );

    /// The ?geru routine performs an unconjugated rank-1 update of a complex general matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^T + A$$
    ///
    /// where $\\alpha$ is a complex scalar, $x$ is an $m$-element complex vector,
    /// $y$ is an $n$-element complex vector, and $A$ is an $m \\times n$ complex general matrix.
    /// The vector $y$ is used without conjugation in the computation.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `alpha` - Specifies the complex scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `a` - Array, size `lda * n`. On entry, the $m \\times n$ complex matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, m)$.
    pub fn cblas_cgeru(
        layout: CBlasLayout,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        a: *mut CBlasVoid,
        lda: CBlasInt,
    );

    /// The ?geru routine performs an unconjugated rank-1 update of a complex double-precision general matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^T + A$$
    ///
    /// where $\\alpha$ is a complex double-precision scalar, $x$ is an $m$-element complex double-precision vector,
    /// $y$ is an $n$-element complex double-precision vector, and $A$ is an $m \\times n$ complex double-precision general matrix.
    /// The vector $y$ is used without conjugation in the computation.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `m` - Specifies the number of rows of the matrix $A$.
    /// * `n` - Specifies the number of columns of the matrix $A$.
    /// * `alpha` - Specifies the complex double-precision scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (m-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `a` - Array, size `lda * n`. On entry, the $m \\times n$ complex double-precision matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, m)$.
    pub fn cblas_zgeru(
        layout: CBlasLayout,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        a: *mut CBlasVoid,
        lda: CBlasInt,
    );

    /// The ?hbmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are complex scalars, $x$ and $y$ are complex vectors,
    /// and $A$ is an $n \\times n$ complex Hermitian band matrix with $k$ super-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals of the matrix $A$.
    /// * `alpha` - Specifies the complex scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the Hermitian band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the complex scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_chbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        k: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        beta: *const CBlasVoid,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?hbmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are complex double-precision scalars, $x$ and $y$ are complex double-precision vectors,
    /// and $A$ is an $n \\times n$ complex double-precision Hermitian band matrix with $k$ super-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals of the matrix $A$.
    /// * `alpha` - Specifies the complex double-precision scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the complex double-precision Hermitian band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the complex double-precision scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_zhbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        k: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        beta: *const CBlasVoid,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?hemv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are complex scalars, $x$ and $y$ are complex vectors,
    /// and $A$ is an $n \\times n$ complex Hermitian matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the complex scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the complex Hermitian matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the complex scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_chemv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        beta: *const CBlasVoid,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?hemv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are complex double-precision scalars, $x$ and $y$ are complex double-precision vectors,
    /// and $A$ is an $n \\times n$ complex double-precision Hermitian matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the complex double-precision scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the complex double-precision Hermitian matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the complex double-precision scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_zhemv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *const CBlasVoid,
        incx: CBlasInt,
        beta: *const CBlasVoid,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?her routine performs a rank-1 update of a Hermitian matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot x^H + A$$
    ///
    /// where $\\alpha$ is a real scalar, $x$ is a complex vector,
    /// and $A$ is an $n \\times n$ complex Hermitian matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the real scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `a` - Array, size `lda * n`. On entry, the complex Hermitian matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    pub fn cblas_cher(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasFloat,
        x: *const CBlasVoid,
        incx: CBlasInt,
        a: *mut CBlasVoid,
        lda: CBlasInt,
    );

    /// The ?her routine performs a rank-1 update of a Hermitian matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot x^H + A$$
    ///
    /// where $\\alpha$ is a real double-precision scalar, $x$ is a complex double-precision vector,
    /// and $A$ is an $n \\times n$ complex double-precision Hermitian matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the real double-precision scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `a` - Array, size `lda * n`. On entry, the complex double-precision Hermitian matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    pub fn cblas_zher(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasDouble,
        x: *const CBlasVoid,
        incx: CBlasInt,
        a: *mut CBlasVoid,
        lda: CBlasInt,
    );

    /// The ?her2 routine performs a rank-2 update of a Hermitian matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^H + \\overline{\\alpha} \\cdot y \\cdot x^H + A$$
    ///
    /// where $\\alpha$ is a complex scalar, $x$ and $y$ are complex vectors,
    /// and $A$ is an $n \\times n$ complex Hermitian matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the complex scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `a` - Array, size `lda * n`. On entry, the complex Hermitian matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    pub fn cblas_cher2(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        a: *mut CBlasVoid,
        lda: CBlasInt,
    );

    /// The ?her2 routine performs a rank-2 update of a Hermitian matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^H + \\overline{\\alpha} \\cdot y \\cdot x^H + A$$
    ///
    /// where $\\alpha$ is a complex double-precision scalar, $x$ and $y$ are complex double-precision vectors,
    /// and $A$ is an $n \\times n$ complex double-precision Hermitian matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the complex double-precision scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `a` - Array, size `lda * n`. On entry, the complex double-precision Hermitian matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    pub fn cblas_zher2(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        a: *mut CBlasVoid,
        lda: CBlasInt,
    );

    /// The ?hpmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are complex scalars, $x$ and $y$ are complex vectors,
    /// and $A$ is an $n \\times n$ complex Hermitian matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the complex scalar $\\alpha$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the Hermitian matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the complex scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_chpmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        ap: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        beta: *const CBlasVoid,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?hpmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are complex double-precision scalars, $x$ and $y$ are complex double-precision vectors,
    /// and $A$ is an $n \\times n$ complex double-precision Hermitian matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the complex double-precision scalar $\\alpha$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the complex double-precision Hermitian matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the complex double-precision scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_zhpmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        ap: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        beta: *const CBlasVoid,
        y: *mut CBlasVoid,
        incy: CBlasInt,
    );

    /// The ?hpr routine performs a rank-1 update of a Hermitian packed matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot x^H + A$$
    ///
    /// where $\\alpha$ is a real scalar, $x$ is a complex vector,
    /// and $A$ is an $n \\times n$ complex Hermitian matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the real scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the complex Hermitian matrix $A$ in packed storage. On exit, overwritten by the updated matrix.
    pub fn cblas_chpr(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasFloat,
        x: *const CBlasVoid,
        incx: CBlasInt,
        ap: *mut CBlasVoid,
    );

    /// The ?hpr routine performs a rank-1 update of a Hermitian packed matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot x^H + A$$
    ///
    /// where $\\alpha$ is a real double-precision scalar, $x$ is a complex double-precision vector,
    /// and $A$ is an $n \\times n$ complex double-precision Hermitian matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of matrix $A$.
    /// * `alpha` - Specifies the real double-precision scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the complex double-precision Hermitian matrix $A$ in packed storage. On exit, overwritten by the updated matrix.
    pub fn cblas_zhpr(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasDouble,
        x: *const CBlasVoid,
        incx: CBlasInt,
        ap: *mut CBlasVoid,
    );

    /// The ?hpr2 routine performs a rank-2 update of a Hermitian packed matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^H + \\overline{\\alpha} \\cdot y \\cdot x^H + A$$
    ///
    /// where $\\alpha$ is a complex scalar, $x$ and $y$ are complex vectors,
    /// and $A$ is an $n \\times n$ complex Hermitian matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the complex scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the complex Hermitian matrix $A$ in packed storage. On exit, overwritten by the updated matrix.
    pub fn cblas_chpr2(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        ap: *mut CBlasVoid,
    );

    /// The ?hpr2 routine performs a rank-2 update of a Hermitian packed matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^H + \\overline{\\alpha} \\cdot y \\cdot x^H + A$$
    ///
    /// where $\\alpha$ is a complex double-precision scalar, $x$ and $y$ are complex double-precision vectors,
    /// and $A$ is an $n \\times n$ complex double-precision Hermitian matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the Hermitian matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the complex double-precision scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the complex double-precision Hermitian matrix $A$ in packed storage. On exit, overwritten by the updated matrix.
    pub fn cblas_zhpr2(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        x: *const CBlasVoid,
        incx: CBlasInt,
        y: *const CBlasVoid,
        incy: CBlasInt,
        ap: *mut CBlasVoid,
    );

    /// The ?sbmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are scalars, $x$ and $y$ are vectors,
    /// and $A$ is an $n \\times n$ symmetric band matrix with $k$ super-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals of the matrix $A$.
    /// * `alpha` - Specifies the scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the symmetric band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_ssbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        k: CBlasInt,
        alpha: CBlasFloat,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *const CBlasFloat,
        incx: CBlasInt,
        beta: CBlasFloat,
        y: *mut CBlasFloat,
        incy: CBlasInt,
    );

    /// The ?sbmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are double-precision scalars, $x$ and $y$ are double-precision vectors,
    /// and $A$ is an $n \\times n$ double-precision symmetric band matrix with $k$ super-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals of the matrix $A$.
    /// * `alpha` - Specifies the double-precision scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the double-precision symmetric band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the double-precision scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_dsbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        k: CBlasInt,
        alpha: CBlasDouble,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *const CBlasDouble,
        incx: CBlasInt,
        beta: CBlasDouble,
        y: *mut CBlasDouble,
        incy: CBlasInt,
    );

    /// The ?spmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are scalars, $x$ and $y$ are vectors,
    /// and $A$ is an $n \\times n$ symmetric matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the scalar $\\alpha$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the symmetric matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_sspmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasFloat,
        ap: *const CBlasFloat,
        x: *const CBlasFloat,
        incx: CBlasInt,
        beta: CBlasFloat,
        y: *mut CBlasFloat,
        incy: CBlasInt,
    );

    /// The ?spmv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are double-precision scalars, $x$ and $y$ are double-precision vectors,
    /// and $A$ is an $n \\times n$ double-precision symmetric matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the double-precision scalar $\\alpha$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the double-precision symmetric matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the double-precision scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_dspmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasDouble,
        ap: *const CBlasDouble,
        x: *const CBlasDouble,
        incx: CBlasInt,
        beta: CBlasDouble,
        y: *mut CBlasDouble,
        incy: CBlasInt,
    );

    /// The ?spr routine performs a rank-1 update of a symmetric packed matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot x^T + A$$
    ///
    /// where $\\alpha$ is a scalar, $x$ is a vector,
    /// and $A$ is an $n \\times n$ symmetric matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the symmetric matrix $A$ in packed storage. On exit, overwritten by the updated matrix.
    pub fn cblas_sspr(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasFloat,
        x: *const CBlasFloat,
        incx: CBlasInt,
        ap: *mut CBlasFloat,
    );

    /// The ?spr routine performs a rank-1 update of a symmetric packed matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot x^T + A$$
    ///
    /// where $\\alpha$ is a double-precision scalar, $x$ is a double-precision vector,
    /// and $A$ is an $n \\times n$ double-precision symmetric matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the double-precision scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the double-precision symmetric matrix $A$ in packed storage. On exit, overwritten by the updated matrix.
    pub fn cblas_dspr(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasDouble,
        x: *const CBlasDouble,
        incx: CBlasInt,
        ap: *mut CBlasDouble,
    );

    /// The ?spr2 routine performs a rank-2 update of a symmetric packed matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^T + \\alpha \\cdot y \\cdot x^T + A$$
    ///
    /// where $\\alpha$ is a scalar, $x$ and $y$ are vectors,
    /// and $A$ is an $n \\times n$ symmetric matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the symmetric matrix $A$ in packed storage. On exit, overwritten by the updated matrix.
    pub fn cblas_sspr2(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasFloat,
        x: *const CBlasFloat,
        incx: CBlasInt,
        y: *const CBlasFloat,
        incy: CBlasInt,
        ap: *mut CBlasFloat,
    );

    /// The ?spr2 routine performs a rank-2 update of a symmetric packed matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^T + \\alpha \\cdot y \\cdot x^T + A$$
    ///
    /// where $\\alpha$ is a double-precision scalar, $x$ and $y$ are double-precision vectors,
    /// and $A$ is an $n \\times n$ double-precision symmetric matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the double-precision scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the double-precision symmetric matrix $A$ in packed storage. On exit, overwritten by the updated matrix.
    pub fn cblas_dspr2(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasDouble,
        x: *const CBlasDouble,
        incx: CBlasInt,
        y: *const CBlasDouble,
        incy: CBlasInt,
        ap: *mut CBlasDouble,
    );

    /// The ?symv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are scalars, $x$ and $y$ are vectors,
    /// and $A$ is an $n \\times n$ symmetric matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the symmetric matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_ssymv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasFloat,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *const CBlasFloat,
        incx: CBlasInt,
        beta: CBlasFloat,
        y: *mut CBlasFloat,
        incy: CBlasInt,
    );

    /// The ?symv routine performs a matrix-vector operation defined as
    ///
    /// $$y := \\alpha \\cdot A \\cdot x + \\beta \\cdot y$$
    ///
    /// where $\\alpha$ and $\\beta$ are double-precision scalars, $x$ and $y$ are double-precision vectors,
    /// and $A$ is an $n \\times n$ double-precision symmetric matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the double-precision scalar $\\alpha$.
    /// * `a` - Array, size `lda * n`. On entry, the double-precision symmetric matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `beta` - Specifies the double-precision scalar $\\beta$.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    pub fn cblas_dsymv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasDouble,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *const CBlasDouble,
        incx: CBlasInt,
        beta: CBlasDouble,
        y: *mut CBlasDouble,
        incy: CBlasInt,
    );

    /// The ?syr routine performs a rank-1 update of a symmetric matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot x^T + A$$
    ///
    /// where $\\alpha$ is a scalar, $x$ is a vector,
    /// and $A$ is an $n \\times n$ symmetric matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `a` - Array, size `lda * n`. On entry, the symmetric matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    pub fn cblas_ssyr(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasFloat,
        x: *const CBlasFloat,
        incx: CBlasInt,
        a: *mut CBlasFloat,
        lda: CBlasInt,
    );

    /// The ?syr routine performs a rank-1 update of a symmetric matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot x^T + A$$
    ///
    /// where $\\alpha$ is a double-precision scalar, $x$ is a double-precision vector,
    /// and $A$ is an $n \\times n$ double-precision symmetric matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the double-precision scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `a` - Array, size `lda * n`. On entry, the double-precision symmetric matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    pub fn cblas_dsyr(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasDouble,
        x: *const CBlasDouble,
        incx: CBlasInt,
        a: *mut CBlasDouble,
        lda: CBlasInt,
    );

    /// The ?syr2 routine performs a rank-2 update of a symmetric matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^T + \\alpha \\cdot y \\cdot x^T + A$$
    ///
    /// where $\\alpha$ is a scalar, $x$ and $y$ are vectors,
    /// and $A$ is an $n \\times n$ symmetric matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `a` - Array, size `lda * n`. On entry, the symmetric matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    pub fn cblas_ssyr2(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasFloat,
        x: *const CBlasFloat,
        incx: CBlasInt,
        y: *const CBlasFloat,
        incy: CBlasInt,
        a: *mut CBlasFloat,
        lda: CBlasInt,
    );

    /// The ?syr2 routine performs a rank-2 update of a symmetric matrix defined as
    ///
    /// $$A := \\alpha \\cdot x \\cdot y^T + \\alpha \\cdot y \\cdot x^T + A$$
    ///
    /// where $\\alpha$ is a double-precision scalar, $x$ and $y$ are double-precision vectors,
    /// and $A$ is an $n \\times n$ double-precision symmetric matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the symmetric matrix $A$ is used: CblasUpper or CblasLower.
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `alpha` - Specifies the double-precision scalar $\\alpha$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    /// * `y` - Array, size at least $(1 + (n-1) \cdot |\\text{incy}|)$.
    /// * `incy` - Specifies the increment for indexing vector `y`.
    /// * `a` - Array, size `lda * n`. On entry, the double-precision symmetric matrix $A$. On exit, overwritten by the updated matrix.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    pub fn cblas_dsyr2(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasDouble,
        x: *const CBlasDouble,
        incx: CBlasInt,
        y: *const CBlasDouble,
        incy: CBlasInt,
        a: *mut CBlasDouble,
        lda: CBlasInt,
    );

    /// The ?tbmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// where $x$ is a vector and $A$ is an $n \\times n$ triangular band matrix with $k$ super-diagonals or sub-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals (if uplo = CblasUpper) or sub-diagonals (if uplo = CblasLower) of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the triangular band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_stbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    /// The ?tbmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// where $x$ is a double-precision vector and $A$ is an $n \\times n$ double-precision triangular band matrix with $k$ super-diagonals or sub-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals (if uplo = CblasUpper) or sub-diagonals (if uplo = CblasLower) of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the double-precision triangular band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the double-precision vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_dtbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    /// The ?tbmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^H \\cdot x$$
    ///
    /// where $x$ is a complex vector and $A$ is an $n \\times n$ complex triangular band matrix with $k$ super-diagonals or sub-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals (if uplo = CblasUpper) or sub-diagonals (if uplo = CblasLower) of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the complex triangular band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the complex vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ctbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    /// The ?tbmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^H \\cdot x$$
    ///
    /// where $x$ is a complex double-precision vector and $A$ is an $n \\times n$ complex double-precision triangular band matrix with $k$ super-diagonals or sub-diagonals.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals (if uplo = CblasUpper) or sub-diagonals (if uplo = CblasLower) of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the complex double-precision triangular band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the complex double-precision vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ztbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    /// The ?tbsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// where $x$ and $b$ are vectors and $A$ is an $n \\times n$ triangular band matrix with $k$ super-diagonals or sub-diagonals.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals (if uplo = CblasUpper) or sub-diagonals (if uplo = CblasLower) of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the triangular band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_stbsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    /// The ?tbsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// where $x$ and $b$ are double-precision vectors and $A$ is an $n \\times n$ double-precision triangular band matrix with $k$ super-diagonals or sub-diagonals.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals (if uplo = CblasUpper) or sub-diagonals (if uplo = CblasLower) of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the double-precision triangular band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side double-precision vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_dtbsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    /// The ?tbsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^H \\cdot x = b$$
    ///
    /// where $x$ and $b$ are complex vectors and $A$ is an $n \\times n$ complex triangular band matrix with $k$ super-diagonals or sub-diagonals.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals (if uplo = CblasUpper) or sub-diagonals (if uplo = CblasLower) of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the complex triangular band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side complex vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ctbsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    /// The ?tbsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^H \\cdot x = b$$
    ///
    /// where $x$ and $b$ are complex double-precision vectors and $A$ is an $n \\times n$ complex double-precision triangular band matrix with $k$ super-diagonals or sub-diagonals.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `k` - Specifies the number of super-diagonals (if uplo = CblasUpper) or sub-diagonals (if uplo = CblasLower) of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the complex double-precision triangular band matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $(k + 1)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side complex double-precision vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ztbsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    /// The ?tpmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// where $x$ is a vector and $A$ is an $n \\times n$ triangular matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the triangular matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_stpmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        ap: *const CBlasFloat,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    /// The ?tpmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// where $x$ is a double-precision vector and $A$ is an $n \\times n$ double-precision triangular matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the double-precision triangular matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the double-precision vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_dtpmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        ap: *const CBlasDouble,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    /// The ?tpmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^H \\cdot x$$
    ///
    /// where $x$ is a complex vector and $A$ is an $n \\times n$ complex triangular matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the complex triangular matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the complex vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ctpmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        ap: *const CBlasVoid,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    /// The ?tpmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^H \\cdot x$$
    ///
    /// where $x$ is a complex double-precision vector and $A$ is an $n \\times n$ complex double-precision triangular matrix stored in packed format.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the complex double-precision triangular matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the complex double-precision vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ztpmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        ap: *const CBlasVoid,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    /// The ?tpsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// where $x$ and $b$ are vectors and $A$ is an $n \\times n$ triangular matrix stored in packed format.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the triangular matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_stpsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        ap: *const CBlasFloat,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    /// The ?tpsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// where $x$ and $b$ are double-precision vectors and $A$ is an $n \\times n$ double-precision triangular matrix stored in packed format.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the double-precision triangular matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side double-precision vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_dtpsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        ap: *const CBlasDouble,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    /// The ?tpsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^H \\cdot x = b$$
    ///
    /// where $x$ and $b$ are complex vectors and $A$ is an $n \\times n$ complex triangular matrix stored in packed format.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the complex triangular matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side complex vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ctpsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        ap: *const CBlasVoid,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    /// The ?tpsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^H \\cdot x = b$$
    ///
    /// where $x$ and $b$ are complex double-precision vectors and $A$ is an $n \\times n$ complex double-precision triangular matrix stored in packed format.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `ap` - Array, size at least $(n(n+1)/2)$. On entry, the complex double-precision triangular matrix $A$ in packed storage.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side complex double-precision vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ztpsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        ap: *const CBlasVoid,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    /// The ?trmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// where $x$ is a vector and $A$ is an $n \\times n$ triangular matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the triangular matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_strmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    /// The ?trmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// where $x$ is a double-precision vector and $A$ is an $n \\times n$ double-precision triangular matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the double-precision triangular matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the double-precision vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_dtrmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    /// The ?trmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^H \\cdot x$$
    ///
    /// where $x$ is a complex vector and $A$ is an $n \\times n$ complex triangular matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the complex triangular matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the complex vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ctrmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    /// The ?trmv routine performs a matrix-vector operation defined as
    ///
    /// $$x := A \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^T \\cdot x$$
    ///
    /// or
    ///
    /// $$x := A^H \\cdot x$$
    ///
    /// where $x$ is a complex double-precision vector and $A$ is an $n \\times n$ complex double-precision triangular matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the complex double-precision triangular matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the complex double-precision vector $x$. On exit, overwritten by the transformed vector.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ztrmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    /// The ?trsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// where $x$ and $b$ are vectors and $A$ is an $n \\times n$ triangular matrix.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the triangular matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_strsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    /// The ?trsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// where $x$ and $b$ are double-precision vectors and $A$ is an $n \\times n$ double-precision triangular matrix.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the double-precision triangular matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side double-precision vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_dtrsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    /// The ?trsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^H \\cdot x = b$$
    ///
    /// where $x$ and $b$ are complex vectors and $A$ is an $n \\times n$ complex triangular matrix.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the complex triangular matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side complex vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ctrsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    /// The ?trsv routine solves a system of linear equations defined as
    ///
    /// $$A \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^T \\cdot x = b$$
    ///
    /// or
    ///
    /// $$A^H \\cdot x = b$$
    ///
    /// where $x$ and $b$ are complex double-precision vectors and $A$ is an $n \\times n$ complex double-precision triangular matrix.
    /// The solution $x$ overwrites the input vector $b$.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `uplo` - Specifies whether the upper or lower triangular part of the matrix $A$ is used: CblasUpper or CblasLower.
    /// * `trans` - Specifies the operation: CblasNoTrans, CblasTrans, or CblasConjTrans.
    /// * `diag` - Specifies whether the matrix $A$ is unit triangular (CblasUnit) or non-unit triangular (CblasNonUnit).
    /// * `n` - Specifies the order of the matrix $A$.
    /// * `a` - Array, size `lda * n`. On entry, the complex double-precision triangular matrix $A$.
    /// * `lda` - Specifies the leading dimension of `a` as declared in the calling program. Must be at least $\\max(1, n)$.
    /// * `x` - Array, size at least $(1 + (n-1) \cdot |\\text{incx}|)$. On entry, the right-hand side complex double-precision vector $b$. On exit, overwritten by the solution vector $x$.
    /// * `incx` - Specifies the increment for indexing vector `x`.
    pub fn cblas_ztrsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CBlasDiag,
        n: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

}
