lazy segment tree の実装例

## links
- [公式ドキュメント](https://atcoder.github.io/ac-library/document_ja/lazysegtree.html)
- [rust acl test case](https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/lazysegtree.rs#L354)
- [rust acl example](https://github.com/rust-lang-ja/ac-library-rs/blob/master/examples/practice2_l_lazy_segment_tree.rs)

## (TODO) まだ理解できていないこと

+ $`id\circ f = f`$ となるべき？
    + $`(id\circ f)(x)`$ は、`mapping`で $`f(id(x))`$ と演算することと等価という認識
+ `composition` で `id` を返すとそこで遅延評価の伝搬が止まるという認識をしているが合っているか？

## examples

### 区間加算・区間最大値

```rs
use ac_library::{LazySegtree, MapMonoid, Max};

// 区間加算、区間最大値
struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<i32>;
    type F = i32;

    // 恒等写像 id
    // mapping(id, x) = x となる値
    fn identity_map() -> Self::F {
        0
    }

    // F
    // 遅延していた区間の更新をノードに適用するイメージ
    fn mapping(&f: &Self::F, &x: &i32) -> i32 {
        f + x
    }

    // F 同士の演算(写像の合成?) f∘g
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}

fn main() {
    const SZ: usize = 100_000 * 2 + 1;
    let mut seg: LazySegtree<MaxAdd> = LazySegtree::new(SZ);
    // 0 に初期化
    for i in 0..SZ {
        seg.set(i, 0);
    }
    seg.apply_range(0..5, 2);
    seg.apply_range(4..8, 3);
    assert_eq!(seg.all_prod(), 5);
}
```

使える問題

+ https://atcoder.jp/contests/abc327/tasks/abc327_f

## 区間加算・区間合計

```rs
use ac_library::{LazySegtree, MapMonoid, Max, Monoid};

#[derive(Debug, Clone)]
struct Data {
    e: f64,
    sz: i32, // 区間サイズ
}
impl Monoid for Data {
    type S = Data;

    fn identity() -> Self::S {
        Data { e: 0.0, sz: 0 }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a.sz == 0 {
            return b.clone();
        }
        if b.sz == 0 {
            return a.clone();
        }

        Data {
            e: a.e + b.e,
            sz: a.sz + b.sz,
        }
    }
}

struct LazyData;
impl MapMonoid for LazyData {
    type M = Data;
    type F = f64;

    // 恒等写像 id
    // mapping(id, x) = x となる値
    fn identity_map() -> Self::F {
        0.0
    }

    // F
    // 区間の値をノードに適用するイメージ
    fn mapping(&f: &Self::F, x: &Self::M) -> Self::M {
        Data {
            // 区間 [l, r] それぞれに +x した合計値を保存しているので、 sz*x 加算する
            e: x.e + f * x.sz as f64, 
            sz: x.sz,
        }
    }

    // F 同士の演算(写像の合成?) f∘g
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        f + g
    }
}
```

使える問題
+ https://atcoder.jp/contests/typical90/tasks/typical90_bn

## Monoid, MapMonoid を自作 Struct に

```rs
use ac_library::{LazySegtree, MapMonoid, Monoid};

#[derive(Debug, Clone)]
struct Data {
    x: usize,
}
impl Data {
    fn new(v: usize) -> Data {
        Data { x: v }
    }
}
impl Monoid for Data {
    type S = Data;

    fn identity() -> Self::S {
        Data { x: 0 }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        Data { x: a.x + b.x }
    }
}

struct LazySegData;
impl MapMonoid for LazySegData {
    type M = Data;
    type F = Option<i32>;

    // 恒等写像 id
    // mapping(id, x) = x となる値
    fn identity_map() -> Self::F {
        None
    }

    // F
    // 遅延していた区間の更新をノードに適用するイメージ
    fn mapping(&f: &Self::F, data: &Self::M) -> Self::M {
        if f.is_none() {
            return data.clone();
        }
        let f = f.unwrap();
        if f == 1 {
            Data { x: data.x ^ 1 }
        } else {
            data.clone()
        }
    }

    // F 同士の演算(写像の合成?) f∘g
    fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
        // f or g が id の場合は skip
        if f.is_none() {
            return g.clone();
        }
        if g.is_none() {
            return f.clone();
        }
        let f = f.unwrap();
        let g = g.unwrap();
        Some(f ^ g)
    }
}

fn main() {
    input! { N:usize,Q:usize, }

    let mut seg: LazySegtree<LazySegData> = LazySegtree::new(N + 5);
    // 初期値の設定
    for i in 0..N {
        seg.set(i, Data::new(1));
    }
    for _qi in 0..Q {
        // クエリ処理
    }
}
```

### その他サンプル

+ https://atcoder.jp/contests/abc357/submissions/54385839
