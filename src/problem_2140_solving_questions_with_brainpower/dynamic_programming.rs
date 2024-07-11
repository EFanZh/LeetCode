pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut cache = vec![0; questions.len()].into_boxed_slice();

        for (i, question) in questions.into_iter().enumerate().rev() {
            let [points, brainpower]: [_; 2] = question.try_into().ok().unwrap();
            let score_if_skip = cache.get(i + 1).copied().unwrap_or(0);

            let score_if_solve =
                u64::from(points as u32) + cache.get(i + 1 + brainpower as u32 as usize).copied().unwrap_or(0);

            cache[i] = score_if_skip.max(score_if_solve);
        }

        cache[0] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        Self::most_points(questions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
