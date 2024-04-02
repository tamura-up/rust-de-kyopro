///! 2次元累積和
use std::ops::{Add, AddAssign, Sub};

pub struct Accumulate2D<T> {
    h: usize,
    w: usize,
    a: Vec<Vec<T>>,
    d: Vec<Vec<T>>,
}
impl<T> Accumulate2D<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + AddAssign,
{
    pub fn new(h: usize, w: usize, init: T) -> Self {
        let a = vec![vec![init; w]; h];
        let s = vec![vec![init; w + 1]; h + 1];
        Accumulate2D { h, w, a, d: s }
    }
    pub fn add(&mut self, r: usize, c: usize, v: T) {
        self.a[r][c] += v;
    }
    pub fn set(&mut self, r: usize, c: usize, v: T) {
        self.a[r][c] = v;
    }
    pub fn build(&mut self) {
        for i in 0..self.h {
            for j in 0..self.w {
                self.d[i + 1][j + 1] = self.a[i][j] + self.d[i + 1][j] + self.d[i][j + 1] - self.d[i][j];
            }
        }
    }
    /// 左上座標 (t, l) から 右下座標 (b, r) の範囲の累積和を求めます (右下は半開区間)
    /// 範囲指定順注意 [t, b), [l, r) の順で指定する
    pub fn query(&self, t: usize, b: usize, l: usize, r: usize) -> T {
        self.d[b][r] + self.d[t][l] - self.d[b][l] - self.d[t][r]
    }
}

#[test]
fn test_accumurate() {
    let mut acc = Accumulate2D::new(3, 3, 0i32);
    for i in 0..3 {
        for j in 0..3 {
            acc.set(i, j, (i * 3 + j) as i32);
        }
    }
    acc.build();
    assert_eq!(acc.query(0, 1, 0, 1), 0);
    assert_eq!(acc.query(2, 3, 2, 3), 8);
    assert_eq!(acc.query(0, 3, 0, 3), 36);
    assert_eq!(acc.query(0, 1, 0, 3), 3);
    assert_eq!(acc.query(1, 3, 1, 3), 24);
}
