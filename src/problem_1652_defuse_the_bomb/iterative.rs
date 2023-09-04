pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            let mut result = code;

            result.fill(0);

            result
        } else {
            let n = code.len();

            if k < 0 {
                let k = (-k) as u32 as usize;
                let mut sum = code[n - k..].iter().sum::<i32>();

                code[n - k..]
                    .iter()
                    .chain(&code)
                    .zip(&code)
                    .map(|(old, new)| {
                        let new_sum = sum - old + new;

                        mem::replace(&mut sum, new_sum)
                    })
                    .collect()
            } else {
                let k = k as u32 as usize;
                let mut sum = code[..k].iter().sum::<i32>();

                code.iter()
                    .zip(code[k..].iter().chain(&code))
                    .map(|(old, new)| {
                        sum -= old;
                        sum += new;

                        sum
                    })
                    .collect()
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        Self::decrypt(code, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
