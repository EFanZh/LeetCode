pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn combination_sum3_helper(first: i32, k: i32, n: i32, base: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if k == 0 {
            if n == 0 {
                result.push(base.clone());
            }
        } else {
            for num in first..11 - k {
                base.push(num);

                Self::combination_sum3_helper(num + 1, k - 1, n - num, base, result);

                base.pop();
            }
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
