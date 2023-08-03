#[derive(Debug, Clone)]
struct Comb {
    fact: Vec<Mod<MOD>>,
    inv_fact: Vec<Mod<MOD>>
}

#[allow(dead_code)]
impl Comb {
    fn new() -> Self {
        let mut comb = Self {
            fact: vec![Mod::new(0); MAX_LIM as usize],
            inv_fact: vec![Mod::new(0); MAX_LIM as usize]
        };
        Self::init(&mut comb);
        comb
    }

    fn init(comb: &mut Self) {
        for i in 2..MAX_LIM as i64 {
            comb.fact[i as usize] = comb.fact[i as usize - 1] * Mod::new(i);
            comb.inv_fact[i as usize] = Mod::new(1) / comb.fact[i as usize];
        }
    }

    fn c(&self, n: i64, r: i64) -> Mod<MOD> {
        if n < 0 || r < 0 || r > n {
            return Mod::new(0);
        }
        self.fact[n as usize] *
            self.inv_fact[r as usize] * self.inv_fact[(n - r) as usize]
    }

    fn bin_exp(mut a: Mod<MOD>, mut b: i64) -> Mod<MOD> {
        let mut res: Mod<MOD> = Mod::new(1);
        while b > 0 {
            if (b & 1) == 1 {
                res *= a;
            }
            a *= a;
            b >>= 1;
        }
        res
    }
}