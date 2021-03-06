#[macro_use]
extern crate linxal;

extern crate ndarray;
extern crate num_traits;
extern crate rand;

use ndarray::{Array, ArrayBase, Data, Ix2};
use rand::thread_rng;
use linxal::eigenvalues::SymEigen;
use linxal::factorization::{Cholesky, CholeskyError};
use linxal::types::{Symmetric, c32, c64};
use linxal::properties::is_triangular;
use linxal::generate::{RandomSemiPositive, MG};
use linxal::util::conj_t;

fn check_cholesky<T, D1, D2>(mat: ArrayBase<D1, Ix2>, chol: ArrayBase<D2, Ix2>, uplo: Symmetric)
    where T: Cholesky, D1: Data<Elem=T>, D2: Data<Elem=T> {
    // Check the dimension
    assert_eq!(mat.dim(), chol.dim());

    // The matrix must be triangular
    assert!(is_triangular(&chol, uplo));

    // The fatorization must match the original matrix.
    match uplo {
        Symmetric::Lower => {
            let u = conj_t(&chol);
            assert_eq_within_tol!(chol.dot(&u), mat, 1e-4.into());
        },
        Symmetric::Upper => {
            let l = conj_t(&chol);
            assert_eq_within_tol!(l.dot(&chol), mat, 1e-4.into());
        }
    }
}


fn cholesky_identity_generic<T: Cholesky>() {
    for n in 1..11 {
        let m: Array<T, Ix2> = Array::eye(n);

        let l = Cholesky::compute(&m, Symmetric::Upper).ok().unwrap();
        assert_eq_within_tol!(l, m, 1e-5.into());
    }
}

fn cholesky_generate_generic<T: MG + Cholesky>(uplo: Symmetric) {
    let mut rng = thread_rng();
    for n in 1..11 {
        let m: Array<T, Ix2> = RandomSemiPositive::new(n, &mut rng).generate().unwrap();

        let res = Cholesky::compute(&m, uplo);
        let chol = res.ok().unwrap();
        check_cholesky(m, chol, uplo);
    }
}

fn cholesky_fail_zero_ev<T: MG + Cholesky + SymEigen>() {
    let mut rng = thread_rng();
    for n in 4..11 {
        let mut gen: RandomSemiPositive<T> = RandomSemiPositive::new(n, &mut rng);
        let r = gen.rank(0).generate_with_sv();
        let m = r.ok().unwrap();

        let res = Cholesky::compute(&m.0, Symmetric::Upper);
        assert_eq!(res.err().unwrap(), CholeskyError::NotPositiveDefinite);
    }
}

#[test]
fn cholesky_identity() {
    cholesky_identity_generic::<f32>();
    cholesky_identity_generic::<c32>();
    cholesky_identity_generic::<f64>();
    cholesky_identity_generic::<c64>();
}

#[test]
fn cholesky_generate() {
    cholesky_generate_generic::<f32>(Symmetric::Upper);
    cholesky_generate_generic::<f32>(Symmetric::Lower);

    cholesky_generate_generic::<f64>(Symmetric::Upper);
    cholesky_generate_generic::<f64>(Symmetric::Lower);

    cholesky_generate_generic::<c32>(Symmetric::Upper);
    cholesky_generate_generic::<c32>(Symmetric::Lower);

    cholesky_generate_generic::<c64>(Symmetric::Upper);
    cholesky_generate_generic::<c64>(Symmetric::Lower);
}

#[test]
fn cholesky_zero() {
    cholesky_fail_zero_ev::<f32>();
}

#[test]
fn cholesky_fail_not_square() {
    for r in 1..11 {
        for c in 1..11 {
            if r == c {
                continue;
            }
            let m: Array<f32, Ix2> = Array::linspace(1.0, 2.0, r*c).into_shape((r, c)).unwrap();

            let res = Cholesky::compute(&m, Symmetric::Upper);
            assert_eq!(res.err().unwrap(), CholeskyError::NotSquare);
        }
    }
}
