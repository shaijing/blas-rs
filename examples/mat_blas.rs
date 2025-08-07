use blas_rs::cblas::prelude::*;

#[inline(always)]
pub fn sdot(x: &[f32], incx: i64, y: &[f32], incy: i64) -> f32 {
    unsafe {
        cblas_sdot(
            x.len() as CBlasInt,
            x.as_ptr(),
            incx as CBlasInt,
            y.as_ptr(),
            incy as CBlasInt,
        )
    }
}

#[inline(always)]
pub fn ddot(x: &[f64], incx: i64, y: &[f64], incy: i64) -> f64 {
    unsafe {
        cblas_ddot(
            x.len() as CBlasInt,
            x.as_ptr(),
            incx as CBlasInt,
            y.as_ptr(),
            incy as CBlasInt,
        )
    }
}

#[inline(always)]
pub fn dgemm(
    layout: CBlasLayout,
    transa: CBlasTranspose,
    transb: CBlasTranspose,
    m: i64,
    n: i64,
    k: i64,
    alpha: f64,
    a: &[f64],
    lda: i64,
    b: &[f64],
    ldb: i64,
    beta: f64,
    c: &mut [f64],
    ldc: i64,
) {
    unsafe {
        cblas_dgemm(
            layout,
            transa,
            transb,
            m,
            n,
            k,
            alpha,
            a.as_ptr(),
            lda,
            b.as_ptr(),
            ldb,
            beta,
            c.as_mut_ptr(),
            ldc,
        );
    }
}

//cargo r -r -F intel-mkl --example test_blas
fn main() {
    let size = 4;
    let a = vec![1.0; size * size];
    let b = vec![2.0; size * size];
    let mut c = vec![0.0; size * size];
    let layout = CBlasLayout::CBlasRowMajor;
    let transa = CBlasTranspose::CBlasNoTrans;
    let transb = CBlasTranspose::CBlasNoTrans;
    dgemm(
        layout,
        transa,
        transb,
        size as i64,
        size as i64,
        size as i64,
        1.0,
        &a,
        size as i64,
        &b,
        size as i64,
        1.0,
        &mut c,
        size as i64,
    );
    println!("{c:?}");
}

#[cfg(any(feature = "intel-mkl", feature = "openblas"))]
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_basic_blas_mat() {
        let a = vec![1.0; 9];
        let b = vec![2.0; 9];
        let mut c = vec![0.0; 9];
        let layout = CBlasLayout::CBlasRowMajor;
        let transa = CBlasTranspose::CBlasNoTrans;
        let transb = CBlasTranspose::CBlasNoTrans;
        dgemm(
            layout, transa, transb, 3, 3, 3, 1.0, &a, 3, &b, 3, 1.0, &mut c, 3,
        );
        println!("{:?}", c);
    }
}
