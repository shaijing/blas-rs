use crate::cblas::cblas_types::*;

unsafe extern "C" {
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

    pub fn cblas_chpr(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasFloat,
        x: *const CBlasVoid,
        incx: CBlasInt,
        ap: *mut CBlasVoid,
    );

    pub fn cblas_zhpr(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasDouble,
        x: *const CBlasVoid,
        incx: CBlasInt,
        ap: *mut CBlasVoid,
    );

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

    pub fn cblas_sspr(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasFloat,
        x: *const CBlasFloat,
        incx: CBlasInt,
        ap: *mut CBlasFloat,
    );

    pub fn cblas_dspr(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        n: CBlasInt,
        alpha: CBlasDouble,
        x: *const CBlasDouble,
        incx: CBlasInt,
        ap: *mut CBlasDouble,
    );

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

    pub fn cblas_stbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    pub fn cblas_dtbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    pub fn cblas_ctbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    pub fn cblas_ztbmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    pub fn cblas_stbsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    pub fn cblas_dtbsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    pub fn cblas_ctbsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    pub fn cblas_ztbsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        k: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    pub fn cblas_stpmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        ap: *const CBlasFloat,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    pub fn cblas_dtpmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        ap: *const CBlasDouble,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    pub fn cblas_ctpmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        ap: *const CBlasVoid,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    pub fn cblas_ztpmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        ap: *const CBlasVoid,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    pub fn cblas_stpsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        ap: *const CBlasFloat,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    pub fn cblas_dtpsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        ap: *const CBlasDouble,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    pub fn cblas_ctpsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        ap: *const CBlasVoid,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    pub fn cblas_ztpsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        ap: *const CBlasVoid,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    pub fn cblas_strmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    pub fn cblas_dtrmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    pub fn cblas_ctrmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    pub fn cblas_ztrmv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    pub fn cblas_strsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        a: *const CBlasFloat,
        lda: CBlasInt,
        x: *mut CBlasFloat,
        incx: CBlasInt,
    );

    pub fn cblas_dtrsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        a: *const CBlasDouble,
        lda: CBlasInt,
        x: *mut CBlasDouble,
        incx: CBlasInt,
    );

    pub fn cblas_ctrsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

    pub fn cblas_ztrsv(
        layout: CBlasLayout,
        uplo: CBlasUplo,
        trans: CBlasTranspose,
        diag: CblasDiag,
        n: CBlasInt,
        a: *const CBlasVoid,
        lda: CBlasInt,
        x: *mut CBlasVoid,
        incx: CBlasInt,
    );

}
