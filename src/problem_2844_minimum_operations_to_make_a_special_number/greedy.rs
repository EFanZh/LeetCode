pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_operations(num: String) -> i32 {
        let mut iter_1 = num.bytes().rev();

        let mut result = iter_1.position(|d| d == b'0').map_or(num.len(), |i| {
            iter_1
                .position(|d| matches!(d, b'5' | b'0'))
                .map_or(num.len() - 1, |j| i + j)
        });

        iter_1 = num.bytes().rev();

        if let Some(operations) = iter_1
            .position(|d| d == b'5')
            .and_then(|i| iter_1.position(|d| matches!(d, b'2' | b'7')).map(|j| i + j))
        {
            result = result.min(operations);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_operations(num: String) -> i32 {
        Self::minimum_operations(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
