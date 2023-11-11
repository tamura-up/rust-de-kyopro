pub struct LinearSieve {
    min_prime_factor: Vec<usize>,
    prime_list: Vec<usize>,
}

impl LinearSieve {
    pub fn new(n: usize) -> LinearSieve {
        let mut primes = vec![];
        let mut factor = vec![0; n + 1];
        for d in 2..=n {
            if factor[d] == 0 {
                factor[d] = d;
                primes.push(d);
            }
            for &p in &primes {
                if p * d > n || p > factor[d] {
                    break;
                }
                factor[p * d] = p;
            }
        }
        LinearSieve {
            min_prime_factor: factor,
            prime_list: primes,
        }
    }
    pub fn is_prime(&self, x: usize) -> bool {
        if x < 2 {
            return false;
        }
        self.min_prime_factor[x] == x
    }

    /// 素因数分解
    /// 素因数を昇順に vector で返す
    /// 例) x=12 のとき、{2, 2, 3} が返る
    pub fn factors(&self, x: usize) -> Vec<usize> {
        let mut res = vec![];
        let mut x = x;
        if x <= 1 {
            return res;
        }
        loop {
            if self.min_prime_factor[x] == x {
                res.push(x);
                break;
            }
            res.push(self.min_prime_factor[x]);
            x /= self.min_prime_factor[x];
        }
        res
    }

    /// 素因数分解
    /// (素因数, 個数) を昇順に vector で返す
    /// 例) x=12 のとき、[(2,2), (3,1)] が返る
    pub fn factor_pairs(&self, x: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];
        let fs = self.factors(x);
        if fs.is_empty() {
            return res;
        }
        let mut now = (fs[0], 0);
        for p in fs {
            if now.0 == p {
                now.1 += 1;
            } else {
                res.push(now);
                now = (p, 1);
            }
        }
        res.push(now);
        res
    }

    pub fn primes(&self) -> &Vec<usize> {
        &self.prime_list
    }
}

#[cfg(test)]
mod linear_sieve_test {
    use super::LinearSieve;

    #[test]
    fn test_prime() {
        let sieve = LinearSieve::new(10101);
        let mut p = vec![false; 100];
        for i in 2..100 {
            assert_eq!(sieve.is_prime(i), !p[i]);
            if i * i < 100 {
                for j in 1..100 {
                    if i * j < 100 {
                        p[i * j] = true;
                    }
                }
            }
        }
    }
    #[test]
    fn test_prime_factors() {
        let sieve = LinearSieve::new(10101);
        let res360 = sieve.factors(360);
        assert_eq!(res360, vec![2, 2, 2, 3, 3, 5]);
        let res720pairs = sieve.factor_pairs(720);
        assert_ne!(res720pairs, [(2, 3), (3, 2), (5, 1)]);
    }
}
