use std::cmp::{max, min};

/// LowLink
/// 関節点, 橋 の検出 を O(V+E) で行う
/// 参考:https://algo-logic.info/articulation-points/
/// **※注意※  多重辺がある場合を想定していないので事前に除くこと**
///
/// ### 使い方
/// ```
/// // https://algo-logic.info/articulation-points/ の冒頭の図の木
/// let mut ll = kyopro_graph::low_link::LowLink::new(5);
/// ll.add_edge2(0, 1);
/// ll.add_edge2(1, 2);
/// ll.add_edge2(0, 2);
/// ll.add_edge2(0, 3);
/// ll.add_edge2(3, 4);
///
/// // 探索の実行
/// ll.run();
/// // 関節点のリストを取得
/// assert_eq!(ll.aps, vec![0, 3]);
/// // 橋の頂点ペアを取得
/// assert_eq!(ll.bridges, vec![(0, 3), (3, 4)]);
/// ```

pub struct LowLink {
    n: usize,
    g: Vec<Vec<usize>>,
    pub ord: Vec<u32>,                // DFS遷移順
    pub low: Vec<u32>,                // 各頂点から(DFS の辺 +)後退辺 1 つで辿れる頂点のうち、最小の ord
    pub aps: Vec<usize>,              // 関節点の頂点リスト
    pub bridges: Vec<(usize, usize)>, // 橋の端点ペアのリスト
}

impl LowLink {
    pub fn new(n: usize) -> Self {
        let g = vec![vec![]; n];
        Self { n, g, ord: vec![], low: vec![], aps: vec![], bridges: vec![] }
    }

    /// u -> v の辺を追加します
    pub fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.n);
        assert!(v < self.n);
        self.g[u].push(v);
    }
    /// u -> v, v -> u の 2 辺を追加します
    pub fn add_edge2(&mut self, u: usize, v: usize) {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }
    fn dfs(&mut self, id: usize, mut k: u32, p: usize) -> u32 {
        k += 1;
        self.ord[id] = k;
        self.low[id] = k;
        let mut is_aps = false;
        let mut ch_cnt = 0;
        for vi in 0..self.g[id].len() {
            let v = self.g[id][vi];
            if v == p {
                continue;
            }
            if self.ord[v] == !0 {
                // 未到達頂点
                ch_cnt += 1;
                k = self.dfs(v, k, id);
                // 子から後退辺1つで辿れる頂点の最小値で更新
                self.low[id] = min(self.low[id], self.low[v]);

                // id が root でないかつ、子が id 以下の頂点へ後退辺を持たない場合は関節点
                if p != !0 && self.ord[id] <= self.low[v] {
                    is_aps = true;
                }
                // 橋
                if self.ord[id] < self.low[v] {
                    self.bridges.push((min(id, v), max(id, v)));
                }
            } else {
                self.low[id] = min(self.low[id], self.ord[v]);
            }
        }
        if p == !0 && ch_cnt >= 2 {
            is_aps = true;
        }
        if is_aps {
            self.aps.push(id);
        }
        k
    }
    /// 関節点検出の実行
    pub fn run(&mut self) {
        self.aps = vec![];
        self.ord = vec![!0; self.n];
        self.low = vec![!0; self.n];
        self.bridges = vec![];
        let mut k = 0;
        for i in 0..self.n {
            if self.ord[i] == !0 {
                k = self.dfs(i, k, !0);
            }
        }
        self.aps.sort();
        self.bridges.sort();
    }
}
#[test]
fn test_lowlink_multiple_component() {
    let mut ll = LowLink::new(7);
    ll.add_edge2(0, 1);
    ll.add_edge2(1, 2);

    ll.add_edge2(3, 4);
    ll.add_edge2(4, 5);
    ll.add_edge2(5, 6);
    ll.add_edge2(5, 3);

    ll.run();
    // 関節点
    assert_eq!(ll.aps, vec![1, 5]);
    // 橋
    assert_eq!(ll.bridges, vec![(0, 1), (1, 2), (5, 6)]);
}
