## BTreeSet 上の二分探索

BTreeSet の lower_bound, upper_bound は `set.range` を使用して同様のことができる。

```rs
use std::collections::BTreeSet;

fn main() {
    let set = BTreeSet::from([2, 4, 6, 8, 10]);
    let mut iter = set.range((std::ops::Bound::Included(4),std::ops::Bound::Unbounded));
    println!("like lower_bound");
    for _ in 0..3 {
        let x = iter.next();
        println!("{:?}", x);
    }
    println!("-------------------------------");
    println!("前方向");
    let mut before = set.range((std::ops::Bound::Unbounded, std::ops::Bound::Included(6)));
    for _ in 0..3 {
        let x = before.next_back();
        println!("{:?}", x);
    }
}
```

[wandbox](https://wandbox.org/permlink/6EHyVg3T9Gv5Cweo)

## trait で実装

https://github.com/yiolino/atcoder-rust から拝借

```rs
use std::collections::BTreeSet;

trait Neighbors<T> {
    fn before(&self, x: T) -> Option<&T>;
    fn lower_bound(&self, x: T) -> Option<&T>;
    fn upper_bound(&self, x: T) -> Option<&T>;
}

impl<T: Ord> Neighbors<T> for BTreeSet<T> {
    fn before(&self, x: T) -> Option<&T> {
        let mut bfr = self.range((std::ops::Bound::Unbounded, std::ops::Bound::Excluded(x)));

        bfr.next_back()
    }
    fn lower_bound(&self, x: T) -> Option<&T> {
        let mut aftr = self.range((std::ops::Bound::Included(x), std::ops::Bound::Unbounded));
        aftr.next()
    }
    fn upper_bound(&self, x: T) -> Option<&T> {
        let mut aftr = self.range((std::ops::Bound::Excluded(x), std::ops::Bound::Unbounded));

        aftr.next()
    }
}

fn main() {
    let set = BTreeSet::from([2, 4, 6, 8, 10]);
    assert_eq!(set.lower_bound(4),Some(&4));
    assert_eq!(set.lower_bound(5),Some(&6));

    assert_eq!(set.upper_bound(4),Some(&6));
    assert_eq!(set.upper_bound(5),Some(&6));
}
```

## BTreeMap の場合

BTreeSet と同様にできる。

**lower_bound, upper_bound は今後のバージョンでは標準実装されることに注意**  
https://github.com/rust-lang/rfcs/issues/2184

```rs
use std::collections::BTreeMap;

fn main() {
    let mut mp=BTreeMap::new();
    mp.insert(1,1);
    mp.insert(2,4);
    mp.insert(3,8);
    println!("{:?}",mp);
    let mut iter=mp.range(( std::ops::Bound::Included(2),std::ops::Bound::Unbounded));
    for it in iter{
        println!("{:?}",it);
    }
}
```

[wandbox](https://wandbox.org/permlink/7kr9RMywMitOBrsY)
