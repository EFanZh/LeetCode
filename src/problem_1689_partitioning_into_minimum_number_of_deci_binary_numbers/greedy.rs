pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut result = b'0';

        for c in n.bytes() {
            if c > result {
                result = c;

                if result == b'9' {
                    break;
                }
            }
        }

        i32::from(result - b'0')
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_partitions(n: String) -> i32 {
        Self::min_partitions(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
