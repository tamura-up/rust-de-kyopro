//! LCA です。頂点間の最近共通祖先を求めます。
//!
//! ## Example
//!
//! ```ignore
//! // LCA インスタンスを作成して、辺を追加
//! let mut lca = LCA::new(N);
//! for (u, v) in ed {
//!     lca.add_edge(u, v);
//!     lca.add_edge(v, u);
//! }
//!
//! // 辺を追加後に初期化します
//! lca.init();
//! // lca を求めます
//! lca.lca(0, N - 1);
//! ```
#![allow(non_snake_case)]
use std::mem;

pub struct LCA {
    g: Vec<Vec<usize>>,
    depth: Vec<usize>,
    tour_v: Vec<usize>,
    v_to_i: Vec<usize>,
    tour_depth: Vec<usize>,
    sparse_table: Vec<Vec<usize>>,
}

/// v を表すために必要な bit size を返します
fn bitsize(mut v: usize) -> usize {
    let mut res = 0;
    while v > 0 {
        res += 1;
        v >>= 1;
    }
    res
}
impl LCA {
    /// インスタンスを作成
    ///
    /// n: 頂点数
    pub fn new(n: usize) -> Self {
        Self {
            g: vec![vec![]; n],
            depth: vec![],
            tour_v: vec![],
            v_to_i: vec![],
            tour_depth: vec![],
            sparse_table: vec![],
        }
    }

    /// u から v の辺を貼ります
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
    }

    /// LCA を構築します
    pub fn init(&mut self) {
        self.v_to_i = vec![!0; self.g.len()];
        self.depth = vec![!0; self.g.len()];
        self.dfs(0, !0, 0);
        self.tour_depth = self.tour_v.iter().map(|&v| self.depth[v]).collect();

        let N = self.tour_v.len();
        let U = bitsize(N);
        // sparse table
        // 区間最小の深さをもつ頂点 index を記録する
        let mut sp = vec![vec![]; 1];
        for i in 0..N {
            sp[0].push(i);
        }
        for i in 1..U {
            let prev = &sp[i - 1];
            let width = 1 << (i - 1);
            let next_size = prev.len() - width;

            let mut tmp = vec![!0; next_size];
            for j in 0..next_size {
                let n1 = prev[j];
                let n2 = prev[j + width];
                let n1d = self.tour_depth[n1];
                let n2d = self.tour_depth[n2];
                // 小さい深さの頂点 index を記録
                tmp[j] = if n1d < n2d { n1 } else { n2 };
            }
            sp.push(tmp);
        }
        self.sparse_table = sp;
    }

    // build euler tour
    fn dfs(&mut self, u: usize, p: usize, d: usize) {
        self.v_to_i[u] = self.tour_v.len();
        self.depth[u] = d;
        self.tour_v.push(u);
        let nodes = self.g[u].clone();
        for v in nodes {
            if v == p {
                continue;
            }
            self.dfs(v, u, d + 1);
            self.tour_v.push(u);
        }
    }

    /// 頂点 u, v の LCA を求めます
    pub fn lca(&self, u: usize, v: usize) -> usize {
        // 初期化済みチェック
        let initialized = !self.sparse_table.is_empty();
        assert!(initialized);

        if u == v {
            return u;
        }
        let mut ix = self.v_to_i[u];
        let mut iy = self.v_to_i[v];
        if ix > iy {
            mem::swap(&mut ix, &mut iy);
        }
        // オイラーツアー上の区間最小の深さをもつ頂点が LCA
        let items = iy - ix + 1;
        let L = bitsize(items) - 1;
        let p1 = self.sparse_table[L][ix];
        let p2 = self.sparse_table[L][1 + iy - (1 << L)];

        let idx = if self.tour_depth[p1] < self.tour_depth[p2] { p1 } else { p2 };
        self.tour_v[idx]
    }
    /// 頂点 u, v の距離を求めます
    pub fn dist(&self, u: usize, v: usize) -> usize {
        if u == v {
            return 0;
        }
        let p = self.lca(u, v);
        self.depth[u] + self.depth[v] - self.depth[p] * 2
    }
}

#[test]
fn test_tour1() {
    let mut lca = LCA::new(3);
    lca.add_edge(0, 1);
    lca.add_edge(1, 0);
    lca.add_edge(0, 2);
    lca.add_edge(2, 0);
    lca.init();
    assert_eq!(lca.lca(0, 0), 0);
    assert_eq!(lca.lca(1, 0), 0);
    assert_eq!(lca.lca(1, 1), 1);
    assert_eq!(lca.lca(1, 2), 0);
}

#[test]
fn test_tour2() {
    let mut lca = LCA::new(6);
    lca.add_edge(0, 1);
    lca.add_edge(1, 0);
    lca.add_edge(0, 2);
    lca.add_edge(2, 0);
    lca.add_edge(1, 3);
    lca.add_edge(3, 1);
    lca.add_edge(1, 3);
    lca.add_edge(1, 4);
    lca.add_edge(4, 1);
    lca.add_edge(2, 5);
    lca.add_edge(5, 2);
    lca.init();
    assert_eq!(lca.lca(1, 2), 0);
    assert_eq!(lca.lca(1, 0), 0);
    assert_eq!(lca.lca(1, 3), 1);
    assert_eq!(lca.lca(3, 4), 1);
    assert_eq!(lca.lca(3, 5), 0);
    assert_eq!(lca.lca(2, 5), 2);
}
