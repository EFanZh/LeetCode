pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let to_score = |&x| x * 2 - 1;
        let sum = possible.iter().map(to_score).sum::<i32>();
        let mut acc = 0;

        (1..)
            .zip(possible[..possible.len() - 1].iter().map(to_score))
            .find_map(|(i, score)| {
                acc += score;

                (acc * 2 > sum).then_some(i)
            })
            .unwrap_or(-1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_levels(possible: Vec<i32>) -> i32 {
        Self::minimum_levels(possible)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
