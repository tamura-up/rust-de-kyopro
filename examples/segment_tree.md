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

## max_right, min_left の使い方メモ

```rs
use ac_library::{Additive, Segtree};

fn main() {
    // https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/segtree.rs
    let a = vec![0, 1, 1, 0, 1, 1];
    let seg: Segtree<Additive<i64>> = Segtree::from(a.clone());

    // max_right
    // > fが単調だとすれば、f(op(a[l], a[l + 1], ..., a[r - 1])) = true となる最大の r、と解釈することが可能です。
    // 条件を満たす最大の半開区間 [l, r) があり、l を指定したら r を返してくれるイメージ
    // f には、半開区間 [l, r) で条件をみたす関数を渡す
    assert_eq!(seg.max_right(1, |&x| x <= 1), 2);
    assert_eq!(seg.max_right(1, |&x| x <= 2), 4);

    // min_left
    // > fが単調だとすれば、f(op(a[l], a[l + 1], ..., a[r - 1])) = true となる最小の l、と解釈することが可能です。
    // 上の説明において `r` が区間に含まれていないことに注意。
    // 条件を満たす最大の半開区間 [l, r) があり、r を指定したら l を返してくれるイメージ
    // f には、半開区間 [l, r) で条件をみたす関数を渡す。
    assert_eq!(seg.min_left(2, |&x| x <= 1), 0);
    assert_eq!(seg.min_left(3, |&x| x <= 1), 2);
}
```
