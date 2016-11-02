#[macro_use]
extern crate rula;
extern crate ndarray;

use ndarray::prelude::*;
use rula::prelude::*;

#[test]
fn try_eig() {
    let mut m = arr2(&[[1.0 as f32, 2.0], [2.0, 1.0]]);

    let r = SymEigen::compute_mut(&mut m, Symmetric::Upper, false);
    assert!(r.is_ok());

    assert_in_tol!(r.unwrap(), arr1(&[-1.0, 3.0]), 0.01);
}