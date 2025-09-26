pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn factorize(mut x: u32) -> (u32, u32, u32) {
        let twos = x.trailing_zeros();
        let mut fives = 0;

        x >>= twos;

        while x.is_multiple_of(5) {
            x /= 5;
            fives += 1;
        }

        (x, twos, fives)
    }

    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len() as u32 - 1;
        let mut result = 0;
        let mut factor = 1;
        let mut twos = 0;
        let mut fives = 0;
        let mut i = 0;

        for num in nums {
            let real_factor = if fives == 0 {
                factor
                    * (if twos == 0 {
                        1
                    } else {
                        [6, 2, 4, 8][(twos % 4) as usize]
                    })
            } else if twos == 0 {
                5
            } else {
                0
            };

            result = (result + (num as u32) * real_factor) % 10;

            if i == n {
                break;
            }

            let (mul_factors, add_twos, add_fives) = Self::factorize(n - i);

            factor = (factor * mul_factors) % 10;
            twos += add_twos;
            fives += add_fives;

            i += 1;

            let (div_factor, sub_twos, sub_fives) = Self::factorize(i);
            let mod_inverse = ((div_factor * div_factor) % 10) * div_factor;

            factor = (factor * mod_inverse) % 10;
            twos -= sub_twos;
            fives -= sub_fives;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn triangular_sum(nums: Vec<i32>) -> i32 {
        Self::triangular_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
