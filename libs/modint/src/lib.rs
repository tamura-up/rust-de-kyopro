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

/// modint の Comb 構造体を定義するマクロ。
/// ac_library の ModInt を Mint として use しているモジュール内で実行してください。
/// 
/// # examples
/// ```
/// use ac_library::ModInt1000000007 as Mint; // 必須！！！
/// use kyopro_modint::define_mint_comb;
///
/// define_mint_comb!();
///
/// fn n_choose_m() {
///     let cmb = Comb::new(100);
///     assert_eq!(cmb.nCm(1, 0).val(), 1);
/// }
/// ```
#[macro_export]
macro_rules! define_mint_comb {
    () => {
        struct Comb {
            fact: Vec<Mint>,
            inv: Vec<Mint>,
        }
        impl Comb {
            fn new(N: usize) -> Comb {
                let mut kaijo = vec![Mint::new(0); N + 1];
                let mut gyaku = vec![Mint::new(0); N + 1];
                kaijo[0] = Mint::new(1);
                for i in 1..=N {
                    kaijo[i] = kaijo[i - 1] * i;
                }
                gyaku[N] = kaijo[N].inv();
                for i in (0..N).rev() {
                    gyaku[i] = gyaku[i + 1] * (i + 1);
                }
                Comb {
                    fact: kaijo,
                    inv: gyaku,
                }
            }
            fn nCm(&self, n: usize, m: usize) -> Mint {
                if n < m {
                    return Mint::new(0);
                }
                assert!(n < self.fact.len());
                assert!(m < self.inv.len());
                self.fact[n] * self.inv[n - m] * self.inv[m]
            }
            fn nPm(&self, n: usize, m: usize) -> Mint {
                if n < m {
                    return Mint::new(0);
                }
                assert!(n < self.fact.len());
                assert!(m < self.inv.len());
                self.fact[n] * self.inv[n - m]
            }
            fn nHm(&self, n: usize, m: usize) -> Mint {
                self.nCm(n + m - 1, n - 1)
            }
        }
    };
}

#[cfg(test)]
mod test_combination {
    use ac_library::ModInt1000000007 as Mint;
    define_mint_comb!();

    #[test]
    fn test_n_choose_m() {
        let cmb = Comb::new(100);
        assert_eq!(cmb.nCm(1, 0).val(), 1);
        assert_eq!(cmb.nCm(100, 0).val(), 1);
        assert_eq!(cmb.nCm(100, 100).val(), 1);
        assert_eq!(cmb.nCm(100, 10).val(), 309335270); // (100 choose 10) % 1000000007 = 309335270
        assert_eq!(cmb.nCm(50, 15).val(), 829559370); // (50 choose 15) % 1000000007 = 829559370
    }

    #[test]
    fn test_n_permutations_m() {
        let cmb = Comb::new(100);
        assert_eq!(cmb.nCm(100, 1).val(), 100);
        let m = 1000000007usize;
        let mut x = 1usize;
        for i in 0..10 {
            x *= 100 - i;
            x %= m;
        }
        assert_eq!(cmb.nPm(100, 10).val(), x as u32);
    }
    #[test]
    fn test_nHm() {
        let cmb = Comb::new(100);
        assert_eq!(cmb.nHm(50, 5).val(), cmb.nCm(50 + 5 - 1, 50 - 1).val());
    }
}
