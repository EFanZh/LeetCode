// ------------------------------------------------------ snip ------------------------------------------------------ //

struct State {
    val: u32,
    acc_index: u32,
    acc_mul: u32,
    acc_inc: u32,
}

pub struct Fancy {
    states: Vec<State>,
    mul: u32,
    inc: u32,
    stack_buffer: Vec<u32>,
}

impl Fancy {
    const MODULUS: u64 = 1_000_000_007;

    fn new() -> Self {
        Self {
            states: Vec::new(),
            mul: 1,
            inc: 0,
            stack_buffer: Vec::new(),
        }
    }

    fn append(&mut self, val: i32) {
        let index = self.states.len() as u32 + 1;

        self.states.push(State {
            val: val as _,
            acc_index: index,
            acc_mul: self.mul,
            acc_inc: self.inc,
        });

        self.mul = 1;
        self.inc = 0;
    }

    fn add_all(&mut self, inc: i32) {
        let modulus = Self::MODULUS as _;

        self.inc += inc as u32;

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
        let states = self.states.as_mut_slice();
        let latest_acc_index = states.len() as _;
        let idx = idx as u32 as usize;

        if let Some(value) = states.get(idx).map(|state| u64::from(state.val)) {
            let mut op_index = idx + 1;

            let result = if let Some(mut state) = states.get_mut(op_index) {
                let stack = &mut self.stack_buffer;
                let mut acc_mul;
                let mut acc_inc;

                'outer: loop {
                    if state.acc_index < latest_acc_index {
                        stack.push(op_index as _);

                        op_index = state.acc_index as _;
                        state = &mut states[op_index];
                    } else {
                        loop {
                            acc_mul = u64::from(state.acc_mul);
                            acc_inc = u64::from(state.acc_inc);

                            if let Some(top_index) = stack.pop() {
                                op_index = top_index as _;
                                state = &mut states[op_index];

                                state.acc_index = latest_acc_index;
                                state.acc_mul = (u64::from(state.acc_mul) * acc_mul % Self::MODULUS) as _;
                                state.acc_inc = ((u64::from(state.acc_inc) * acc_mul + acc_inc) % Self::MODULUS) as _;
                            } else {
                                break 'outer;
                            }
                        }
                    }
                }

                (value * acc_mul + acc_inc) % Self::MODULUS
            } else {
                value
            };

            ((result * u64::from(self.mul) + u64::from(self.inc)) % Self::MODULUS) as _
        } else {
            -1
        }
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
