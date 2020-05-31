pub struct Solution {}

impl Solution {
    fn combine_helper(n: i32, k: i32, base: &mut [i32], result: &mut Vec<Vec<i32>>) {
        if k == 0 {
            result.push(base.to_vec());
        } else {
            for i in k..=n {
                base[(k - 1) as usize] = i;

                Self::combine_helper(i - 1, k - 1, base, result)
            }
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::combine_helper(n, k, &mut vec![0; k as usize], &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        Self::combine(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
