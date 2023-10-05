pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut players = scores
            .iter()
            .zip(&ages)
            .map(|(&score, &age)| (age as u32, score as u32))
            .collect::<Box<_>>();

        players.sort_unstable();

        drop(ages);

        let mut cache = scores; // Reuse already allocated memory.
        let mut result = 0;

        #[allow(clippy::unnecessary_lazy_evaluations)] // Not supported.
        for (i, &(age, score)) in players.iter().enumerate() {
            let (target, left_cache) = cache[..=i].split_last_mut().unwrap();

            let total_score = left_cache
                .iter()
                .zip(&*players)
                .filter(|&(_, &(left_age, left_score))| left_age == age || left_score <= score)
                .map(|(&left_total_score, _)| left_total_score as u32)
                .max()
                .map_or(score, |left_score| left_score + score);

            *target = total_score as _;

            result = result.max(total_score);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        Self::best_team_score(scores, ages)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
