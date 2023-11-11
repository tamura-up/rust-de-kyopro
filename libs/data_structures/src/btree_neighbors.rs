//! Set, Map に lower_bound ライクなメソッドを追加する trait です
//! original: https://github.com/yiolino/atcoder-rust#btreeset
//!
use std::collections::{BTreeMap, BTreeSet};

pub trait Neighbors<T> {
    /// x 以上の最小値を検索
    fn find_first(&self, x: &T) -> Option<&T>;
    /// x 以下の最大値を検索
    fn find_last(&self, x: &T) -> Option<&T>;
}

impl<T: Ord> Neighbors<T> for BTreeSet<T> {
    fn find_first(&self, x: &T) -> Option<&T> {
        let mut aftr = self.range((std::ops::Bound::Included(x), std::ops::Bound::Unbounded));
        aftr.next()
    }
    fn find_last(&self, x: &T) -> Option<&T> {
        let mut bfr = self.range((std::ops::Bound::Unbounded, std::ops::Bound::Included(x)));
        bfr.next_back()
    }
}

// HACK: set とひとつにまとめられないか？
pub trait MapNeighbors<K, V> {
    /// x 以上の最小値を取得
    fn find_first(&self, x: &K) -> Option<(&K, &V)>;
    /// x 以下の最大値を取得
    fn find_last(&self, x: &K) -> Option<(&K, &V)>;
}
impl<K: Ord, V> MapNeighbors<K, V> for BTreeMap<K, V> {
    fn find_first(&self, x: &K) -> Option<(&K, &V)> {
        let mut aftr = self.range((std::ops::Bound::Included(x), std::ops::Bound::Unbounded));
        aftr.next()
    }

    fn find_last(&self, x: &K) -> Option<(&K, &V)> {
        let mut bfr = self.range((std::ops::Bound::Unbounded, std::ops::Bound::Included(x)));
        bfr.next_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};
    #[test]
    fn test_set_impl() {
        let set = BTreeSet::from([2, 4, 6, 8, 10]);
        assert_eq!(set.find_first(&2), Some(&2));
        assert_eq!(set.find_first(&3), Some(&4));
        assert_eq!(set.find_first(&10), Some(&10));
        assert_eq!(set.find_first(&11), None);
    }
    #[test]
    fn test_set_impl_rev() {
        let set = BTreeSet::from([2, 4, 6, 8, 10]);
        assert_eq!(set.find_last(&10), Some(&10));
        assert_eq!(set.find_last(&9), Some(&8));
        assert_eq!(set.find_last(&2), Some(&2));
        assert_eq!(set.find_last(&1), None);
    }

    #[test]
    fn test_map_impl() {
        let mut mp = BTreeMap::new();
        mp.insert(2, 1);
        mp.insert(4, 2);
        mp.insert(8, 3);

        assert_eq!(mp.find_first(&2), Some((&2, &1)));
        assert_eq!(mp.find_first(&3), Some((&4, &2)));
        assert_eq!(mp.find_first(&8), Some((&8, &3)));
        assert_eq!(mp.find_first(&9), None);
    }
    #[test]
    fn test_map_impl_rev() {
        let mut mp = BTreeMap::new();
        mp.insert(2, 1);
        mp.insert(4, 2);
        mp.insert(8, 3);

        assert_eq!(mp.find_last(&8), Some((&8, &3)));
        assert_eq!(mp.find_last(&7), Some((&4, &2)));
        assert_eq!(mp.find_last(&2), Some((&2, &1)));
        assert_eq!(mp.find_last(&1), None);
    }
}
