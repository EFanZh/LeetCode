pub struct Solution;

impl Solution {
    fn combination_sum2_helper_non_empty(
        first: i32,
        mut rest: &[i32],
        target: i32,
        base: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target >= first {
            base.push(first);
            Self::combination_sum2_helper(rest, target - first, base, result);
            base.pop();

            while let Some((&new_first, new_rest)) = rest.split_first() {
                if new_first == first {
                    rest = new_rest;
                } else {
                    Self::combination_sum2_helper_non_empty(new_first, new_rest, target, base, result);

                    break;
                }
            }
        }
    }

    fn combination_sum2_helper(candidates: &[i32], target: i32, used: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(used.clone());
        } else if let Some((&first, rest)) = candidates.split_first() {
            Self::combination_sum2_helper_non_empty(first, rest, target, used, result);
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
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
