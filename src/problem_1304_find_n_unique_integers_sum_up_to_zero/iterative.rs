pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let n = n as u32;
        let mut result = Vec::with_capacity(n as _);

        if n % 2 != 0 {
            result.push(0);
        }

        for i in 0..n / 2 {
            let i = i as i32 + 1;

            result.push(-i);
            result.push(i);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_zero(n: i32) -> Vec<i32> {
        Self::sum_zero(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
