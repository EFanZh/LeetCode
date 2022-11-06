pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut result = 0;

        for (i, flip) in (1..).zip(flips) {
            max = max.max(flip as u32);

            result += i32::from(max == i);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        Self::num_times_all_blue(flips)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
