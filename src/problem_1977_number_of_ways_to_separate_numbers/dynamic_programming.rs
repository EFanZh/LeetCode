pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_combinations(num: String) -> i32 {
        const MODULUS: u32 = 1_000_000_007;

        let num = num.as_bytes();
        let n = num.len();
        let n_minus_1 = n - 1;
        let mut longest_common_prefixes = vec![0_u16; n_minus_1 * n_minus_1].into_boxed_slice();
        let mut cache = vec![0_u32; n * n].into_boxed_slice();

        // Compute longest common prefixes.

        let mut i = n;

        loop {
            i = i.wrapping_sub(1);

            if let Some(&left) = num.get(i) {
                let mut j = i;

                loop {
                    j += 1;

                    if let Some(&right) = num.get(j) {
                        if left == right {
                            let index = n_minus_1 * i + (j - 1);

                            longest_common_prefixes[index] =
                                1 + longest_common_prefixes.get(index + n).copied().unwrap_or(0);
                        }
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        // Dynamic programming.

        for (start, &c) in num.iter().enumerate().rev() {
            if c != b'0' {
                let max_length = n - start;
                let mut count = 1_u32;

                cache[start * n + (max_length - 1)] = count;

                for length in (1..max_length).rev() {
                    let first_end = start + length;
                    let second_end = first_end + length;

                    if second_end <= n {
                        let mut index = first_end * n + length;

                        let longest_common_prefix =
                            usize::from(longest_common_prefixes[n_minus_1 * start + start + length - 1]);

                        if longest_common_prefix >= length
                            || num[start + longest_common_prefix] < num[start + length + longest_common_prefix]
                        {
                            index -= 1;
                        }

                        count += cache[index];
                        count = count.checked_sub(MODULUS).unwrap_or(count);
                    }

                    cache[start * n + (length - 1)] = count;
                }
            }
        }

        *cache.first().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_combinations(num: String) -> i32 {
        Self::number_of_combinations(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
