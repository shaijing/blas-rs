use crate::cblas::cblas_types::*;

unsafe extern "C" {

        /// The ?gemm routines compute a scalar-matrix-matrix product and add the result to a scalar-matrix product, with general matrices. The operation is defined as
    ///
    /// $$C :=  \alpha * \mathrm{op}(A) * \mathrm{op}(B) + \beta * C$$
    ///
    /// where
    ///
    /// $\mathrm{op}(X)$ is one of $\mathrm{op}(X) = X$, or $\mathrm{op}(X)=X^{\top}$, or $\mathrm{op}(X)=X^{\mathrm{H}}$
    ///
    /// $\alpha$ and $\beta$ are scalars,
    ///
    /// $A,$ $B$ and $C$ are matrices:
    ///
    /// $\mathrm{op}(A)$ is an $m$-by-$k$ matrix,
    ///
    /// $\mathrm{op}(B)$ is a $k$-by-$n$ matrix,
    ///
    /// $C$ is an $m$-by-$n$ matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `transa` - Specifies the form of $\mathrm{op}(A)$ used in the matrix multiplication:
    ///     * if `transa`=CblasNoTrans, then $\mathrm{op}(A) = A$;
    ///     * if transa=CblasTrans, then $\mathrm{op}(A) = A^{\top}$;
    ///     * if transa=CblasConjTrans, then $\mathrm{op}(A) = A^{\mathrm{H}}$.
    /// * `transb` - Specifies the form of $\mathrm{op}(B)$ used in the matrix multiplication.
    /// * `m` - Specifies the number of rows of the matrix $\mathrm{op}(A)$ and of the matrix $C$. The value of $m$ must be at least zero.
    /// * `n` - Specifies the number of columns of the matrix $\mathrm{op}(B)$ and the number of columns of the matrix $C$. The value of $n$ must be at least zero.
    /// * `k` - Specifies the number of columns of the matrix $\mathrm{op}(A)$ and the number of rows of the matrix $\mathrm{op}(B)$. The value of $k$ must be at least zero.
    /// * `alpha` - Specifies the scalar alpha.
    /// * `a` -
    ///
    /// | $A$       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |     Array, size $lda*k$.<br> Before entry, the leading $k$-by-$n$ part of the array $a$ must contain the matrix $A$.      |    Array, size $lda*m$. <br> Before entry, the leading $k$-by-$m$ part of the array $a$ must contain the matrix $A$.       |
    /// | `layout` = CBlasRowMajor   |     Array, size $lda*m$.<br> Before entry, the leading $k$-by-$m$ part of the array $a$ must contain the matrix $A$.        |  Array, size $lda*k$.<br> Before entry, the leading $m$-by-$k$ part of the array $a$ must contain the matrix $A$.          |
    ///
    /// * `lda` - Specifies the leading dimension of a as declared in the calling (sub)program.
    ///
    /// | lda       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |    lda must be at least $\max(1, m)$.     |    lda must be at least $\max(1, k)$.      |
    /// | `layout` = CBlasRowMajor   |     lda must be at least $\max(1, k)$.      |  lda must be at least $\max(1, m)$.         |
    ///
    /// * `b` -
    ///
    /// | $B$       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |     Array, size $ldb*n$.<br> Before entry, the leading $k$-by-$n$ part of the array $b$ must contain the matrix $B$.      |    Array, size $ldb*k$. <br> Before entry, the leading $n$-by-$k$ part of the array $b$ must contain the matrix $B$.       |
    /// | `layout` = CBlasRowMajor   |     Array, size $ldb*k$.<br> Before entry, the leading $n$-by-$k$ part of the array $b$ must contain the matrix $B$.        |  Array, size $ldb*n$.<br> Before entry, the leading $k$-by-$n$ part of the array $b$ must contain the matrix $B$.          |
    ///
    /// * `ldb` - Specifies the leading dimension of b as declared in the calling (sub)program.
    ///
    /// | ldb       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |    ldb must be at least $\max(1, k)$.     |    ldb must be at least $\max(1, n)$.      |
    /// | `layout` = CBlasRowMajor   |     ldb must be at least $\max(1, n)$.      |  ldb must be at least $\max(1, k)$.         |
    ///
    /// * `beta` - Specifies the scalar beta. When beta is equal to zero, then c need not be set on input.
    /// * `c` -
    ///
    /// | $C$       |       |
    /// | :------:     | :------------: |
    /// | `layout` = CBlasColMajor  |    Array, size $ldc*n$. Before entry, the leading $m$-by-$n$ part of the array $c$ must contain the matrix $C$, except when beta is equal to zero, in which case $c$ need not be set on entry.      |  
    /// | `layout` = CBlasRowMajor   |     Array, size $ldc*m$. Before entry, the leading $n$-by-$m$ part of the array $c$ must contain the matrix $C$, except when beta is equal to zero, in which case $c$ need not be set on entry.        |
    ///
    /// * `ldc` - Specifies the leading dimension of c as declared in the calling (sub)program.
    ///
    /// | ldc       |       |
    /// | :------:     | :------------: |
    /// | `layout` = CBlasColMajor  |   ldc must be at least $\max(1, m)$.     |  
    /// | `layout` = CBlasRowMajor   |     ldc must be at least $\max(1, n)$.       |
    ///
    /// # Output
    /// * `c` - Overwritten by the $m$-by-$n$ matrix
    pub fn cblas_hgemm(
        layout: CBlasLayout,
        transa: CBlasTranspose,
        transb: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        k: CBlasInt,
        alpha: CBlasF16,
        a: *const CBlasF16,
        lda: CBlasInt,
        b: *const CBlasF16,
        ldb: CBlasInt,
        beta: CBlasF16,
        c: *mut CBlasF16,
        ldc: CBlasInt,
    );

    /// The ?gemm routines compute a scalar-matrix-matrix product and add the result to a scalar-matrix product, with general matrices. The operation is defined as
    ///
    /// $$C :=  \alpha * \mathrm{op}(A) * \mathrm{op}(B) + \beta * C$$
    ///
    /// where
    ///
    /// $\mathrm{op}(X)$ is one of $\mathrm{op}(X) = X$, or $\mathrm{op}(X)=X^{\top}$, or $\mathrm{op}(X)=X^{\mathrm{H}}$
    ///
    /// $\alpha$ and $\beta$ are scalars,
    ///
    /// $A,$ $B$ and $C$ are matrices:
    ///
    /// $\mathrm{op}(A)$ is an $m$-by-$k$ matrix,
    ///
    /// $\mathrm{op}(B)$ is a $k$-by-$n$ matrix,
    ///
    /// $C$ is an $m$-by-$n$ matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `transa` - Specifies the form of $\mathrm{op}(A)$ used in the matrix multiplication:
    ///     * if `transa`=CblasNoTrans, then $\mathrm{op}(A) = A$;
    ///     * if transa=CblasTrans, then $\mathrm{op}(A) = A^{\top}$;
    ///     * if transa=CblasConjTrans, then $\mathrm{op}(A) = A^{\mathrm{H}}$.
    /// * `transb` - Specifies the form of $\mathrm{op}(B)$ used in the matrix multiplication.
    /// * `m` - Specifies the number of rows of the matrix $\mathrm{op}(A)$ and of the matrix $C$. The value of $m$ must be at least zero.
    /// * `n` - Specifies the number of columns of the matrix $\mathrm{op}(B)$ and the number of columns of the matrix $C$. The value of $n$ must be at least zero.
    /// * `k` - Specifies the number of columns of the matrix $\mathrm{op}(A)$ and the number of rows of the matrix $\mathrm{op}(B)$. The value of $k$ must be at least zero.
    /// * `alpha` - Specifies the scalar alpha.
    /// * `a` -
    ///
    /// | $A$       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |     Array, size $lda*k$.<br> Before entry, the leading $k$-by-$n$ part of the array $a$ must contain the matrix $A$.      |    Array, size $lda*m$. <br> Before entry, the leading $k$-by-$m$ part of the array $a$ must contain the matrix $A$.       |
    /// | `layout` = CBlasRowMajor   |     Array, size $lda*m$.<br> Before entry, the leading $k$-by-$m$ part of the array $a$ must contain the matrix $A$.        |  Array, size $lda*k$.<br> Before entry, the leading $m$-by-$k$ part of the array $a$ must contain the matrix $A$.          |
    ///
    /// * `lda` - Specifies the leading dimension of a as declared in the calling (sub)program.
    ///
    /// | lda       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |    lda must be at least $\max(1, m)$.     |    lda must be at least $\max(1, k)$.      |
    /// | `layout` = CBlasRowMajor   |     lda must be at least $\max(1, k)$.      |  lda must be at least $\max(1, m)$.         |
    ///
    /// * `b` -
    ///
    /// | $B$       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |     Array, size $ldb*n$.<br> Before entry, the leading $k$-by-$n$ part of the array $b$ must contain the matrix $B$.      |    Array, size $ldb*k$. <br> Before entry, the leading $n$-by-$k$ part of the array $b$ must contain the matrix $B$.       |
    /// | `layout` = CBlasRowMajor   |     Array, size $ldb*k$.<br> Before entry, the leading $n$-by-$k$ part of the array $b$ must contain the matrix $B$.        |  Array, size $ldb*n$.<br> Before entry, the leading $k$-by-$n$ part of the array $b$ must contain the matrix $B$.          |
    ///
    /// * `ldb` - Specifies the leading dimension of b as declared in the calling (sub)program.
    ///
    /// | ldb       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |    ldb must be at least $\max(1, k)$.     |    ldb must be at least $\max(1, n)$.      |
    /// | `layout` = CBlasRowMajor   |     ldb must be at least $\max(1, n)$.      |  ldb must be at least $\max(1, k)$.         |
    ///
    /// * `beta` - Specifies the scalar beta. When beta is equal to zero, then c need not be set on input.
    /// * `c` -
    ///
    /// | $C$       |       |
    /// | :------:     | :------------: |
    /// | `layout` = CBlasColMajor  |    Array, size $ldc*n$. Before entry, the leading $m$-by-$n$ part of the array $c$ must contain the matrix $C$, except when beta is equal to zero, in which case $c$ need not be set on entry.      |  
    /// | `layout` = CBlasRowMajor   |     Array, size $ldc*m$. Before entry, the leading $n$-by-$m$ part of the array $c$ must contain the matrix $C$, except when beta is equal to zero, in which case $c$ need not be set on entry.        |
    ///
    /// * `ldc` - Specifies the leading dimension of c as declared in the calling (sub)program.
    ///
    /// | ldc       |       |
    /// | :------:     | :------------: |
    /// | `layout` = CBlasColMajor  |   ldc must be at least $\max(1, m)$.     |  
    /// | `layout` = CBlasRowMajor   |     ldc must be at least $\max(1, n)$.       |
    ///
    /// # Output
    /// * `c` - Overwritten by the $m$-by-$n$ matrix
    pub fn cblas_sgemm(
        layout: CBlasLayout,
        transa: CBlasTranspose,
        transb: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        k: CBlasInt,
        alpha: f32,
        a: *const f32,
        lda: CBlasInt,
        b: *const f32,
        ldb: CBlasInt,
        beta: f32,
        c: *mut f32,
        ldc: CBlasInt,
    );

        /// The ?gemm routines compute a scalar-matrix-matrix product and add the result to a scalar-matrix product, with general matrices. The operation is defined as
    ///
    /// $$C :=  \alpha * \mathrm{op}(A) * \mathrm{op}(B) + \beta * C$$
    ///
    /// where
    ///
    /// $\mathrm{op}(X)$ is one of $\mathrm{op}(X) = X$, or $\mathrm{op}(X)=X^{\top}$, or $\mathrm{op}(X)=X^{\mathrm{H}}$
    ///
    /// $\alpha$ and $\beta$ are scalars,
    ///
    /// $A,$ $B$ and $C$ are matrices:
    ///
    /// $\mathrm{op}(A)$ is an $m$-by-$k$ matrix,
    ///
    /// $\mathrm{op}(B)$ is a $k$-by-$n$ matrix,
    ///
    /// $C$ is an $m$-by-$n$ matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `transa` - Specifies the form of $\mathrm{op}(A)$ used in the matrix multiplication:
    ///     * if `transa`=CblasNoTrans, then $\mathrm{op}(A) = A$;
    ///     * if transa=CblasTrans, then $\mathrm{op}(A) = A^{\top}$;
    ///     * if transa=CblasConjTrans, then $\mathrm{op}(A) = A^{\mathrm{H}}$.
    /// * `transb` - Specifies the form of $\mathrm{op}(B)$ used in the matrix multiplication.
    /// * `m` - Specifies the number of rows of the matrix $\mathrm{op}(A)$ and of the matrix $C$. The value of $m$ must be at least zero.
    /// * `n` - Specifies the number of columns of the matrix $\mathrm{op}(B)$ and the number of columns of the matrix $C$. The value of $n$ must be at least zero.
    /// * `k` - Specifies the number of columns of the matrix $\mathrm{op}(A)$ and the number of rows of the matrix $\mathrm{op}(B)$. The value of $k$ must be at least zero.
    /// * `alpha` - Specifies the scalar alpha.
    /// * `a` -
    ///
    /// | $A$       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |     Array, size $lda*k$.<br> Before entry, the leading $k$-by-$n$ part of the array $a$ must contain the matrix $A$.      |    Array, size $lda*m$. <br> Before entry, the leading $k$-by-$m$ part of the array $a$ must contain the matrix $A$.       |
    /// | `layout` = CBlasRowMajor   |     Array, size $lda*m$.<br> Before entry, the leading $k$-by-$m$ part of the array $a$ must contain the matrix $A$.        |  Array, size $lda*k$.<br> Before entry, the leading $m$-by-$k$ part of the array $a$ must contain the matrix $A$.          |
    ///
    /// * `lda` - Specifies the leading dimension of a as declared in the calling (sub)program.
    ///
    /// | lda       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |    lda must be at least $\max(1, m)$.     |    lda must be at least $\max(1, k)$.      |
    /// | `layout` = CBlasRowMajor   |     lda must be at least $\max(1, k)$.      |  lda must be at least $\max(1, m)$.         |
    ///
    /// * `b` -
    ///
    /// | $B$       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |     Array, size $ldb*n$.<br> Before entry, the leading $k$-by-$n$ part of the array $b$ must contain the matrix $B$.      |    Array, size $ldb*k$. <br> Before entry, the leading $n$-by-$k$ part of the array $b$ must contain the matrix $B$.       |
    /// | `layout` = CBlasRowMajor   |     Array, size $ldb*k$.<br> Before entry, the leading $n$-by-$k$ part of the array $b$ must contain the matrix $B$.        |  Array, size $ldb*n$.<br> Before entry, the leading $k$-by-$n$ part of the array $b$ must contain the matrix $B$.          |
    ///
    /// * `ldb` - Specifies the leading dimension of b as declared in the calling (sub)program.
    ///
    /// | ldb       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |    ldb must be at least $\max(1, k)$.     |    ldb must be at least $\max(1, n)$.      |
    /// | `layout` = CBlasRowMajor   |     ldb must be at least $\max(1, n)$.      |  ldb must be at least $\max(1, k)$.         |
    ///
    /// * `beta` - Specifies the scalar beta. When beta is equal to zero, then c need not be set on input.
    /// * `c` -
    ///
    /// | $C$       |       |
    /// | :------:     | :------------: |
    /// | `layout` = CBlasColMajor  |    Array, size $ldc*n$. Before entry, the leading $m$-by-$n$ part of the array $c$ must contain the matrix $C$, except when beta is equal to zero, in which case $c$ need not be set on entry.      |  
    /// | `layout` = CBlasRowMajor   |     Array, size $ldc*m$. Before entry, the leading $n$-by-$m$ part of the array $c$ must contain the matrix $C$, except when beta is equal to zero, in which case $c$ need not be set on entry.        |
    ///
    /// * `ldc` - Specifies the leading dimension of c as declared in the calling (sub)program.
    ///
    /// | ldc       |       |
    /// | :------:     | :------------: |
    /// | `layout` = CBlasColMajor  |   ldc must be at least $\max(1, m)$.     |  
    /// | `layout` = CBlasRowMajor   |     ldc must be at least $\max(1, n)$.       |
    ///
    /// # Output
    /// * `c` - Overwritten by the $m$-by-$n$ matrix
    pub fn cblas_dgemm(
        layout: CBlasLayout,
        transa: CBlasTranspose,
        transb: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        k: CBlasInt,
        alpha: f64,
        a: *const f64,
        lda: CBlasInt,
        b: *const f64,
        ldb: CBlasInt,
        beta: f64,
        c: *mut f64,
        ldc: CBlasInt,
    );

        /// The ?gemm routines compute a scalar-matrix-matrix product and add the result to a scalar-matrix product, with general matrices. The operation is defined as
    ///
    /// $$C :=  \alpha * \mathrm{op}(A) * \mathrm{op}(B) + \beta * C$$
    ///
    /// where
    ///
    /// $\mathrm{op}(X)$ is one of $\mathrm{op}(X) = X$, or $\mathrm{op}(X)=X^{\top}$, or $\mathrm{op}(X)=X^{\mathrm{H}}$
    ///
    /// $\alpha$ and $\beta$ are scalars,
    ///
    /// $A,$ $B$ and $C$ are matrices:
    ///
    /// $\mathrm{op}(A)$ is an $m$-by-$k$ matrix,
    ///
    /// $\mathrm{op}(B)$ is a $k$-by-$n$ matrix,
    ///
    /// $C$ is an $m$-by-$n$ matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `transa` - Specifies the form of $\mathrm{op}(A)$ used in the matrix multiplication:
    ///     * if `transa`=CblasNoTrans, then $\mathrm{op}(A) = A$;
    ///     * if transa=CblasTrans, then $\mathrm{op}(A) = A^{\top}$;
    ///     * if transa=CblasConjTrans, then $\mathrm{op}(A) = A^{\mathrm{H}}$.
    /// * `transb` - Specifies the form of $\mathrm{op}(B)$ used in the matrix multiplication.
    /// * `m` - Specifies the number of rows of the matrix $\mathrm{op}(A)$ and of the matrix $C$. The value of $m$ must be at least zero.
    /// * `n` - Specifies the number of columns of the matrix $\mathrm{op}(B)$ and the number of columns of the matrix $C$. The value of $n$ must be at least zero.
    /// * `k` - Specifies the number of columns of the matrix $\mathrm{op}(A)$ and the number of rows of the matrix $\mathrm{op}(B)$. The value of $k$ must be at least zero.
    /// * `alpha` - Specifies the scalar alpha.
    /// * `a` -
    ///
    /// | $A$       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |     Array, size $lda*k$.<br> Before entry, the leading $k$-by-$n$ part of the array $a$ must contain the matrix $A$.      |    Array, size $lda*m$. <br> Before entry, the leading $k$-by-$m$ part of the array $a$ must contain the matrix $A$.       |
    /// | `layout` = CBlasRowMajor   |     Array, size $lda*m$.<br> Before entry, the leading $k$-by-$m$ part of the array $a$ must contain the matrix $A$.        |  Array, size $lda*k$.<br> Before entry, the leading $m$-by-$k$ part of the array $a$ must contain the matrix $A$.          |
    ///
    /// * `lda` - Specifies the leading dimension of a as declared in the calling (sub)program.
    ///
    /// | lda       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |    lda must be at least $\max(1, m)$.     |    lda must be at least $\max(1, k)$.      |
    /// | `layout` = CBlasRowMajor   |     lda must be at least $\max(1, k)$.      |  lda must be at least $\max(1, m)$.         |
    ///
    /// * `b` -
    ///
    /// | $B$       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |     Array, size $ldb*n$.<br> Before entry, the leading $k$-by-$n$ part of the array $b$ must contain the matrix $B$.      |    Array, size $ldb*k$. <br> Before entry, the leading $n$-by-$k$ part of the array $b$ must contain the matrix $B$.       |
    /// | `layout` = CBlasRowMajor   |     Array, size $ldb*k$.<br> Before entry, the leading $n$-by-$k$ part of the array $b$ must contain the matrix $B$.        |  Array, size $ldb*n$.<br> Before entry, the leading $k$-by-$n$ part of the array $b$ must contain the matrix $B$.          |
    ///
    /// * `ldb` - Specifies the leading dimension of b as declared in the calling (sub)program.
    ///
    /// | ldb       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |    ldb must be at least $\max(1, k)$.     |    ldb must be at least $\max(1, n)$.      |
    /// | `layout` = CBlasRowMajor   |     ldb must be at least $\max(1, n)$.      |  ldb must be at least $\max(1, k)$.         |
    ///
    /// * `beta` - Specifies the scalar beta. When beta is equal to zero, then c need not be set on input.
    /// * `c` -
    ///
    /// | $C$       |       |
    /// | :------:     | :------------: |
    /// | `layout` = CBlasColMajor  |    Array, size $ldc*n$. Before entry, the leading $m$-by-$n$ part of the array $c$ must contain the matrix $C$, except when beta is equal to zero, in which case $c$ need not be set on entry.      |  
    /// | `layout` = CBlasRowMajor   |     Array, size $ldc*m$. Before entry, the leading $n$-by-$m$ part of the array $c$ must contain the matrix $C$, except when beta is equal to zero, in which case $c$ need not be set on entry.        |
    ///
    /// * `ldc` - Specifies the leading dimension of c as declared in the calling (sub)program.
    ///
    /// | ldc       |       |
    /// | :------:     | :------------: |
    /// | `layout` = CBlasColMajor  |   ldc must be at least $\max(1, m)$.     |  
    /// | `layout` = CBlasRowMajor   |     ldc must be at least $\max(1, n)$.       |
    ///
    /// # Output
    /// * `c` - Overwritten by the $m$-by-$n$ matrix
    pub fn cblas_cgemm(
        layout: CBlasLayout,
        transa: CBlasTranspose,
        transb: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        k: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

        /// The ?gemm routines compute a scalar-matrix-matrix product and add the result to a scalar-matrix product, with general matrices. The operation is defined as
    ///
    /// $$C :=  \alpha * \mathrm{op}(A) * \mathrm{op}(B) + \beta * C$$
    ///
    /// where
    ///
    /// $\mathrm{op}(X)$ is one of $\mathrm{op}(X) = X$, or $\mathrm{op}(X)=X^{\top}$, or $\mathrm{op}(X)=X^{\mathrm{H}}$
    ///
    /// $\alpha$ and $\beta$ are scalars,
    ///
    /// $A,$ $B$ and $C$ are matrices:
    ///
    /// $\mathrm{op}(A)$ is an $m$-by-$k$ matrix,
    ///
    /// $\mathrm{op}(B)$ is a $k$-by-$n$ matrix,
    ///
    /// $C$ is an $m$-by-$n$ matrix.
    ///
    /// # Arguments
    /// * `layout` - Specifies whether two-dimensional array storage is row-major (CblasRowMajor) or column-major (CblasColMajor).
    /// * `transa` - Specifies the form of $\mathrm{op}(A)$ used in the matrix multiplication:
    ///     * if `transa`=CblasNoTrans, then $\mathrm{op}(A) = A$;
    ///     * if transa=CblasTrans, then $\mathrm{op}(A) = A^{\top}$;
    ///     * if transa=CblasConjTrans, then $\mathrm{op}(A) = A^{\mathrm{H}}$.
    /// * `transb` - Specifies the form of $\mathrm{op}(B)$ used in the matrix multiplication.
    /// * `m` - Specifies the number of rows of the matrix $\mathrm{op}(A)$ and of the matrix $C$. The value of $m$ must be at least zero.
    /// * `n` - Specifies the number of columns of the matrix $\mathrm{op}(B)$ and the number of columns of the matrix $C$. The value of $n$ must be at least zero.
    /// * `k` - Specifies the number of columns of the matrix $\mathrm{op}(A)$ and the number of rows of the matrix $\mathrm{op}(B)$. The value of $k$ must be at least zero.
    /// * `alpha` - Specifies the scalar alpha.
    /// * `a` -
    ///
    /// | $A$       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |     Array, size $lda*k$.<br> Before entry, the leading $k$-by-$n$ part of the array $a$ must contain the matrix $A$.      |    Array, size $lda*m$. <br> Before entry, the leading $k$-by-$m$ part of the array $a$ must contain the matrix $A$.       |
    /// | `layout` = CBlasRowMajor   |     Array, size $lda*m$.<br> Before entry, the leading $k$-by-$m$ part of the array $a$ must contain the matrix $A$.        |  Array, size $lda*k$.<br> Before entry, the leading $m$-by-$k$ part of the array $a$ must contain the matrix $A$.          |
    ///
    /// * `lda` - Specifies the leading dimension of a as declared in the calling (sub)program.
    ///
    /// | lda       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |    lda must be at least $\max(1, m)$.     |    lda must be at least $\max(1, k)$.      |
    /// | `layout` = CBlasRowMajor   |     lda must be at least $\max(1, k)$.      |  lda must be at least $\max(1, m)$.         |
    ///
    /// * `b` -
    ///
    /// | $B$       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |     Array, size $ldb*n$.<br> Before entry, the leading $k$-by-$n$ part of the array $b$ must contain the matrix $B$.      |    Array, size $ldb*k$. <br> Before entry, the leading $n$-by-$k$ part of the array $b$ must contain the matrix $B$.       |
    /// | `layout` = CBlasRowMajor   |     Array, size $ldb*k$.<br> Before entry, the leading $n$-by-$k$ part of the array $b$ must contain the matrix $B$.        |  Array, size $ldb*n$.<br> Before entry, the leading $k$-by-$n$ part of the array $b$ must contain the matrix $B$.          |
    ///
    /// * `ldb` - Specifies the leading dimension of b as declared in the calling (sub)program.
    ///
    /// | ldb       | `transa`=CBlasNoTrans      | `transa`=CBlasTrans or `transa`=CBlasConjTrans     |
    /// | :------:     | :------------: | :----------: |
    /// | `layout` = CBlasColMajor  |    ldb must be at least $\max(1, k)$.     |    ldb must be at least $\max(1, n)$.      |
    /// | `layout` = CBlasRowMajor   |     ldb must be at least $\max(1, n)$.      |  ldb must be at least $\max(1, k)$.         |
    ///
    /// * `beta` - Specifies the scalar beta. When beta is equal to zero, then c need not be set on input.
    /// * `c` -
    ///
    /// | $C$       |       |
    /// | :------:     | :------------: |
    /// | `layout` = CBlasColMajor  |    Array, size $ldc*n$. Before entry, the leading $m$-by-$n$ part of the array $c$ must contain the matrix $C$, except when beta is equal to zero, in which case $c$ need not be set on entry.      |  
    /// | `layout` = CBlasRowMajor   |     Array, size $ldc*m$. Before entry, the leading $n$-by-$m$ part of the array $c$ must contain the matrix $C$, except when beta is equal to zero, in which case $c$ need not be set on entry.        |
    ///
    /// * `ldc` - Specifies the leading dimension of c as declared in the calling (sub)program.
    ///
    /// | ldc       |       |
    /// | :------:     | :------------: |
    /// | `layout` = CBlasColMajor  |   ldc must be at least $\max(1, m)$.     |  
    /// | `layout` = CBlasRowMajor   |     ldc must be at least $\max(1, n)$.       |
    ///
    /// # Output
    /// * `c` - Overwritten by the $m$-by-$n$ matrix
    pub fn cblas_zgemm(
        layout: CBlasLayout,
        transa: CBlasTranspose,
        transb: CBlasTranspose,
        m: CBlasInt,
        n: CBlasInt,
        k: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_chemm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_zhemm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_cherk(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: f32,
        a: *const CBlasVoid,
        lda: CBlasInt,
        beta: f32,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_zherk(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: f64,
        a: *const CBlasVoid,
        lda: CBlasInt,
        beta: f64,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_cher2k(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: f32,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_zher2k(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: f64,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_ssymm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        m: CBlasInt,
        n: CBlasInt,
        alpha: f32,
        a: *const f32,
        lda: CBlasInt,
        b: *const f32,
        ldb: CBlasInt,
        beta: f32,
        c: *mut f32,
        ldc: CBlasInt,
    );

    pub fn cblas_dsymm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        m: CBlasInt,
        n: CBlasInt,
        alpha: f64,
        a: *const f64,
        lda: CBlasInt,
        b: *const f64,
        ldb: CBlasInt,
        beta: f64,
        c: *mut f64,
        ldc: CBlasInt,
    );

    pub fn cblas_csymm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_zsymm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_ssyrk(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: f32,
        a: *const f32,
        lda: CBlasInt,
        beta: f32,
        c: *mut f32,
        ldc: CBlasInt,
    );

    pub fn cblas_dsyrk(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: f64,
        a: *const f64,
        lda: CBlasInt,
        beta: f64,
        c: *mut f64,
        ldc: CBlasInt,
    );

    pub fn cblas_csyrk(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_zsyrk(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_ssyr2k(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: f32,
        a: *const f32,
        lda: CBlasInt,
        b: *const f32,
        ldb: CBlasInt,
        beta: f32,
        c: *mut f32,
        ldc: CBlasInt,
    );

    pub fn cblas_dsyr2k(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: f64,
        a: *const f64,
        lda: CBlasInt,
        b: *const f64,
        ldb: CBlasInt,
        beta: f64,
        c: *mut f64,
        ldc: CBlasInt,
    );

    pub fn cblas_csyr2k(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_zsyr2k(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        n: CBlasInt,
        k: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_strmm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: f32,
        a: *const f32,
        lda: CBlasInt,
        b: *mut f32,
        ldb: CBlasInt,
    );

    pub fn cblas_dtrmm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: f64,
        a: *const f64,
        lda: CBlasInt,
        b: *mut f64,
        ldb: CBlasInt,
    );

    pub fn cblas_ctrmm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *mut CBlasVoid,
        ldb: CBlasInt,
    );

    pub fn cblas_ztrmm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *mut CBlasVoid,
        ldb: CBlasInt,
    );

    pub fn cblas_strsm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: f32,
        a: *const f32,
        lda: CBlasInt,
        b: *mut f32,
        ldb: CBlasInt,
    );

    pub fn cblas_dtrsm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: f64,
        a: *const f64,
        lda: CBlasInt,
        b: *mut f64,
        ldb: CBlasInt,
    );

    pub fn cblas_ctrsm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *mut CBlasVoid,
        ldb: CBlasInt,
    );

    pub fn cblas_ztrsm(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *mut CBlasVoid,
        ldb: CBlasInt,
    );

    pub fn cblas_strmm_oop(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: f32,
        a: *const f32,
        lda: CBlasInt,
        b: *const f32,
        ldb: CBlasInt,
        beta: f32,
        c: *mut f32,
        ldc: CBlasInt,
    );

    pub fn cblas_dtrmm_oop(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: f64,
        a: *const f64,
        lda: CBlasInt,
        b: *const f64,
        ldb: CBlasInt,
        beta: f64,
        c: *mut f64,
        ldc: CBlasInt,
    );

    pub fn cblas_ctrmm_oop(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_ztrmm_oop(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_strsm_oop(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: f32,
        a: *const f32,
        lda: CBlasInt,
        b: *const f32,
        ldb: CBlasInt,
        beta: f32,
        c: *mut f32,
        ldc: CBlasInt,
    );

    pub fn cblas_dtrsm_oop(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: f64,
        a: *const f64,
        lda: CBlasInt,
        b: *const f64,
        ldb: CBlasInt,
        beta: f64,
        c: *mut f64,
        ldc: CBlasInt,
    );

    pub fn cblas_ctrsm_oop(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

    pub fn cblas_ztrsm_oop(
        layout: CBlasLayout,
        side: CblasSide,
        uplo: CBlasUplo,
        transa: CBlasTranspose,
        diag: CblasSide,
        m: CBlasInt,
        n: CBlasInt,
        alpha: *const CBlasVoid,
        a: *const CBlasVoid,
        lda: CBlasInt,
        b: *const CBlasVoid,
        ldb: CBlasInt,
        beta: *const CBlasVoid,
        c: *mut CBlasVoid,
        ldc: CBlasInt,
    );

}
