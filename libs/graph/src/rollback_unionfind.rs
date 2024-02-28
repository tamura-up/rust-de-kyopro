use std::mem;

/// RollbackつきUnion Find
/// Undo 機能や rollback 機能をつけた UnionFind
///
/// 参考: https://nyaannyaan.github.io/library/data-structure/rollback-union-find.hpp
///
/// ### verified
/// + https://judge.yosupo.jp/problem/persistent_unionfind
///   + submit: https://judge.yosupo.jp/submission/194157
pub struct RollbackUnionFind {
    data: Vec<i32>,
    history: Vec<(usize, i32)>,
    inner_snap: usize,
}
impl RollbackUnionFind {
    pub fn new(sz: usize) -> Self {
        Self { data: vec![-1; sz], history: vec![], inner_snap: 0 }
    }
    /// x, y をマージします。
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        self.history.push((x, self.data[x]));
        self.history.push((y, self.data[y]));
        if x == y {
            return false;
        }
        if self.data[x] > self.data[y] {
            mem::swap(&mut x, &mut y);
        }
        self.data[x] += self.data[y];
        self.data[y] = x as i32;
        true
    }

    /// k の属する木の root の探索
    pub fn find(&self, k: usize) -> usize {
        if self.data[k] < 0 {
            return k;
        }
        return self.find(self.data[k] as usize);
    }
    /// x, y が同じ連結成分に属するか？
    /// O(log N)
    pub fn same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    /// x の連結成分のサイズを返す。
    /// O(log N)
    pub fn size(&self, x: usize) -> usize {
        (-self.data[self.find(x)]) as usize
    }
    /// 直前の `unite` の操作を取り消す。
    /// O(1)
    pub fn undo(&mut self) {
        assert!(self.history.len() >= 2);
        let a = self.history.pop().unwrap();
        let b = self.history.pop().unwrap();
        self.data[a.0] = a.1;
        self.data[b.0] = b.1;
    }
    /// 現在状態を保存。
    /// `rollback(None)` で保存状態を復元することができる。
    pub fn snapshot(&mut self) {
        self.inner_snap = self.history.len() >> 1;
    }
    /// 現在の `unite` が呼ばれた回数を返す。
    /// `rollback(x)` で `unite` が x 回呼ばれた状態まで戻すことができる。
    pub fn get_state(&self) -> usize {
        self.history.len() >> 1
    }
    /// UnionFindをロールバックする。
    /// 計算量は状況による。(ボトルネックにはならない）
    ///
    /// + `state = None` のとき：`snapshot` で保存した状態にロールバック。
    /// + そうでないとき：`unite` が `state` 呼び出された時の状態にロールバックする。
    pub fn rollback(&mut self, state: Option<usize>) {
        let state = state.unwrap_or(self.inner_snap)<<1;
        assert!(state <= self.history.len());
        while state < self.history.len() {
            self.undo();
        }
    }

    /// alias for unite
    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        self.unite(x, y)
    }
    /// alias for find
    pub fn leader(&mut self, x: usize) -> usize {
        self.find(x)
    }
}
#[test]
fn test_undo() {
    let mut uf = RollbackUnionFind::new(3);
    assert!(!uf.same(0, 1));
    assert!(!uf.same(0, 2));
    uf.merge(0, 1);
    uf.merge(1, 2);
    assert!(uf.same(0, 1));
    assert!(uf.same(0, 2));
    uf.undo();
    assert!(uf.same(0, 1));
    assert!(!uf.same(0, 2));
    uf.undo();
    assert!(!uf.same(0, 1));
    assert!(!uf.same(0, 2));
}

#[test]
fn test_rollback_using_snapshot() {
    let mut uf = RollbackUnionFind::new(5);
    uf.merge(0, 1);
    uf.merge(1, 2);
    uf.snapshot();
    uf.merge(0, 3);
    uf.merge(3, 4);
    assert!(uf.same(0, 1));
    assert!(uf.same(0, 2));
    assert!(uf.same(0, 3));
    assert!(uf.same(0, 4));
    uf.rollback(None);
    assert!(uf.same(0, 1));
    assert!(uf.same(0, 2));
    assert!(!uf.same(0, 3));
    assert!(!uf.same(0, 4));
}

#[test]
fn test_rollback_using_state() {
    let mut uf = RollbackUnionFind::new(5);
    uf.merge(0, 1);
    uf.merge(1, 2);
    let state = uf.get_state();
    uf.merge(0, 3);
    uf.merge(3, 4);
    assert!(uf.same(0, 3));
    assert!(uf.same(0, 4));
    uf.rollback(Some(state));
    assert!(uf.same(0, 1));
    assert!(uf.same(0, 2));
    assert!(!uf.same(0, 3));
    assert!(!uf.same(0, 4));
}
