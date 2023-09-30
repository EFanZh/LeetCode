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
        let val = val as u32;

        self.values.push(
            (u64::from(val + (Self::MODULUS as u32 - self.inc)) * Self::mod_inv(u64::from(self.mul)) % Self::MODULUS)
                as _,
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
        let m = m as u32;

        self.mul = (u64::from(self.mul) * u64::from(m) % Self::MODULUS) as _;
        self.inc = (u64::from(self.inc) * u64::from(m) % Self::MODULUS) as _;
    }

    fn get_index(&mut self, idx: i32) -> i32 {
        self.values.get(idx as u32 as usize).map_or(-1, |&value| {
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
