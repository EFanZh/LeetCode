pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_factors(x: i32, f: impl FnOnce(u16)) {
        f(match x {
            1 => 0b_0000000000,
            2 => 0b_0000000001,
            3 => 0b_0000000010,
            5 => 0b_0000000100,
            6 => 0b_0000000011,
            7 => 0b_0000001000,
            10 => 0b_0000000101,
            11 => 0b_0000010000,
            13 => 0b_0000100000,
            14 => 0b_0000001001,
            15 => 0b_0000000110,
            17 => 0b_0001000000,
            19 => 0b_0010000000,
            21 => 0b_0000001010,
            22 => 0b_0000010001,
            23 => 0b_0100000000,
            26 => 0b_0000100001,
            29 => 0b_1000000000,
            30 => 0b_0000000111,
            _ => return,
        });
    }

    fn assign_add(target: &mut u32, value: u32) {
        *target += value;
        *target = target.checked_sub(1_000_000_007).unwrap_or(*target);
    }

    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let mut cache = [0_u32; 1024];

        cache[0] = 1;

        for num in nums {
            Self::get_factors(num, |factors| {
                let extra_bits = 1023 ^ factors;
                let mut state = extra_bits;

                loop {
                    let extra_count = cache[usize::from(state)];

                    Self::assign_add(&mut cache[usize::from(factors | state)], extra_count);

                    if state == 0 {
                        break;
                    }

                    state = (state - 1) & extra_bits;
                }
            });
        }

        let mut result = 0;

        for &count in &cache[1..] {
            Self::assign_add(&mut result, count);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        Self::number_of_good_subsets(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
