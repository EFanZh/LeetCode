// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Fancy {
    values: Vec<u32>,
    mul: u32,
    inc: u32,
}

impl Fancy {
    const MODULUS: u64 = 1_000_000_007;

    fn new() -> Self {
        Self {
            values: Vec::new(),
            mul: 1,
            inc: 0,
        }
    }

    fn exp_mod(mut base: u64, mut exponent: u64) -> u64 {
        let mut result = 1;

        while exponent != 0 {
            if exponent & 1 != 0 {
                result = (result * base) % Self::MODULUS;
            }

            base = (base * base) % Self::MODULUS;

            exponent >>= 1;
        }

        result
    }

    fn mod_inv(x: u64) -> u64 {
        Self::exp_mod(x, Self::MODULUS - 2)
    }

    fn append(&mut self, val: i32) {
        let val = u64::from(val as u32);

        self.values.push(
            ((val + (Self::MODULUS - u64::from(self.inc))) * Self::mod_inv(u64::from(self.mul)) % Self::MODULUS) as _,
        );
    }

    fn add_all(&mut self, inc: i32) {
        let modulus = Self::MODULUS as _;
        let inc = inc as u32;

        self.inc += inc;

        if self.inc >= modulus {
            self.inc -= modulus;
        }
    }

    fn mult_all(&mut self, m: i32) {
        let m = u64::from(m as u32);

        self.mul = (u64::from(self.mul) * m % Self::MODULUS) as _;
        self.inc = (u64::from(self.inc) * m % Self::MODULUS) as _;
    }

    fn get_index(&mut self, idx: i32) -> i32 {
        let idx = idx as u32 as usize;

        self.values.get(idx).map_or(-1, |&value| {
            ((u64::from(value) * u64::from(self.mul) + u64::from(self.inc)) % Self::MODULUS) as _
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Fancy for Fancy {
    fn new() -> Self {
        Self::new()
    }

    fn append(&mut self, val: i32) {
        self.append(val);
    }

    fn add_all(&mut self, inc: i32) {
        self.add_all(inc);
    }

    fn mult_all(&mut self, m: i32) {
        self.mult_all(m);
    }

    fn get_index(&mut self, idx: i32) -> i32 {
        self.get_index(idx)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Fancy>();
    }
}
