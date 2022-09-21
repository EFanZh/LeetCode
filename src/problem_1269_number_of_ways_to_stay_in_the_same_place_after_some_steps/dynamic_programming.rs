pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        const MODULUS: u32 = 1_000_000_007;

        let add = |lhs: u32, rhs: u32| {
            let candidate = lhs + rhs;

            candidate.checked_sub(MODULUS).unwrap_or(candidate)
        };

        let steps = steps as usize;
        let arr_len = arr_len as usize;
        let mut cache = vec![0; steps.min(arr_len)];

        cache[0] = 1;

        for _ in 0..steps {
            let mut prev_1 = 0;
            let mut prev_2 = &mut 0;

            for value in &mut cache {
                let result = add(add(prev_1, *prev_2), *value);

                prev_1 = *prev_2;
                *prev_2 = result;
                prev_2 = value;
            }

            *prev_2 = add(*prev_2, prev_1);
        }

        cache[0] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_ways(steps: i32, arr_len: i32) -> i32 {
        Self::num_ways(steps, arr_len)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
