pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn has_even_digits(n: i32) -> bool {
        let n = n as u32;

        if n < 1_000 {
            (10..100).contains(&n)
        } else {
            !(10_000..100_000).contains(&n)
        }
    }

    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums {
            if Self::has_even_digits(num) {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_numbers(nums: Vec<i32>) -> i32 {
        Self::find_numbers(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
