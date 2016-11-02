//! Compute eigenvalues and eigenvectors of symmetric matrices.

use lapack::c::{ssyev, dsyev, cheev, zheev};
use super::types::{Solution, EigenError};
use impl_prelude::*;

pub trait SymEigen: Sized {
    type SingularValue;
    type Solution;

    /// Return the real eigenvalues of a symmetric matrix.
    ///
    /// The input matrix is mutated as part of the computation. Use
    /// [`.values()`](#tymethod.values) if you need to preserve the original
    /// matrix.
    fn compute_mut<D>(mat: &mut ArrayBase<D, Ix2>,
                      uplo: Symmetric,
                      with_vectors: bool)
                      -> Result<Array<Self::SingularValue, Ix>, EigenError>
        where D: DataMut<Elem = Self>;

    /// Return the eigenvalues of a symmetric matrix.
    ///
    /// # Remarks
    ///
    /// The input matrix is copied before the calculation takes place.
    fn compute<D>(mat: &ArrayBase<D, Ix2>,
                  uplo: Symmetric,
                  with_vectors: bool)
                  -> Result<Self::Solution, EigenError>
        where D: Data<Elem = Self>;
}

macro_rules! impl_sym_eigen {
    ($impl_type:ident, $eigen_type:ident, $func:ident) => (
        impl SymEigen for $impl_type {
            type SingularValue = $eigen_type;
            type Solution = Solution<Self, Self::SingularValue>;

            fn compute_mut<D>(mat: &mut ArrayBase<D, Ix2>, uplo: Symmetric, with_vectors: bool) ->
                Result<Array<Self::SingularValue, Ix>, EigenError> where D: DataMut<Elem=Self>
            {
                let dim = mat.dim();
                if dim.0 != dim.1 {
                    return Err(EigenError::NotSquare);
                }

                let n = dim.0 as i32;

                let (mut data_slice, layout, ld) = match slice_and_layout_mut(mat) {
                    Some(x) => x,
                    None => return Err(EigenError::BadLayout)
                };

                let mut values = Array::default(n as Ix);
                let job = if with_vectors { b'V' } else { b'N' };

                let info = $func(layout, job, uplo as u8, n, data_slice,
                                 ld as i32, values.as_slice_mut().unwrap());

                if info  == 0 {
                    Ok(values)
                } else if info < 0 {
                    Err(EigenError::BadParameter(-info))
                } else {
                    Err(EigenError::Failed)
                }
            }


            fn compute<D>(mat: &ArrayBase<D, Ix2>,
                          uplo: Symmetric,
                          with_vectors: bool) -> Result<Self::Solution, EigenError>
                where D: Data<Elem=Self> {
                let vec: Vec<Self> = mat.iter().cloned().collect();
                let mut new_mat = Array::from_shape_vec(mat.dim(), vec).unwrap();
                Self::compute_mut(&mut new_mat, uplo, with_vectors).map(|values| {
                    Solution {
                        values: values,
                        left_vectors: None,
                        right_vectors: if with_vectors { Some(new_mat) } else { None }
                    }
                })
            }
        }
    )
}

impl_sym_eigen!(f32, f32, ssyev);
impl_sym_eigen!(c32, f32, cheev);
impl_sym_eigen!(f64, f64, dsyev);
impl_sym_eigen!(c64, f64, zheev);

#[cfg(test)]
mod tests {}
