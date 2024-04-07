#![allow(non_snake_case)]
///! 2次元 segment tree
/// org: https://nyaannyaan.github.io/library/data-structure-2d/2d-segment-tree.hpp

//// 2次元 segment tree
////
//// Example
//// ```
//// let mut seg = SegmentTree2D::new(3, 3, |a: usize, b: usize| a + b, 0usize);
//// for i in 0..3 {
////     for j in 0..3 {
////         seg.set(i, j, i * 3 + j);
////     }
//// }
//// seg.build();
//// assert_eq!(seg.query(0, 3, 0, 3), 36);
//// ```

pub struct SegmentTree2D<T, F> {
    H: usize,
    W: usize,
    seg: Vec<T>,
    // 単位元
    I: T,
    f: F,
}

impl<F: Fn(T, T) -> T, T: Copy + Eq> SegmentTree2D<T, F> {
    pub fn new(h: usize, w: usize, f: F, I: T) -> SegmentTree2D<T, F> {
        let mut H = 1;
        let mut W = 1;
        while H < h {
            H <<= 1;
        }
        while W < w {
            W <<= 1;
        }
        let seg = vec![I; 4 * H * W];

        SegmentTree2D { H, W, seg, I, f }
    }
    fn id(&self, h: usize, w: usize) -> usize {
        h * 2 * self.W + w
    }

    pub fn set(&mut self, h: usize, w: usize, x: T) {
        let idx = self.id(h + self.H, w + self.W);
        self.seg[idx] = x;
    }

    pub fn build(&mut self) {
        // w in [W, 2W)
        for w in self.W..2 * self.W {
            for h in (1..self.H).rev() {
                let id1 = self.id(h, w);
                let id2 = self.id(2 * h + 0, w);
                let id3 = self.id(2 * h + 1, w);
                self.seg[id1] = (self.f)(self.seg[id2], self.seg[id3]);
            }
        }
        // h in [0, 2H)
        for h in 0..2 * self.H {
            for w in (1..self.W).rev() {
                let id1 = self.id(h, w);
                let id2 = self.id(h, 2 * w + 0);
                let id3 = self.id(h, 2 * w + 1);
                self.seg[id1] = (self.f)(self.seg[id2], self.seg[id3]);
            }
        }
    }

    pub fn update(&mut self, h: usize, w: usize, x: T) {
        let mut h = h + self.H;
        let w = w + self.W;

        let id = self.id(h, w);
        self.seg[id] = x;

        {
            // 縦方向の累積
            let mut i = h >> 1;
            while i > 0 {
                let id1 = self.id(i, w);
                let id2 = self.id(2 * i + 0, w);
                let id3 = self.id(2 * i + 1, w);
                self.seg[id1] = (self.f)(self.seg[id2], self.seg[id3]);
                i >>= 1;
            }
        }
        {
            // 横方向の累積(葉より上の頂点の計算も行う)
            while h > 0 {
                let mut j = w >> 1;
                while j > 0 {
                    let id1 = self.id(h, j);
                    let id2 = self.id(h, 2 * j + 0);
                    let id3 = self.id(h, 2 * j + 1);
                    self.seg[id1] = (self.f)(self.seg[id2], self.seg[id3]);
                    j >>= 1;
                }
                h >>= 1;
            }
        }
    }

    fn _inner_query(&self, h: usize, w1: usize, w2: usize) -> T {
        let mut res: T = self.I;
        let mut w1 = w1;
        let mut w2 = w2;
        while w1 < w2 {
            if w1 & 1 > 0 {
                let id = self.id(h, w1);
                res = (self.f)(res, self.seg[id]);
                w1 += 1;
            }
            if w2 & 1 > 0 {
                w2 -= 1;
                let id = self.id(h, w2);
                res = (self.f)(res, self.seg[id]);
            }
            w1 >>= 1;
            w2 >>= 1;
        }
        res
    }

    /// [h1, w1] から [h2,w2] (半開区間)  の演算結果を求めます
    /// 範囲指定順注意 [h1, h2), [w1, w2) の順で指定する
    pub fn query(&self, h1: usize, h2: usize, w1: usize, w2: usize) -> T {
        if h1 >= h2 || w1 >= w2 {
            return self.I;
        };

        let mut h1 = h1 + self.H;
        let mut h2 = h2 + self.H;
        let w1 = w1 + self.W;
        let w2 = w2 + self.W;

        let mut res = self.I;
        while h1 < h2 {
            if h1 & 1 > 0 {
                let tmp = self._inner_query(h1, w1, w2);
                res = (self.f)(res, tmp);
                h1 += 1;
            }
            if h2 & 1 > 0 {
                h2 -= 1;
                let tmp = self._inner_query(h2, w1, w2);
                res = (self.f)(res, tmp);
            }

            h1 >>= 1;
            h2 >>= 1;
        }
        res
    }

    pub fn get(&self, h: usize, w: usize) -> T {
        self.seg[self.id(h + self.H, w + self.W)]
    }
}

#[test]
fn test_query() {
    let mut seg = SegmentTree2D::new(3, 3, |a: usize, b: usize| a + b, 0usize);
    for i in 0..3 {
        for j in 0..3 {
            seg.set(i, j, i * 3 + j);
        }
    }
    seg.build();
    assert_eq!(seg.query(0, 1, 0, 1), 0);
    assert_eq!(seg.query(2, 3, 2, 3), 8);
    assert_eq!(seg.query(0, 3, 0, 3), 36);
    assert_eq!(seg.query(0, 1, 0, 3), 3);
    assert_eq!(seg.query(1, 3, 1, 3), 24);
}



