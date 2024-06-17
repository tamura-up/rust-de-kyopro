use std::{
    fmt,
    ops::{self, Index, IndexMut},
};

/// down, right, up, left
pub const D4: [P; 4] = [P(1, 0), P(0, 1), P(!0, 0), P(0, !0)];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    D,
    R,
    U,
    L,
}
impl Direction {
    pub fn rev(&self) -> Self {
        match self {
            Direction::D => Direction::U,
            Direction::U => Direction::D,
            Direction::R => Direction::L,
            Direction::L => Direction::R,
        }
    }
}
impl From<usize> for Direction {
    fn from(d: usize) -> Self {
        match d {
            0 => Direction::D,
            1 => Direction::R,
            2 => Direction::U,
            3 => Direction::L,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Direction::D => 'D',
            Direction::R => 'R',
            Direction::U => 'U',
            Direction::L => 'L',
        };
        write!(f, "{}", c)
    }
}

/// 座標を表す構造体
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct P(pub usize, pub usize);

impl P {
    pub fn adj4(self) -> impl Iterator<Item = P> {
        D4.iter().map(move |&d| self.add(&P(d.0, d.1)))
    }

    pub fn add(self, rhs: &P) -> P {
        P(self.0.wrapping_add(rhs.0), self.1.wrapping_add(rhs.1))
    }
    /// マンハッタン距離を求める
    pub fn dist(&self, rhp: &P) -> usize {
        let a = (self.0 as i32 - rhp.0 as i32).abs() as usize;
        let b = (self.1 as i32 - rhp.1 as i32).abs() as usize;
        a + b
    }
}
impl ops::Add<Direction> for P {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self {
        match rhs {
            Direction::D => self.add(&D4[0]),
            Direction::R => self.add(&D4[1]),
            Direction::U => self.add(&D4[2]),
            Direction::L => self.add(&D4[3]),
        }
    }
}

impl<T> Index<P> for Vec<Vec<T>> {
    type Output = T;
    fn index(&self, p: P) -> &T {
        &self[p.0][p.1]
    }
}

impl<T> IndexMut<P> for Vec<Vec<T>> {
    fn index_mut(&mut self, p: P) -> &mut T {
        &mut self[p.0][p.1]
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    h: usize,
    w: usize,
    pub g: Vec<Vec<T>>,
}
impl<T> Grid<T>
where
    T: Copy,
{
    pub fn from(g: &Vec<Vec<T>>) -> Self {
        Grid {
            h: g.len(),
            w: g[0].len(),
            g: g.clone(),
        }
    }
    /// # Arguments
    ///
    /// * `I` - 初期値
    #[allow(non_snake_case)]
    pub fn new(h: usize, w: usize, I: T) -> Self {
        let g = vec![vec![I; w]; h];
        Grid { h, w, g }
    }
}

impl<T> Grid<T> {
    /// p の隣接4点の iterator を返します
    ///
    /// ## 注意
    /// 以下のコードよりも少し遅い
    ///  ```ignore
    ///  for q in p.adj4(){
    ///    if g.is_valid_position(&q){
    ///      ....
    ///    }
    ///  }
    ///  ```
    pub fn adj4<'a>(&'a self, p: P) -> impl Iterator<Item = P> + 'a {
        D4.iter()
            .map(move |&d| p.add(&P(d.0, d.1)))
            .filter(|&p| p.0 < self.h && p.1 < self.w)
    }

    /// b から 各 offset 分移動した座標 の Iterator<P> を返します
    pub fn positions_from<'a>(&'a self, b: P, offsets: &'a [P]) -> impl Iterator<Item = P> + 'a {
        let x = offsets
            .iter()
            .map(move |&d| P(b.0.wrapping_add(d.0), b.1.wrapping_add(d.1)));
        x.filter(|&p| p.0 < self.h && p.1 < self.w)
    }

    /// b から offset 移動した場所の座標を返します
    /// グリッド範囲外の場合は None を返します
    pub fn position_from(&self, b: P, offset: P) -> Option<P> {
        let x = b.add(&offset);
        if self.is_valid_position(&x) {
            Some(x)
        } else {
            None
        }
    }

    /// p がグリッド内の座標を示しているかを返します
    pub fn is_valid_position(&self, p: &P) -> bool {
        p.0 < self.h && p.1 < self.w
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    #[inline]
    fn index(&self, idx: usize) -> &[T] {
        &self.g[idx]
    }
}
impl<T> IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut [T] {
        &mut self.g[idx]
    }
}

impl<T> Index<P> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, idx: P) -> &T {
        &self.g[idx.0][idx.1]
    }
}

impl<T> IndexMut<P> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, p: P) -> &mut T {
        &mut self.g[p.0][p.1]
    }
}

#[cfg(test)]
mod test_mod {
    use super::{Grid, D4, P};

    #[test]
    fn test_position_from() {
        let g = Grid::new(3, 3, 0);
        for d in D4 {
            assert_eq!(g.position_from(P(1, 1), d), Some(P(1, 1).add(&d)));
        }
        assert_eq!(g.position_from(P(1, 0), P(0, !0)), None);
    }

    #[test]
    fn test_positions_from() {
        let g = Grid::new(2, 2, 0);
        let offsets = vec![P(1, 0), P(0, 1), P(!0, 0), P(0, !0)];
        let ps: Vec<P> = g.positions_from(P(0, 1), &offsets).collect();
        let expected = vec![P(0, 0), P(1, 1)];
        assert_eq!(ps.len(), expected.len());
        assert!(expected.into_iter().all(|p| ps.iter().any(|&x| x == p)));
    }
    #[test]
    fn test_adj4() {
        let g = Grid::new(3, 3, 0);
        {
            let ps = g.adj4(P(0, 0)).collect::<Vec<P>>();
            let expected = vec![P(1, 0), P(0, 1)];
            assert_eq!(ps.len(), expected.len());
            assert!(expected.into_iter().all(|p| ps.iter().any(|&x| x == p)));
        }
        {
            let ps = g.adj4(P(1, 1)).collect::<Vec<P>>();
            let expected = vec![P(1, 0), P(0, 1), P(2, 1), P(1, 2)];
            assert_eq!(ps.len(), expected.len());
            assert!(expected.into_iter().all(|p| ps.iter().any(|&x| x == p)));
        }
    }
}

/// グリッドの時計回り回転
pub fn rot_clock<T>(vec: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy + Default,
{
    let h = vec.len();
    let w = vec[0].len();
    let mut res = vec![vec![T::default(); h]; w];
    for i in 0..w {
        for j in 0..h {
            res[i][j] = vec[h - 1 - j][i];
        }
    }
    res
}

/// グリッドの半時計回り回転
pub fn rot_rev_clock<T>(vec: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy + Default,
{
    let h = vec.len();
    let w = vec[0].len();
    let mut res = vec![vec![T::default(); h]; w];
    for i in 0..w {
        for j in 0..h {
            res[i][j] = vec[j][w - i - 1];
        }
    }
    res
}
