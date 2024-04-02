use std::ops::{Add, Rem};

/// 多次元の vector を作成します
///
/// Examples
///  ```ignore
/// # N*N の vector
/// let mut result = mat![0; N; N];
/// ```
#[macro_export]
macro_rules! mat {
($($e:expr),*) => { Vec::from(vec![$($e),*]) };
($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[macro_export]
macro_rules! echo { ($($num:expr),*)=> {
    let mut tmp=vec![];
    $ (tmp.push(format!("{}",$num));) *
    println!("{}",tmp.join(" ")); };
}
#[macro_export]
macro_rules! ec { ($($num:expr),*)=> {
    let mut tmp=vec![];
    $ (tmp.push(format!("{}",$num));) *
    println!("{}",tmp.join(" ")); };
}

#[macro_export]
macro_rules! YesNo {
    ($num:expr) => {
        if ($num) as i64 == 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    };
}
#[macro_export]
macro_rules! Yes {
    () => {
        println!("Yes");
    };
}
#[macro_export]
macro_rules! No {
    () => {
        println!("No");
    };
}

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

pub fn print_vec<T>(v: &[T])
where
    T: std::fmt::Display,
{
    for i in 0..v.len() {
        print!("{}{}", v[i], if i + 1 == v.len() { "" } else { " " });
    }
    println!();
}

/// 正の剰余を求める
pub fn pmod<T: Copy + Add<Output = T> + Rem<Output = T>>(x: T, m: T) -> T {
    ((x % m) + m) % m
}

/// lower_bound like c++
pub fn lower_bound<T>(a: &[T], x: &T) -> usize
where
    T: Ord,
{
    if a.len() == 0 || a[0] >= *x {
        return 0;
    }
    let mut l = 0;
    let mut r = a.len();
    while l + 1 < r {
        let m = (l + r) / 2;
        if a[m] < *x {
            l = m;
        } else {
            r = m;
        }
    }
    r
}

/// upper_bound like c++
pub fn upper_bound<T>(a: &[T], x: &T) -> usize
where
    T: Ord,
{
    if a.len() == 0 || a[0] > *x {
        return 0;
    }
    let mut l = 0;
    let mut r = a.len();
    while l + 1 < r {
        let m = (l + r) / 2;
        if a[m] <= *x {
            l = m;
        } else {
            r = m;
        }
    }
    r
}

#[cfg(test)]
mod bound_test {
    use super::{lower_bound, upper_bound};

    #[test]
    fn test_lb() {
        let a = [1, 2, 4, 8];
        assert_eq!(lower_bound(&a, &0), 0);
        assert_eq!(lower_bound(&a, &1), 0);
        assert_eq!(lower_bound(&a, &2), 1);
        assert_eq!(lower_bound(&a, &3), 2);
        assert_eq!(lower_bound(&a, &8), 3);
        assert_eq!(lower_bound(&a, &9), 4);
    }
    #[test]
    fn test_ub() {
        let a = [1, 2, 4, 8];
        assert_eq!(upper_bound(&a, &0), 0);
        assert_eq!(upper_bound(&a, &1), 1);
        assert_eq!(upper_bound(&a, &3), 2);
        assert_eq!(upper_bound(&a, &7), 3);
        assert_eq!(upper_bound(&a, &8), 4);
    }
}


#[allow(unused_macros)]
#[macro_export]
/// デバック用の出力
/// release オプションの場合は出力されません
macro_rules! db {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*), $(&$a),*);
    };
}
#[allow(unused_macros)]
#[macro_export]
/// 2次元配列のデバック用の出力
/// release オプションの場合は出力されません
macro_rules! db2d {
    ($vec:expr) => {
        #[cfg(debug_assertions)]
        {
            eprintln!("> {}=", stringify!($vec));
            for a in $vec.iter() {
                eprintln!("> {:?}", a);
            }
        }
    };
}


#[derive(PartialEq, PartialOrd)]
/// float ソート用の wrapper
pub struct OrdF<T>(pub T);

impl<T: PartialEq> Eq for OrdF<T> {}

impl<T: PartialOrd> Ord for OrdF<T> {
    fn cmp(&self, other: &OrdF<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
