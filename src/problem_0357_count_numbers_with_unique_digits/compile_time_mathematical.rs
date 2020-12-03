pub struct Solution;

impl Solution {
    const fn get_results() -> [i32; 9] {
        let mut result = [1; 9];
        let mut total = 0;
        let mut premutations = 1;
        let mut i = 1;

        while i != 9 {
            total += premutations;
            result[i] = total * 9 + 1;
            premutations *= 10 - i as i32;
            i += 1;
        }

        result
    }

    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        const RESULTS: [i32; 9] = Solution::get_results();

        RESULTS[n as usize]
    }
}

impl super::Solution for Solution {
    fn count_numbers_with_unique_digits(n: i32) -> i32 {
        Self::count_numbers_with_unique_digits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
