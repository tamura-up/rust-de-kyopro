//! 入力を受け取るマクロを定義します
#![allow(unused_macros)]

/// src: https://yukicoder.me/submissions/892593
/// ## examples
/// let x = get!(i32); // 1行の i32 の入力を受け取る
/// let x = get!(i32;2); // 2行の i32 の入力を受け取る
/// let lst = get!{i32;;}; // 1行に入力された配列を受け取る
/// let x = get!(i32,i32,i32); // (i32, i32, i32 )のタプルを受け取る
/// let x = get!(i32,i32,i32;2); // 2行 (i32, i32, i32 )のタプルを受け取る

#[macro_export]
macro_rules! get {
      ($t:ty) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.trim().parse::<$t>().unwrap()
          }
      };
      ($($t:ty),*) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              let mut iter = line.split_whitespace();
              (
                  $(iter.next().unwrap().parse::<$t>().unwrap(),)*
              )
          }
      };
      ($t:ty; $n:expr) => {
          (0..$n).map(|_|
              get!($t)
          ).collect::<Vec<_>>()
      };
      ($($t:ty),*; $n:expr) => {
          (0..$n).map(|_|
              get!($($t),*)
          ).collect::<Vec<_>>()
      };
      ($t:ty ;;) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.split_whitespace()
                  .map(|t| t.parse::<$t>().unwrap())
                  .collect::<Vec<_>>()
          }
      };
      ($t:ty ;; $n:expr) => {
          (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
      };
}
