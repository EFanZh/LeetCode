pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut n = n as u32;
        let mut result = n;

        n -= 1;

        if n != 0 {
            result *= n;
            n -= 1;

            if n != 0 {
                result /= n;
                n -= 1;

                while n != 0 {
                    result += n;

                    n -= 1;

                    if n == 0 {
                        break;
                    }

                    let mut value = n;

                    n -= 1;

                    if n == 0 {
                        result -= value;

                        break;
                    }

                    value *= n;

                    n -= 1;

                    if n == 0 {
                        result -= value;

                        break;
                    }

                    value /= n;
                    result -= value;

                    n -= 1;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn clumsy(n: i32) -> i32 {
        Self::clumsy(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
