pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let (num1, mut num2) = if num2.len() < num1.len() {
            (num2.into_bytes(), num1.into_bytes())
        } else {
            (num1.into_bytes(), num2.into_bytes())
        };

        let mut iter = num1.iter().rev().zip(num2.iter_mut().rev());
        let (lhs, mut rhs) = iter.next().unwrap();

        *rhs += lhs - b'0';

        loop {
            if *rhs < b'0' + 10 {
                if let Some((lhs, new_rhs)) = iter.next() {
                    *new_rhs += lhs - b'0';
                    rhs = new_rhs;
                } else {
                    break;
                }
            } else {
                *rhs -= 10;

                if let Some((lhs, new_rhs)) = iter.next() {
                    *new_rhs += lhs - (b'0' - 1);
                    rhs = new_rhs;
                } else {
                    let mut iter = num2.iter_mut().rev().skip(num1.len());

                    loop {
                        if let Some(rhs) = iter.next() {
                            if *rhs == b'9' {
                                *rhs = b'0';
                            } else {
                                *rhs += 1;

                                break;
                            }
                        } else {
                            num2.insert(0, b'1');

                            break;
                        }
                    }

                    break;
                }
            }
        }

        String::from_utf8(num2).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn add_strings(num1: String, num2: String) -> String {
        Self::add_strings(num1, num2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
