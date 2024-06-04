//! Interval Heap (両端優先度付きキュー)です。
//! from: https://qiita.com/hatoo@github/items/652b81e8e83b0680bc0a#interval-heap
//! メモ: https://www.notion.so/Interval-Heap-c534cc8bcdba479c9ff9a16c24449c23
//!
//! ## 実装メモ
//! [両端優先度付きキューのInterval-Heap実装](https://natsugiri.hatenablog.com/entry/2016/10/10/035445) を参考にすると良い。
//! この実装では、偶数 index に min heap、奇数 index に max heap を持つ点が異なるので注意。
use std::cmp::Reverse;

/// Interval Heap (両端優先度付きキュー)です。
#[derive(Clone, Debug)]
pub struct IntervalHeap<T: PartialOrd> {
    data: Vec<T>,
}

impl<T: PartialOrd> IntervalHeap<T> {
    pub fn new() -> IntervalHeap<T> {
        IntervalHeap { data: Vec::new() }
    }
    /// capacity を指定して初期化します
    pub fn with_capacity(n: usize) -> IntervalHeap<T> {
        IntervalHeap { data: Vec::with_capacity(n) }
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// データ x を追加します。
    #[inline]
    pub fn push(&mut self, x: T) {
        let i = self.data.len();
        self.data.push(x);
        self.up(i);
    }
    /// 最小値の参照を返します。
    /// Heap が空の場合は None です。
    #[inline]
    pub fn peek_min(&self) -> Option<&T> {
        self.data.first()
    }
    /// 最大値の参照を返します。
    /// Heap が空の場合は None です。
    #[inline]
    pub fn peek_max(&self) -> Option<&T> {
        if self.data.len() > 1 {
            self.data.get(1)
        } else {
            self.data.first()
        }
    }
    /// 最小値を返します。
    /// Heap が空の場合は None です。
    #[allow(dead_code)]
    #[inline]
    pub fn pop_min(&mut self) -> Option<T> {
        if self.data.len() <= 1 {
            return self.data.pop();
        }
        let len = self.data.len();
        self.data.swap(0, len - 1);
        let res = self.data.pop();
        self.down(0);
        res
    }
    /// 最大値を返します。
    /// Heap が空の場合は None です。
    #[allow(dead_code)]
    #[inline]
    pub fn pop_max(&mut self) -> Option<T> {
        if self.data.len() <= 2 {
            return self.data.pop();
        }
        let len = self.data.len();
        self.data.swap(1, len - 1);
        let res = self.data.pop();
        self.down(1);
        res
    }
    /// 親の index を返します。
    #[allow(dead_code)]
    #[inline]
    fn parent(i: usize) -> usize {
        ((i >> 1) - 1) & !1
    }
    #[allow(dead_code)]
    #[inline]
    fn down(&mut self, i: usize) {
        let mut i = i;
        let n = self.data.len();
        if i & 1 == 0 {
            while (i << 1) + 2 < n {
                let mut k = (i << 1) + 2;
                // 子のうち小さい方が swap 候補
                if k + 2 < n && unsafe { self.data.get_unchecked(k + 2) } < unsafe { self.data.get_unchecked(k) } {
                    k = k + 2;
                }
                if unsafe { self.data.get_unchecked(i) } > unsafe { self.data.get_unchecked(k) } {
                    self.data.swap(i, k);
                    i = k;
                    // 左右 swap
                    if i + 1 < self.data.len()
                        && unsafe { self.data.get_unchecked(i) } > unsafe { self.data.get_unchecked(i + 1) }
                    {
                        self.data.swap(i, i + 1);
                    }
                } else {
                    break;
                }
            }
        } else {
            while (i << 1) + 1 < n {
                let mut k = (i << 1) + 1;
                // 子のうち大きい方が swap 候補
                if k + 2 < n && unsafe { self.data.get_unchecked(k + 2) } > unsafe { self.data.get_unchecked(k) } {
                    k = k + 2;
                }
                if unsafe { self.data.get_unchecked(i) } < unsafe { self.data.get_unchecked(k) } {
                    self.data.swap(i, k);
                    i = k;
                    if i > 0 && unsafe { self.data.get_unchecked(i) } < unsafe { self.data.get_unchecked(i - 1) } {
                        self.data.swap(i, i - 1);
                    }
                } else {
                    break;
                }
            }
        }
    }
    /// up 操作を行います。
    #[allow(dead_code)]
    #[inline]
    fn up(&mut self, i: usize) {
        let mut i = i;
        // 左右比較
        if i & 1 == 1 && unsafe { self.data.get_unchecked(i) } < unsafe { self.data.get_unchecked(i - 1) } {
            self.data.swap(i, i - 1);
            i -= 1;
        }
        // 親 min と比較
        while i > 1 && unsafe { self.data.get_unchecked(i) } < unsafe { self.data.get_unchecked(Self::parent(i)) } {
            let p = Self::parent(i);
            self.data.swap(i, p);
            i = p;
        }
        // 親 max と比較
        while i > 1 && unsafe { self.data.get_unchecked(i) } > unsafe { self.data.get_unchecked(Self::parent(i) + 1) } {
            let p = Self::parent(i) + 1;
            self.data.swap(i, p);
            i = p;
        }
    }
    /// ヒープをクリアします。
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

#[test]
fn test_interval_heap_peek() {
    let mut heap = IntervalHeap::<i32>::new();
    heap.push(2);
    assert_eq!(heap.peek_min(), Some(&2));
    assert_eq!(heap.peek_max(), Some(&2));
    heap.push(5);
    assert_eq!(heap.peek_min(), Some(&2));
    assert_eq!(heap.peek_max(), Some(&5));
    heap.push(3);
    assert_eq!(heap.peek_min(), Some(&2));
    assert_eq!(heap.peek_max(), Some(&5));
    heap.push(1);
    assert_eq!(heap.peek_min(), Some(&1));
    assert_eq!(heap.peek_max(), Some(&5));
}

#[test]
fn test_interval_heap_pop_min() {
    let mut heap = IntervalHeap::<i32>::new();
    heap.push(2);
    heap.push(5);
    heap.push(3);
    heap.push(1);
    heap.push(4);
    for i in 1..=5 {
        let x = heap.pop_min();
        assert_eq!(x, Some(i));
    }
    assert_eq!(heap.pop_min(), None);
}

#[test]
fn test_interval_heap_pop_max() {
    let mut heap = IntervalHeap::<i32>::new();
    heap.push(2);
    heap.push(5);
    heap.push(3);
    heap.push(1);
    heap.push(4);
    for i in (1..=5).rev() {
        let x = heap.pop_max();
        assert_eq!(x, Some(i));
    }
    assert_eq!(heap.pop_min(), None);
}

/// 容量制限付きの最大値優先キューです。
#[derive(Clone, Debug)]
pub struct LimitedMaximumHeap<T: PartialOrd> {
    heap: IntervalHeap<T>,
    limit: usize,
}

impl<T: PartialOrd> LimitedMaximumHeap<T> {
    /// コンストラクタ
    /// + limit: 最大容量
    pub fn new(limit: usize) -> LimitedMaximumHeap<T> {
        LimitedMaximumHeap { heap: IntervalHeap::with_capacity(limit), limit }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    #[inline]
    pub fn push(&mut self, x: T) -> Option<T> {
        if self.heap.len() < self.limit {
            self.heap.push(x);
            None
        } else {
            // x が heap の 最小値より大きいなら入れ替える
            if self.heap.data[0] < x {
                let mut x = x;
                std::mem::swap(&mut x, &mut self.heap.data[0]);
                if self.heap.len() >= 2 && self.heap.data[0] > self.heap.data[1] {
                    self.heap.data.swap(0, 1);
                }
                self.heap.down(0);
                Some(x)
            } else {
                Some(x)
            }
        }
    }
    /// heap から最大値を取得します。
    /// heap が空の場合は None を返します。
    #[allow(dead_code)]
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop_max()
    }
    /// 最大値への参照を返します。
    /// heap が空の場合は None を返します。
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek_max()
    }
    #[allow(dead_code)]
    #[inline]
    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

/// 容量制限付きの最小値優先キューです。
#[derive(Clone, Debug)]
pub struct LimitedMinimumHeap<T: PartialOrd> {
    heap: LimitedMaximumHeap<Reverse<T>>,
}

impl<T: PartialOrd> LimitedMinimumHeap<T> {
    pub fn new(limit: usize) -> LimitedMinimumHeap<T> {
        LimitedMinimumHeap { heap: LimitedMaximumHeap::new(limit) }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    #[inline]
    pub fn push(&mut self, x: T) -> Option<T> {
        if let Some(Reverse(x)) = self.heap.push(Reverse(x)) {
            Some(x)
        } else {
            None
        }
    }
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if let Some(Reverse(x)) = self.heap.pop() {
            Some(x)
        } else {
            None
        }
    }
    #[inline]
    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

#[test]
fn test_limited_minimum_heap() {
    let mut que = LimitedMinimumHeap::<i32>::new(3);
    que.push(3);
    que.push(4);
    que.push(1);
    que.push(2);
    assert_eq!(que.pop(), Some(1));
    assert_eq!(que.pop(), Some(2));
    assert_eq!(que.pop(), Some(3));
    assert_eq!(que.pop(), None);
}
#[test]
fn test_limited_maximum_heap() {
    let mut que = LimitedMaximumHeap::<i32>::new(3);
    que.push(3);
    que.push(4);
    que.push(1);
    que.push(2);
    assert_eq!(que.pop(), Some(4));
    assert_eq!(que.pop(), Some(3));
    assert_eq!(que.pop(), Some(2));
    assert_eq!(que.pop(), None);
}
