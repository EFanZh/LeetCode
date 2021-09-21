pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn combination_sum3_helper(first: i32, k: i32, n: i32, base: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        let remaining_numbers = 10 - first;

        if remaining_numbers == k {
            if (first + 9) * remaining_numbers / 2 == n {
                let saved_length = base.len();

                base.extend(first..10);

                result.push(base.clone());

                base.truncate(saved_length);
            }
        } else {
            if first <= n {
                base.push(first);

                Self::combination_sum3_helper(first + 1, k - 1, n - first, base, result);

                base.pop();
            }

            Self::combination_sum3_helper(first + 1, k, n, base, result);
        }
    }

    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::combination_sum3_helper(1, k, n, &mut Vec::new(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        Self::combination_sum3(k, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
