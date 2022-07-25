pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn is_perfect_square(value: u32) -> bool {
        u32::pow(f64::from(value).sqrt() as _, 2) == value
    }

    fn shift_left(first: &mut i32, rest: &mut [i32]) {
        let saved_first = *first;
        let mut prev = first;

        for value in rest {
            *prev = *value;
            prev = value;
        }

        *prev = saved_first;
    }

    fn dfs(first: i32, rest: &mut [i32], result: &mut i32) {
        if let Some((second, rest)) = rest.split_first_mut() {
            if Self::is_perfect_square((first + *second) as _) {
                Self::dfs(*second, rest, result);
            }

            for i in 0..rest.len() {
                let candidate = &mut rest[i];

                if *candidate != *second {
                    mem::swap(second, candidate);

                    if Self::is_perfect_square((first + *second) as _) {
                        Self::dfs(*second, rest, result);
                    }
                }
            }

            Self::shift_left(second, rest);
        } else {
            *result += 1;
        }
    }

    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut result = 0;
        let (first, rest) = nums.split_first_mut().unwrap();
        let mut first = *first;

        Self::dfs(first, rest, &mut result);

        for i in 0..rest.len() {
            let candidate = &mut rest[i];

            if *candidate != first {
                mem::swap(&mut first, candidate);
                Self::dfs(first, rest, &mut result);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        Self::num_squareful_perms(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
