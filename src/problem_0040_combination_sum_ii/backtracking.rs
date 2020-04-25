pub struct Solution {}

impl Solution {
    fn combination_sum2_helper(candidates: &[i32], target: i32, used: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(used.clone());
        } else if let Some((&first, rest)) = candidates.split_first() {
            if target >= first {
                used.push(first);
                Self::combination_sum2_helper(rest, target - first, used, result);
                used.pop();

                if let Some(next) = rest.iter().position(|x| *x != first) {
                    Self::combination_sum2_helper(&rest[next..], target, used, result);
                }
            }
        }
    }

    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        candidates.sort_unstable();

        Self::combination_sum2_helper(&candidates, target, &mut Vec::new(), &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::combination_sum2(candidates, target)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
