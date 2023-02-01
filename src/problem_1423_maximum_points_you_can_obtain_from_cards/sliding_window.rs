pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let tail = &card_points[card_points.len() - k as usize..];
        let mut sum = tail.iter().sum::<i32>();
        let mut result = sum;

        for (&new, &old) in card_points.iter().zip(tail) {
            sum += new - old;
            result = result.max(sum);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        Self::max_score(card_points, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
