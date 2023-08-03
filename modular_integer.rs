#[derive(Debug, Clone, Copy)]
struct Mod<const MOD: i64> {
    val: i64
}

impl Mod<MOD> {
    fn new(x: i64) -> Self {
        let mut val: i64 = x % MOD;
        if val < 0 {
            val += MOD;
        }
        Self { val }
    }
}

impl Add for Mod<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut lhs: Self = Mod::new(self.val);
        lhs += rhs;
        lhs
    }
}

impl Sub for Mod<MOD> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut lhs: Self = Mod::new(self.val);
        lhs -= rhs;
        lhs
    }
}

impl Mul for Mod<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut lhs: Self = Mod::new(self.val);
        lhs *= rhs;
        lhs
    }
}

impl Div for Mod<MOD> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut lhs: Self = Mod::new(self.val);
        lhs /= rhs;
        lhs
    }
}

impl AddAssign for Mod<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        self.val += rhs.val;
        if self.val >= MOD {
            self.val -= MOD;
        }
    }
}

impl SubAssign for Mod<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        self.val -= rhs.val;
        if self.val < 0 {
            self.val += MOD;
        }
    }
}

impl MulAssign for Mod<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        self.val = (self.val * rhs.val) % MOD;
    }
}

impl DivAssign for Mod<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        let (mut a, mut b, mut c): (i64, i64, i64) = (0, 1, rhs.val);
        let mut m: i64 = MOD;
        while c != 0 {
            let t: i64 = m / c;
            m -= t * c;
            swap(&mut c, &mut m);
            a -= t * b;
            swap(&mut a, &mut b);
        }
        a %= MOD;
        if a < 0 {
            a += MOD;
        }
        self.val = (self.val * a) % MOD;
    }
}

impl Neg for Mod<MOD> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Mod::new(-self.val)
    }
}

impl PartialEq for Mod<MOD> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl PartialOrd for Mod<MOD> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }

    fn lt(&self, other: &Self) -> bool {
        self.val.lt(&other.val)
    }

    fn le(&self, other: &Self) -> bool {
        self.val.le(&other.val)
    }

    fn gt(&self, other: &Self) -> bool {
        self.val.gt(&other.val)
    }

    fn ge(&self, other: &Self) -> bool {
        self.val.ge(&other.val)
    }
}