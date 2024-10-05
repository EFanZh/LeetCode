pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn combinations(n: u8, mut k: u8, base: u16, f: &mut impl FnMut(u16)) {
        match k {
            0 => f(base),
            1 => {
                let mut probe = 1;

                for _ in 0..n {
                    f(base + probe);

                    probe *= 3;
                }
            }
            _ => {
                let mut start = k / 2;
                let mut probe = u16::pow(3, u32::from(start));

                k -= 1;

                for i in start..n {
                    Self::combinations(i, k, base + probe, f);

                    probe *= 3;
                }

                start = k / 2;
                probe = u16::pow(3, u32::from(start)) * 2;
                k -= 1;

                for i in start..n {
                    Self::combinations(i, k, base + probe, f);

                    probe *= 3;
                }
            }
        }
    }

    pub fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32 {
        let n = nums.len() as u8;
        let num_slots = num_slots as u32 as usize;
        let configurations = u16::pow(3, num_slots as _);
        let mut cache = vec![0_u8; usize::from(configurations)].into_boxed_slice();

        (1..).zip(&nums).for_each(|(count, num)| {
            let num = *num as u8;

            Self::combinations(num_slots as _, count, 0, &mut |configuration| {
                let mut iter = configuration;
                let mut slot = 1;
                let mut probe = 1;
                let mut max = 0;

                loop {
                    let digit = iter % 3;

                    iter /= 3;

                    if digit != 0 {
                        max = max.max(cache[usize::from(configuration - probe)] + (slot & num));
                    }

                    if iter == 0 {
                        break;
                    }

                    slot += 1;
                    probe *= 3;
                }

                cache[usize::from(configuration)] = max;
            });
        });

        let mut result = 0;

        Self::combinations(num_slots as _, n, 0, &mut |configuration| {
            result = result.max(cache[usize::from(configuration)]);
        });

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32 {
        Self::maximum_and_sum(nums, num_slots)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
