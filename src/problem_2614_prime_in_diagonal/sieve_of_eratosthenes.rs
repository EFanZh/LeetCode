pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let n_minus_1 = nums.len() - 1;

        let candidates = nums
            .iter()
            .enumerate()
            .flat_map(|(i, row)| [row[i] as u32, row[n_minus_1 - i] as u32])
            .collect::<Vec<_>>();

        let max = candidates.iter().fold(0, |max, &x| max.max(x));
        let mut states = vec![false; max as usize - 1].into_boxed_slice();

        for i in 2..=max.isqrt() as usize {
            if !states[i - 2] {
                states[i * i - 2..]
                    .iter_mut()
                    .step_by(i)
                    .for_each(|is_composite| *is_composite = true);
            }
        }

        candidates
            .iter()
            .filter(|&&x| {
                states
                    .get((x as usize).wrapping_sub(2))
                    .is_some_and(|&is_composite| !is_composite)
            })
            .fold(0, |max, &x| max.max(x)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        Self::diagonal_prime(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
