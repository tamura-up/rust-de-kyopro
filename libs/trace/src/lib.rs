///! 履歴管理データ構造
///! src: https://atcoder.jp/contests/ahc021/submissions/42958368
pub struct Trace<T: Clone> {
    log: Vec<(T, usize)>,
}

impl<T: Clone> Trace<T> {
    /// # Examples
    /// ```
    /// use kyoupuro_trace::Trace;
    ///
    /// let mut trace = Trace::new();
    /// let mut k = trace.add(1, !0);
    /// k = trace.add(2, k);
    /// k = trace.add(3, k);
    /// assert_eq!(trace.get(k), vec![1, 2, 3]);
    /// ```
    pub fn new() -> Self {
        Trace { log: vec![] }
    }
    /// 履歴の追加
    ///
    /// - c: 情報
    /// - p: previous log index. 始点の場合は !0 を指定してください。
    ///
    /// return: log index
    pub fn add(&mut self, c: T, p: usize) -> usize {
        self.log.push((c, p));
        self.log.len() - 1
    }
    /// trace の取得
    /// 指定された log index: i について、 始点から i までの trace を取得します
    pub fn get(&self, mut i: usize) -> Vec<T> {
        let mut out = vec![];
        while i != !0 {
            out.push(self.log[i].0.clone());
            i = self.log[i].1;
        }
        out.reverse();
        out
    }
    pub fn get_tid_list(&self, mut i: usize) -> Vec<usize> {
        let mut out = vec![];
        while i != !0 {
            out.push(i);
            i = self.log[i].1;
        }
        out.reverse();
        out
    }
    pub fn get_last_k(&self, mut i: usize, k: usize)->Vec<T> {
        let mut out = vec![];
        let mut iter = 0;
        while i != !0 {
            iter += 1;
            out.push(self.log[i].0.clone());
            i = self.log[i].1;
            if iter == k {
                break;
            }
        }
        out.reverse();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace() {
        let mut trace = Trace::new();
        let k1 = trace.add(1, !0);
        let k2 = trace.add(2, k1);
        let k3 = trace.add(3, k2);
        assert_eq!(trace.get(k3), vec![1, 2, 3]);

        let k11 = trace.add(11, k2);

        let k12 = trace.add(12, !0);

        assert_eq!(trace.get(k3), vec![1, 2, 3]);
        assert_eq!(trace.get(k11), vec![1, 2, 11]);
        assert_eq!(trace.get(k12), vec![12]);
    }
}
