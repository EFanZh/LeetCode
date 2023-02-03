pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let threshold = candies.iter().copied().max().unwrap() - extra_candies;

        candies.into_iter().map(|x| x >= threshold).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        Self::kids_with_candies(candies, extra_candies)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
