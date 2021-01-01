pub struct Solution;

impl Solution {
    pub fn max_rotate_function(a: Vec<i32>) -> i32 {
        a.split_first().map_or(0, |(_, rest)| {
            let mut prev = (1..).zip(rest).map(|(i, x)| i * x).sum::<i32>();
            let mut result = prev;
            let sum = a.iter().sum::<i32>();
            let n = a.len() as i32;

            for x in rest.iter().rev() {
                prev += sum - x * n;
                result = result.max(prev);
            }

            result
        })
    }
}

impl super::Solution for Solution {
    fn max_rotate_function(a: Vec<i32>) -> i32 {
        Self::max_rotate_function(a)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
