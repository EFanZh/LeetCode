pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn combine_helper(n: usize, k: usize, base: &mut [i32], result: &mut Vec<Vec<i32>>) {
        let index = k.wrapping_sub(1);

        if index < base.len() {
            for i in k..=n {
                base[index] = i as _;

                Self::combine_helper(i - 1, index, base, result);
            }
        } else {
            result.push(base.to_vec());
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        Self::combine_helper(n as _, k as _, &mut vec![0; k as usize], &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
