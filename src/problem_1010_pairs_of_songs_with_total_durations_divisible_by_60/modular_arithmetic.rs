pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut counts = [0_u16; 60];

        for duration in time {
            let duration = duration as u16 % 60;

            result += i32::from(counts[usize::from(if duration == 0 { 0 } else { 60 - duration })]);
            counts[usize::from(duration)] += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        Self::num_pairs_divisible_by60(time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
