// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Fancy {
    values: Vec<u32>,
    mul: u64,
    inc: u64,
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

    // See <https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures>.
    fn mod_inv(x: u64) -> u64 {
        let mut state = (Self::MODULUS, x, 0_i64, 1_i64);

        while state.1 != 0 {
            let quotient = state.0 / state.1;

            state = (
                state.1,
                state.0 - state.1 * quotient,
                state.3,
                state.2 - state.3 * quotient as i64,
            );
        }

        if state.2 < 0 {
            state.2 += Self::MODULUS as i64;
        }

        state.2 as _
    }

    fn append(&mut self, val: i32) {
        let val = u64::from(val as u32);

        self.values
            .push(((val + (Self::MODULUS - self.inc)) * Self::mod_inv(self.mul) % Self::MODULUS) as _);
    }

    fn add_all(&mut self, inc: i32) {
        let inc = u64::from(inc as u32);

        self.inc += inc;

        if self.inc >= Self::MODULUS {
            self.inc -= Self::MODULUS;
        }
    }

    fn mult_all(&mut self, m: i32) {
        let m = u64::from(m as u32);

        self.mul = self.mul * m % Self::MODULUS;
        self.inc = self.inc * m % Self::MODULUS;
    }

    fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as u32 as usize;

        self.values.get(idx).map_or(-1, |&value| {
            ((u64::from(value) * self.mul + self.inc) % Self::MODULUS) as _
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
        (*self).get_index(idx)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Fancy>();
    }
}
