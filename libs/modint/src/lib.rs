#![allow(non_snake_case)]
//! modint 関連のライブラリ

#[macro_export]
macro_rules! mint {
    ($num:expr) => {
        Mint::new($num)
    };
    () => {
        Mint::new(0)
    };
}
