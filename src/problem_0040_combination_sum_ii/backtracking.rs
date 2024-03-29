pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn combination_sum2_helper(candidates: &[i32], target: i32, base: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(base.clone());
        } else if let Some((&first, rest)) = candidates.split_first() {
            if target >= first {
                base.push(first);
                Self::combination_sum2_helper(rest, target - first, base, result);
                base.pop();

                if let Some(next) = rest.iter().position(|x| *x != first) {
                    Self::combination_sum2_helper(&rest[next..], target, base, result);
                }
            }
        }
    }

    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut candidates = candidates;

        candidates.sort_unstable();

        Self::combination_sum2_helper(&candidates, target, &mut Vec::new(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
