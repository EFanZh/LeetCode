pub struct Solution {}

impl Solution {
    fn combination_sum_helper(candidates: &[i32], target: i32, base: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(base.clone());
        } else if let Some((&first, rest)) = candidates.split_first() {
            if target >= first {
                base.push(first);
                Self::combination_sum_helper(candidates, target - first, base, result);
                base.pop();
            }

            Self::combination_sum_helper(rest, target, base, result);
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
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
