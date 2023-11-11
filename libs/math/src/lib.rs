//! 算数系の処理をまとめています
pub mod prime;

// 以下 num-integer の実装を参考にしています
// ## 参考サイト
// - https://docs.rs/num-integer/latest/num_integer/
// - https://kagcc.hatenablog.com/entry/2019/12/12/221811

use std::ops::{Add, Div, Mul, Rem, Sub};

pub trait Integer<Rhs = Self, Output = Self>:
    Sized
    + Copy
    + Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
    fn gcd(&self, n: &Self) -> Self;
}

pub fn gcd<T: Integer>(x: T, y: T) -> T {
    x.gcd(&y)
}

pub fn lcm<T: Integer>(x: T, y: T) -> T {
    let g = x.gcd(&y);
    (x / g) * (y / g) * g
}

// isize 型に Integer trait を実装する
macro_rules! impl_integer_for_isize {
    ($T:ty,$test_mod:ident) => {
        impl Integer for $T {
            fn gcd(&self, other: &$T) -> $T {
                let n = (*self).abs();
                let m = (*other).abs();
                let (mut a, mut b) = if n > m { (n, m) } else { (m, n) };
                if a == 0 || b == 0 {
                    return (a | b).abs();
                }
                while b != 0 {
                    let tmp = b;
                    b = a % b;
                    a = tmp;
                }
                a
            }
        }

        #[cfg(test)]
        mod $test_mod {
            use crate::Integer;
            #[test]
            fn test_gcd() {
                assert_eq!((12 as $T).gcd(&8), 4 as $T);
                assert_eq!((0 as $T).gcd(&3), 3 as $T);
                assert_eq!((3 as $T).gcd(&3), 3 as $T);
                assert_eq!((3 as $T).gcd(&-3), 3 as $T);
                assert_eq!((-4 as $T).gcd(&-2), 2 as $T);
            }
        }
    };
}
impl_integer_for_isize!(i8, test_integer_i8);
impl_integer_for_isize!(i32, test_integer_i32);
impl_integer_for_isize!(i64, test_integer_i64);
impl_integer_for_isize!(isize, test_integer_isize);

// usize 型に Integer trait を実装する
macro_rules! impl_integer_for_usize {
    ($T:ty,$test_mod:ident) => {
        impl Integer for $T {
            fn gcd(&self, other: &$T) -> $T {
                let n = *self;
                let m = *other;
                let (mut a, mut b) = if n > m { (n, m) } else { (m, n) };
                if a == 0 || b == 0 {
                    return a | b;
                }
                while b != 0 {
                    let tmp = b;
                    b = a % b;
                    a = tmp;
                }
                a
            }
        }

        #[cfg(test)]
        mod $test_mod {
            use crate::Integer;
            #[test]
            fn test_gcd() {
                assert_eq!((12 as $T).gcd(&8), 4 as $T);
                assert_eq!((0 as $T).gcd(&3), 3 as $T);
                assert_eq!((3 as $T).gcd(&3), 3 as $T);
            }
        }
    };
}
impl_integer_for_usize!(u8, test_integer_u8);
impl_integer_for_usize!(u32, test_integer_u32);
impl_integer_for_usize!(u64, test_integer_u64);
impl_integer_for_usize!(usize, test_integer_usize);

