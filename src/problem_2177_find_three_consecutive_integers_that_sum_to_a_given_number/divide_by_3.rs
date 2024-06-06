pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        let num = num as u64;

        if num % 3 == 0 {
            let middle = (num / 3) as i64;

            vec![middle - 1, middle, middle + 1]
        } else {
            Vec::new()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_three(num: i64) -> Vec<i64> {
        Self::sum_of_three(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
