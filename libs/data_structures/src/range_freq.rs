use kyopro_utils::lower_bound;

/// 数列の区間 [l, r) において、 x 未満の値の要素数を取得する
/// (蟻本 p.170)
pub struct RangeFreq<T> {
    n: usize,
    dat: Vec<Vec<T>>,
}
impl<T: Copy + Ord> RangeFreq<T> {
    /// RangeFreq の構築
    /// ```
    /// use kyopro_data_stractures::range_freq::RangeFreq;
    /// let data = vec![1, 5, 3, 6, 1];
    /// let rf = RangeFreq::new(&data);
    /// assert_eq!(rf.query(1, 5, 3), 1);
    /// assert_eq!(rf.query(1, 5, 5), 2);
    /// ```
    pub fn new(vec: &[T]) -> Self {
        // 木のサイズ
        let mut n = 1;
        while n < vec.len() {
            n <<= 1;
        }
        let mut dat = vec![vec![]; 2 * n - 1];
        vec.iter().enumerate().for_each(|(i, &v)| {
            dat[i + n - 1].push(v);
        });
        for i in (0..=n - 2).rev() {
            // 2つの子をマージ
            dat[i] = Self::merge(&dat[i * 2 + 1], &dat[i * 2 + 2]);
        }
        // for v in &dat{
        //     eprintln!("{:?}",v);
        // }
        Self { n, dat }
    }
    fn merge(l: &[T], r: &[T]) -> Vec<T> {
        let mut vec: Vec<T> = Vec::with_capacity(l.len() + r.len());
        let mut li = 0;
        let mut ri = 0;
        while li < l.len() || ri < r.len() {
            if li >= l.len() || (ri < r.len() && l[li] > r[ri]) {
                vec.push(r[ri]);
                ri += 1;
            } else {
                vec.push(l[li]);
                li += 1;
            }
        }
        vec
    }
    fn _query(&self, a: usize, b: usize, x: T, k: usize, l: usize, r: usize) -> usize {
        if b <= l || r <= a {
            return 0;
        }
        if a <= l && r <= b {
            return lower_bound(&self.dat[k], &x);
        }
        return self._query(a, b, x, k * 2 + 1, l, (l + r) / 2) + self._query(a, b, x, k * 2 + 2, (l + r) / 2, r);
    }
    /// [a, b) から x 未満の要素数を取得
    pub fn query(&self, a: usize, b: usize, x: T) -> usize {
        return self._query(a, b, x, 0, 0, self.n);
    }
}
#[test]
fn test_range_freq() {
    let data = vec![
        1, 5, 3, 6, 1, //
        10, 9, 4, 10, 2,
    ];
    let rf = RangeFreq::new(&data);
    assert_eq!(rf.query(0, 1, 1), 0);
    assert_eq!(rf.query(0, 1, 2), 1);

    assert_eq!(rf.query(0, 4, 3), 1);
    assert_eq!(rf.query(0, 4, 4), 2);
    assert_eq!(rf.query(0, 4, 7), 4);

    assert_eq!(rf.query(5, 8, 9), 1);
    assert_eq!(rf.query(5, 8, 10), 2);
    assert_eq!(rf.query(5, 8, 11), 3);

    assert_eq!(rf.query(8, 10, 2), 0);
    assert_eq!(rf.query(8, 10, 3), 1);
    assert_eq!(rf.query(8, 10, 10), 1);
    assert_eq!(rf.query(8, 10, 11), 2);
}
