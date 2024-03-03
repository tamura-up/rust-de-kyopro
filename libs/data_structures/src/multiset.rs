//! multiset です
//! 多重集合を管理します
//!
use std::collections::BTreeMap;

use crate::btree_neighbors::MapNeighbors;

#[derive(Clone, Debug)]
pub struct Multiset<T> {
    size: usize,
    map: BTreeMap<T, usize>,
}

impl<T> Multiset<T>
where
    T: Ord + Copy,
{
    /// # Examples
    ///
    /// ```
    /// use kyopro_data_stractures::multiset::Multiset;
    ///
    /// let mut st = Multiset::new();
    /// st.insert(1);
    /// st.insert(1);
    /// st.insert(2);
    /// assert_eq!(st.len(), 3);
    /// ```
    pub fn new() -> Multiset<T> {
        Multiset { size: 0, map: BTreeMap::new() }
    }

    /// 要素を追加します
    pub fn insert(&mut self, val: T) {
        self.size += 1;
        *self.map.entry(val).or_insert(0) += 1;
    }

    /// 要素を削除します
    pub fn erase(&mut self, val: T) {
        // HACK: 要素が存在しない場合はエラーにしたほうが良いですか?
        if self.map.contains_key(&val) {
            self.map.entry(val).and_modify(|v| *v -= 1);
            if self.map[&val] == 0 {
                self.map.remove(&val);
            }
            self.size -= 1;
        }
    }

    /// 最小値を返します
    pub fn get_min(&self) -> Option<T> {
        if let Some((&k, _)) = self.map.iter().nth(0) {
            return Some(k);
        } else {
            return None;
        }
    }

    /// 最大値を返します
    pub fn get_max(&self) -> Option<T> {
        if let Some((&k, _)) = self.map.iter().last() {
            return Some(k);
        } else {
            return None;
        }
    }
    /// 要素 val が存在するかどうかを返します
    pub fn contains(&self, val: &T) -> bool {
        self.map.contains_key(val)
    }

    /// 集合を vector で返します
    pub fn to_vec(&self) -> Vec<T> {
        let mut res = vec![];
        for (&k, &v) in self.map.iter() {
            for _ in 0..v {
                res.push(k);
            }
        }
        return res;
    }
    /// val 以上の最小の値を返します
    pub fn find_first(&self, val: &T) -> Option<T> {
        if let Some((k, _)) = self.map.find_first(val) {
            Some(*k)
        } else {
            None
        }
    }

    /// val 以下の最大の値を返します
    pub fn find_last(&self, val: &T) -> Option<T> {
        if let Some((k, _)) = self.map.find_last(val) {
            Some(*k)
        } else {
            None
        }
    }

    /// 登録要素数を返します
    pub fn len(&self) -> usize {
        self.size
    }
    /// ユニークな要素数を返します
    pub fn unique_len(&self) -> usize {
        self.map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Multiset;

    #[test]
    fn it_works() {
        let mut st = Multiset::new();
        st.insert(1usize);
        st.insert(2);
        st.insert(2);
        assert_eq!(st.size, 3);
        st.erase(2);
        assert_eq!(st.size, 2);
        assert!(st.contains(&1));
        assert!(st.contains(&2));
        assert!(!st.contains(&3));
    }

    #[test]
    fn test_get_max() {
        let mut st = Multiset::new();
        st.insert(1);
        st.insert(2);
        assert_eq!(st.get_max(), Some(2));
        st.erase(2);
        assert_eq!(st.get_max(), Some(1));
        st.erase(1);
        assert_eq!(st.get_max(), None);
    }

    #[test]
    fn test_get_min() {
        let mut st = Multiset::new();
        st.insert(1);
        st.insert(2);
        assert_eq!(st.get_min(), Some(1));
        st.erase(1);
        assert_eq!(st.get_min(), Some(2));
        st.erase(2);
        assert_eq!(st.get_min(), None);
    }

    #[test]
    fn test_to_vec() {
        let mut st = Multiset::new();
        st.insert(1usize);
        assert_eq!(st.to_vec(), vec![1]);
        st.insert(2);
        st.insert(2);
        assert_eq!(st.to_vec(), vec![1, 2, 2]);
    }
    #[test]
    fn test_find_first_last() {
        let mut st = Multiset::new();
        st.insert(2usize);
        st.insert(2usize);
        st.insert(2usize);
        st.insert(4usize);
        st.insert(4usize);
        assert_eq!(st.find_first(&3), Some(4));
        assert_eq!(st.find_first(&4), Some(4));
        assert_eq!(st.find_first(&5), None);

        assert_eq!(st.find_last(&3), Some(2));
        assert_eq!(st.find_last(&2), Some(2));
        assert_eq!(st.find_last(&1), None);
    }
}
