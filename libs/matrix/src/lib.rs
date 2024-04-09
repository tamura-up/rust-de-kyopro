#![allow(non_snake_case)]
/// 2次元の行列積 Ax を求めます
/// # Example
///
/// ```
/// use kyopro_matrix::prod_matrix;
/// let add = |x: usize, y: usize| x + y;
/// let prod = |x: usize, y: usize| x * y;
/// let A = vec![vec![2, 5],
///              vec![4, 7]];
/// let x = vec![vec![1, 3],
///              vec![6, 9]];
/// assert_eq!(prod_matrix(&A, &x, &add, &prod),
///     vec![vec![32, 51],
///          vec![46, 75]]);
/// ```
pub fn prod_matrix<T, MatAdd, MatProd>(A: &Vec<Vec<T>>, x: &Vec<Vec<T>>, add: MatAdd, prod: MatProd) -> Vec<Vec<T>>
where
    T: Copy + Default,
    MatAdd: Fn(T, T) -> T,
    MatProd: Fn(T, T) -> T,
{
    assert_eq!(A[0].len(), x.len());

    let mut res = vec![vec![T::default(); x[0].len()]; A.len()];
    let row = A.len();
    let col = x[0].len();
    for i in 0..row {
        for j in 0..col {
            let mut tot = T::default();
            for k in 0..A[0].len() {
                tot = add(tot, prod(A[i][k], x[k][j]));
            }
            res[i][j] = tot;
        }
    }
    res
}
#[test]
#[rustfmt::skip]
fn test_prod_matrix() {
    let add = |x: usize, y: usize| x + y;
    let prod = |x: usize, y: usize| x * y;

    let a = vec![vec![5],
                 vec![2]];
    let b = vec![vec![1, 5, 9]];
    assert_eq!(prod_matrix(&a, &b, &add, &prod),
        vec![vec![5, 25, 45],
             vec![2, 10, 18]]
    );

    let a = vec![vec![2, 5],
                 vec![4, 7]];
    let b = vec![vec![1, 3],
                 vec![6, 9]];
    assert_eq!(prod_matrix(&a, &b, &add, &prod),
        vec![vec![32, 51],
             vec![46, 75]]);
}

/// 行列累乗を計算します
///
/// # Example
///
/// ```
/// use kyopro_matrix::pow_matrix;
/// let add = |x: usize, y: usize| x + y;
/// let prod = |x: usize, y: usize| x * y;
///
/// let A = vec![vec![2, 1],
///              vec![1, 0]];
/// let E = vec![vec![1, 0],
///              vec![0, 1]];
/// assert_eq!(pow_matrix(2, &A, &E, &add, &prod), vec![vec![5, 2],
///                                                     vec![2, 1]]);
/// ```
pub fn pow_matrix<T, MatAdd, MatProd>(
    mut n: usize,
    A: &Vec<Vec<T>>,
    E: &Vec<Vec<T>>,
    add: MatAdd,
    prod: MatProd,
) -> Vec<Vec<T>>
where
    T: Copy + Default,
    MatAdd: Fn(T, T) -> T,
    MatProd: Fn(T, T) -> T,
{
    assert_eq!(A.len(), A[0].len());
    let mut a = A.clone();
    let mut mat0 = E.clone();
    while n > 0 {
        if n % 2 == 1 {
            mat0 = prod_matrix(&a, &mat0, &add, &prod);
        }
        a = prod_matrix(&a, &a, &add, &prod);
        n /= 2;
    }
    mat0
}
#[test]
#[rustfmt::skip]
fn test_pow_matrix() {
    let add = |x: usize, y: usize| x + y;
    let prod = |x: usize, y: usize| x * y;

    let A = vec![vec![2, 1],
                 vec![1, 0]];
    let E = vec![vec![1, 0],
                 vec![0, 1]];
    assert_eq!(pow_matrix(0, &A, &E, &add, &prod), E);
    assert_eq!(pow_matrix(1, &A, &E, &add, &prod), A);
    assert_eq!(pow_matrix(2, &A, &E, &add, &prod), vec![vec![5, 2],
                                                        vec![2, 1]]);
    assert_eq!(pow_matrix(3, &A, &E, &add, &prod), vec![vec![12, 5],
                                                        vec![5, 2]]);
    assert_eq!(pow_matrix(4, &A, &E, &add, &prod), vec![vec![29, 12],
                                                        vec![12, 5]]);
}
