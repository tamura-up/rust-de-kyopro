//! 64 ビット 8*8 のビットボードを操作する関数をまとめています
//! 
//! ## 参考資料
//! + https://qiita.com/ysuzuk81/items/453b08a14d23fb8c6c11
//! + https://qiita.com/rimol/items/1f70b4063500f18ad75a
#![allow(non_snake_case)]

/// Delta Swap
/// 2組の部分列を入れ替える
/// https://qiita.com/rimol/items/1f70b4063500f18ad75a
///
/// ## Params
/// + `x`: bit 列
/// + `mask`: 下位に位置する部分列を 1 で埋めた mask
/// + `delta`: `mask << delta` が上位の部分列の位置を示すような `delta`
pub fn delta_swap(x: u64, mask: u64, delta: usize) -> u64 {
    let t = ((x >> delta) ^ x) & mask;
    (t ^ (t << delta)) ^ x
}

/// 水平反転
pub fn flip_horizontal(x: u64) -> u64 {
    let x = delta_swap(x, 0x5555555555555555, 1);
    let x = delta_swap(x, 0x3333333333333333, 2);
    return delta_swap(x, 0x0F0F0F0F0F0F0F0F, 4);
}
/// 垂直反転
pub fn flip_vertical(x: u64) -> u64 {
    let x = delta_swap(x, 0x00FF00FF00FF00FF, 8);
    let x = delta_swap(x, 0x0000FFFF0000FFFF, 16);
    return delta_swap(x, 0x00000000FFFFFFFF, 32);
}
/// 対角線(左上から右下) を軸に反転
pub fn flip_diagonal_A1H8(x: u64) -> u64 {
    let x = delta_swap(x, 0x00AA00AA00AA00AA, 7);
    let x = delta_swap(x, 0x0000CCCC0000CCCC, 14);
    return delta_swap(x, 0x00000000F0F0F0F0, 28);
}

/// 対角線(右上から左下) を軸に反転
pub fn flip_diagonal_A8H1(x: u64) -> u64 {
    let x = delta_swap(x, 0x0055005500550055, 9);
    let x = delta_swap(x, 0x0000333300003333, 18);
    return delta_swap(x, 0x000000000F0F0F0F, 36);
}

/// 時計まわりに 90 度回転
pub fn rotate_clockwise90(x: u64) -> u64 {
    flip_horizontal(flip_diagonal_A1H8(x))
}
/// 反時計まわりに 90 度回転
pub fn rotate_counter_clockwise90(x: u64) -> u64 {
    flip_vertical(flip_diagonal_A1H8(x))
}
/// 180 度回転
pub fn rotate180(x: u64) -> u64 {
    flip_vertical(flip_horizontal(x))
}

#[test]
fn test_delta_swap() {
    let x = 0b110011u64;
    assert_eq!(delta_swap(x, 0b111, 3), 0b011110);
    assert_eq!(delta_swap(x, 0b11, 3), 0b111010);
    assert_eq!(delta_swap(x, 0b01, 2), 0b110110);
}

#[test]
fn test_flip_horizontal() {
    let x = 0b11000000_00110000_00001100_00000011_11000000_00110000_00001100_00000011u64;
    let y = 0b00000011_00001100_00110000_11000000_00000011_00001100_00110000_11000000u64;
    assert_eq!(flip_horizontal(x), y);
}
#[test]
fn test_flip_vertical() {
    let x = 0b11000000_00110000_00001100_00000011_11000000_00110000_00001100_00000011u64;
    let y = 0b00000011_00001100_00110000_11000000_00000011_00001100_00110000_11000000u64;
    assert_eq!(flip_vertical(x), y);
}
#[test]
fn test_diagonal_A1H8() {
    let x = 0b11000000_00110000_00001100_00000011_11000000_00110000_00001100_00000011u64;
    let y = 0b10001000_10001000_01000100_01000100_00100010_00100010_00010001_00010001u64;
    assert_eq!(flip_diagonal_A1H8(x), y);
}
#[test]
fn test_diagonal_A8H1() {
    let x = 0b11000000_00110000_00001100_00000011_11000000_00110000_00001100_00000011u64;
    let y = 0b10001000_10001000_01000100_01000100_00100010_00100010_00010001_00010001u64;
    assert_eq!(flip_diagonal_A8H1(x), y);
}
#[test]
fn test_rotate_clockwise90() {
    let x = 0b11000000_00110000_00001100_00000011_11000000_00110000_00001100_00000011u64;
    let y = 0b00010001_00010001_00100010_00100010_01000100_01000100_10001000_10001000u64;
    assert_eq!(rotate_clockwise90(x), y);
}
#[test]
fn test_rotate_counter_clockwise90() {
    let x = 0b11000000_00110000_00001100_00000011_11000000_00110000_00001100_00000011u64;
    let y = rotate_clockwise90(rotate_clockwise90(rotate_clockwise90(x)));
    assert_eq!(rotate_counter_clockwise90(x), y);
}
#[test]
fn test_rotate180() {
    let x = 0b11000000_00110000_00001100_00000011_11000000_00110000_00001100_00000011u64;
    let y = rotate_clockwise90(rotate_clockwise90(x));
    assert_eq!(rotate180(x), y);
}

/// bitboard を出力します
pub fn print_bitboard(x: u64) {
    for i in 0..8 {
        let v = x >> ((8 - i - 1) * 8) & 0xFF;
        println!("{:08b}", v);
    }
}
