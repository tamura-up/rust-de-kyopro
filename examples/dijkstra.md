ダイクストラ法のテンプレート

```rs
fn dijkstra(g: &Vec<Vec<(usize, i64)>>, s: usize) -> Vec<i64> {
    let N = g.len();
    let inf = i64::MAX / 2 - 114514;
    let mut dist = vec![inf; N];
    let mut que = BinaryHeap::new();
    dist[s]=0;
    que.push((Reverse(0), s));
    while let Some((Reverse(co), u)) = que.pop() {
        if dist[u] != co {
            continue;
        }
        g[u].iter().for_each(|&(v, d)| {
            if chmin!(dist[v], co + d) {
                que.push((Reverse(dist[v]), v));
            }
        });
    }
    dist
}
```
