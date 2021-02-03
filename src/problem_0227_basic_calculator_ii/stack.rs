pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut iter = s.bytes().peekable();
        let mut nums = Vec::new();
        let mut ops = Vec::new();

        while let Some(c) = iter.next() {
            match c {
                b'+' | b'-' => {
                    match ops.pop() {
                        Some(b'+') => *nums.last_mut().unwrap() += nums.pop().unwrap(),
                        Some(b'-') => *nums.last_mut().unwrap() -= nums.pop().unwrap(),
                        _ => {}
                    }

                    ops.push(c);
                }
                b'*' | b'/' => ops.push(c),
                b'0'..=b'9' => {
                    let mut num = i32::from(c - b'0');

                    while let Some(&c @ b'0'..=b'9') = iter.peek() {
                        num *= 10;
                        num += i32::from(c - b'0');

                        iter.next();
                    }

                    match ops.last() {
                        Some(b'*') => {
                            ops.pop();
                            *nums.last_mut().unwrap() *= num
                        }
                        Some(b'/') => {
                            ops.pop();
                            *nums.last_mut().unwrap() /= num
                        }
                        _ => nums.push(num),
                    }
                }
                _ => {}
            }
        }

        while let Some(op) = ops.pop() {
            if op == b'+' {
                *nums.last_mut().unwrap() += nums.pop().unwrap();
            } else {
                *nums.last_mut().unwrap() -= nums.pop().unwrap();
            }
        }

        nums.pop().unwrap()
    }
}

impl super::Solution for Solution {
    fn calculate(s: String) -> i32 {
        Self::calculate(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
