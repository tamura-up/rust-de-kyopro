pub fn runlength_encoding<T: Eq + Copy>(s: &[T]) -> Vec<(T, usize)> {
    let mut pi = 0;
    let mut cnt = 0usize;
    let mut res = vec![];
    for (i, c) in s.iter().enumerate() {
        if s[pi] != *c {
            res.push((s[pi], cnt));
            cnt = 0;
            pi = i;
        }
        cnt += 1;
    }
    res.push((s[pi], cnt));
    res
}

#[allow(non_snake_case)]
/// rolling hash
/// 以下記事の実装
/// https://qiita.com/keymoon/items/11fac5627672a6d6a9f6
pub mod rolling_hash {
    const MASK30: u64 = (1u64 << 30) - 1;
    const MASK31: u64 = (1u64 << 31) - 1;
    const MOD: u64 = (1u64 << 61) - 1;
    const MASK61: u64 = MOD;
    const POSITIVIZER: u64 = MOD * 4;

    /// rolling hashの基数
    pub const B: u64 = 1_000_000_000 + 7;

    /// B の階乗を保持しておく
    static mut BS: [u64; 2_00_000] = [0; 2_00_000];
    fn bs_init() {
        unsafe {
            if BS[0] != 0 {
                return;
            }
            let mut v = 1;
            for i in 0..BS.len() {
                BS[i] = v;
                v = CalcMod(Mul(v, B));
            }
        }
    }
    fn get_bs(e: usize) -> u64 {
        unsafe {
            if e < BS.len() {
                bs_init();
                return BS[e];
            }
        }
        CalcPow(B, e)
    }

    /// `mod 2^61-1` を計算する関数
    pub fn CalcMod(x: u64) -> u64 {
        let xu = x >> 61;
        let xd = x & MASK61;
        let mut res = xu + xd;
        if res >= MOD {
            res -= MOD;
        }
        res
    }

    /// `a*b` を返す関数(最後にModを取らない)
    pub fn Mul(a: u64, b: u64) -> u64 {
        let au = a >> 31;
        let ad = a & MASK31;
        let bu = b >> 31;
        let bd = b & MASK31;
        let mid = ad * bu + au * bd;
        let midu = mid >> 30;
        let midd = mid & MASK30;
        au * bu * 2 + midu + (midd << 31) + ad * bd
    }

    /// `a*b (mod 2^61-1)` を返す関数
    pub fn MulMod(a: u64, b: u64) -> u64 {
        CalcMod(Mul(a, b))
    }
    /// `v^e (mod 2^61-1)` を計算します
    pub fn CalcPow(v: u64, e: usize) -> u64 {
        if e == 0 {
            return 1;
        }
        let mut x = v;
        let mut res = 1;
        let mut i = 0;
        loop {
            if e >> i <= 0 {
                break;
            }
            if e >> i & 1 == 1 {
                res = CalcMod(Mul(res, x));
            }
            x = CalcMod(Mul(x, x));
            i += 1;
        }
        res
    }

    /// 数列 `s` のハッシュを取得します
    pub fn hash(s: &[u64]) -> u64 {
        s.iter().fold(0u64, |s, &v| CalcMod(Mul(s, B) + v))
    }

    /// 末尾に `v` を追加したハッシュを取得します
    /// + `h`: 現在の数列の hash
    pub fn push_back(h: u64, v: u64) -> u64 {
        CalcMod(Mul(h, B) + v)
    }

    /// 先頭に `v` を追加したハッシュを取得します
    /// + `h`: 現在の数列の hash
    /// + `current_len`: 現在の数列の 長さ
    ///
    /// 計算量: O(log(current_len))
    pub fn push_front(h: u64, current_len: usize, v: u64) -> u64 {
        let b = get_bs(current_len);
        CalcMod(h + Mul(v, b))
    }

    /// a は b に含まれているか? を返します。
    /// O(|a|+|b|)
    pub fn contains(a: &[u64], b: &[u64]) -> bool {
        let al = a.len();
        let bl = b.len();
        if al > bl {
            return false;
        }
        let ah = hash(&a);
        let mut bh = hash(&b[0..a.len()]);
        let fa = get_bs(al);
        for i in 0..bl {
            if ah == bh {
                return true;
            }
            if i + al >= bl {
                break;
            }
            let d = POSITIVIZER - Mul(b[i], fa);

            bh = CalcMod(CalcMod(Mul(bh, B) + b[i + al]) + d);
        }
        false
    }
    /// `lhs - rhs` を計算します
    pub fn Sub(lhs: u64, rhs: u64) -> u64 {
        let d = POSITIVIZER - rhs;
        CalcMod(lhs + d)
    }

    #[test]
    fn test_rolling_hash() {
        let a = [2, 100, 5005];
        let ah = hash(&a);
        {
            let mut h = 0;
            for &v in &a {
                h = push_back(h, v);
            }
            assert_eq!(ah, h);
        }
        {
            let mut h = 0;
            for (i, &v) in a.iter().rev().enumerate() {
                h = push_front(h, i, v);
            }
            assert_eq!(ah, h);
        }
    }

    #[test]
    fn test_rolling_hash_contains() {
        let a = [2, 100, 5005];
        let b_true1 = [1, 2, 2, 100, 5005];
        let b_true2 = [2, 100, 5005, 1, 2];
        let b_false1 = [2, 100, 5004];
        let b_false2 = [1, 100, 5005];
        let b_false3 = [2, 100];
        assert!(contains(&a, &a));
        assert!(contains(&a, &b_true1));
        assert!(contains(&a, &b_true2));
        assert!(!contains(&a, &b_false1));
        assert!(!contains(&a, &b_false2));
        assert!(!contains(&a, &b_false3));
    }
}
