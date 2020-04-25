pub struct Solution {}

impl Solution {
    fn combination_sum_helper(candidates: &[i32], target: i32, used: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(used.clone());
        } else if let Some((&first, rest)) = candidates.split_first() {
            if target >= first {
                used.push(first);
                Self::combination_sum_helper(candidates, target - first, used, result);
                used.pop();
            }

            Self::combination_sum_helper(rest, target, used, result);
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::combination_sum_helper(&candidates, target, &mut Vec::new(), &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::combination_sum(candidates, target)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::Solution;

    #[test]
    fn test_solution() {
        run::<Solution>();
    }
}
