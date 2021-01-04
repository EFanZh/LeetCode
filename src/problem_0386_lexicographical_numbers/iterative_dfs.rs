pub struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as _);
        let mut value = 1;

        loop {
            result.push(value);

            let next = value * 10;

            if next <= n {
                value = next;
            } else {
                if value == n {
                    value /= 10;
                }

                while value % 10 == 9 {
                    value /= 10;
                }

                if value == 0 {
                    break;
                }

                value += 1;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn lexical_order(n: i32) -> Vec<i32> {
        Self::lexical_order(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
