//! K が小さいトップ n 個を保持するリスト
use std::collections::BinaryHeap;

/// K が小さいトップ n 個を保持するリスト
/// from: https://atcoder.jp/contests/ahc032/submissions/52151408
///
/// オンラインで最小値を取得できないことに注意してください。最小値を得るためには、`list()` を呼び出す必要があります。
/// オンラインで最小値が欲しい場合は、Interval Heap の利用を検討してください。
///
/// # Example
///
/// ```
/// use kyopro_bounded_sorted_list::BoundedSortedList;
///
/// let mut lst = BoundedSortedList::new(3);
/// // 値の登録は `{key: value}` の形で追加する。リストは `key` の昇順に保持する。
/// lst.insert(3, 13);
/// lst.insert(2, 12);
/// lst.insert(4, 14);
/// assert_eq!(lst.list(), [(2, 12), (3, 13), (4, 14)]);
/// // lst.get_min_value() のようにオンラインで最小値の取得はできない
///
/// lst.insert(1, 11);
/// assert_eq!(lst.list(), [(1, 11), (2, 12), (3, 13)]);
/// ```
#[derive(Clone, Debug)]
pub struct BoundedSortedList<K: PartialOrd + Copy, V: Clone> {
    que: BinaryHeap<Entry<K, V>>,
    size: usize,
}

impl<K: PartialOrd + Copy, V: Clone> BoundedSortedList<K, V> {
    /// 最大 `size` 個のデータを昇順で保持するリストを作成します
    pub fn new(size: usize) -> Self {
        Self { que: BinaryHeap::with_capacity(size), size }
    }
    /// 値 k のデータを追加できるか？
    pub fn can_insert(&self, k: K) -> bool {
        self.que.len() < self.size || self.que.peek().unwrap().k > k
    }
    /// データ{k, v} を追加します
    /// O(logN)
    pub fn insert(&mut self, k: K, v: V) {
        if self.can_insert(k) {
            if self.que.len() == self.size {
                self.que.pop();
            }
            self.que.push(Entry { k, v });
        }
    }
    /// 昇順に {k, v} の vector を返します。
    /// O(NlogN)
    pub fn list(&self) -> Vec<(K, V)> {
        let mut v = self.que.clone().into_vec();
        v.sort();
        v.into_iter().map(|e| (e.k, e.v)).collect()
    }
    /// 現在のリストサイズ
    pub fn len(&self) -> usize {
        self.que.len()
    }
}
#[test]
fn test_bounded_sorted_list() {
    let mut lst = BoundedSortedList::new(3);
    lst.insert(3, 13);
    lst.insert(2, 12);
    lst.insert(4, 14);
    assert_eq!(lst.list(), [(2, 12), (3, 13), (4, 14)]);
    lst.insert(5, 15);
    assert_eq!(lst.list(), [(2, 12), (3, 13), (4, 14)]);
    lst.insert(1, 11);
    assert_eq!(lst.list(), [(1, 11), (2, 12), (3, 13)]);
}

/// BoundedSortedList の要素
#[derive(Clone, Debug)]
struct Entry<K, V> {
    k: K,
    v: V,
}

impl<K: PartialOrd, V> Ord for Entry<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<K: PartialOrd, V> PartialOrd for Entry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.k.partial_cmp(&other.k)
    }
}

impl<K: PartialEq, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.k.eq(&other.k)
    }
}

impl<K: PartialEq, V> Eq for Entry<K, V> {}
