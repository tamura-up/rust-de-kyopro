use std::{mem::swap, ops::Index};

/// 座標圧縮
/// ```
/// use kyopro_zaatsu::Zaatsu;
/// let vals = [3, 2, 4];
/// let mut za = Zaatsu::new();
/// for &v in &vals {
///     za.add(v);
/// }
/// za.init();
/// assert_eq!(za.index(&2), 0);
/// assert_eq!(za.index(&3), 1);
/// assert_eq!(za.index(&4), 2);
/// ```
pub struct Zaatsu<T>
where
    T: Ord,
{
    list: Vec<T>,
    initialized: bool,
}
impl<T> Zaatsu<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Zaatsu {
            list: vec![],
            initialized: false,
        }
    }
    pub fn add(&mut self, v: T) {
        self.list.push(v);
    }
    pub fn init(&mut self) {
        self.initialized = true;
        self.list.sort();
        if self.list.len() == 0 {
            return;
        }
        let mut tmp = vec![];
        swap(&mut self.list, &mut tmp);
        for v in tmp {
            if self.list.is_empty() || *(self.list.last().unwrap()) < v {
                self.list.push(v);
            }
        }
    }
    pub fn index(&self, v: &T) -> usize {
        assert!(self.initialized);
        let mut l = 0;
        let mut r = self.list.len();
        while l + 1 < r {
            let m = (l + r) / 2;
            if self.list[m] <= *v {
                l = m;
            } else {
                r = m;
            }
        }
        l
    }
    pub fn size(&self)->usize{
        assert!(self.initialized);
        self.list.len()
    }
}

impl<T> Index<usize> for Zaatsu<T>
where
    T: Ord,
{
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.list[i]
    }
}

#[cfg(test)]
mod test_mod {
    use super::Zaatsu;
    use itertools::Itertools;

    #[test]
    fn test1() {
        let vals = vec![3, 5, 7, 1];
        let mut ordered = vals.clone();
        ordered.sort();
        let mut za = Zaatsu::new();
        for &v in &vals {
            za.add(v);
        }
        za.init();
        for i in 0..vals.len() {
            assert_eq!(i, za.index(&ordered[i]));
            assert_eq!(ordered[i], za[i]);
        }
    }
    #[test]
    fn test_has_duplicate_values() {
        let vals = vec![3, 5, 5, 2, 1, 2, 5, 4, 4];
        let mut ordered = vals.clone();
        ordered.sort();
        let uniq = ordered.into_iter().unique().collect_vec();
        let mut za = Zaatsu::new();
        for &v in &vals {
            za.add(v);
        }
        za.init();
        for i in 0..uniq.len() {
            assert_eq!(i, za.index(&uniq[i]));
            assert_eq!(uniq[i], za[i]);
        }
    }
}
