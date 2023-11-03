## 公式実装例

https://github.com/rust-lang-ja/ac-library-rs/blob/master/examples/practice2_j_segment_tree.rs

## Monoid

頻出の min, max, 加算などは公式に実装されている
https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/segtree.rs

#### 自作構造体を使った実装例

acl の segment tree を使って、モノイドを成す構造体を定義する(?)実装例 

```rs
use ac_library::{Monoid, Segtree};
use std::cmp::min;

#[derive(Debug, Clone, Copy)]
struct Data {
    x: i32,
    y: i32,
}
impl Monoid for Data {
    type S = Data;

    fn identity() -> Self::S {
        const inf: i32 = 1_100_100_100;
        Data { x: inf, y: 0 }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        Data {
            x: min(a.x, a.y + b.x),
            y: a.y + b.y,
        }
    }
}

#[test]
fn segtree_exsample() {
    let X = vec![1, 0, -1, -1, 1, 1];
    let N: usize = X.len();

    let mut seg = Segtree::<Data>::new(N);
    // update
    for (i, &v) in X.iter().enumerate() {
        seg.set(i, Data { x: v, y: v });
    }
    // query
    {
        let res1 = seg.prod(0..N); // Range で範囲指定
        let res2 = seg.all_prod();
        assert_eq!(res1.x, res2.x);
        assert_eq!(res1.y, res2.y);
    }
    {
        let res = seg.prod(1..5);
        assert_eq!(res.x, -2);
        assert_eq!(res.y, -1);
    }
}
```
