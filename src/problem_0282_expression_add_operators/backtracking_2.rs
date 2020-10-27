pub struct Solution;

use std::slice::Iter;

impl Solution {
    fn add_operators_helper(
        lhs: i64,
        rhs: i64,
        mut iter: Iter<u8>,
        target: i64,
        base: &mut Vec<u8>,
        result: &mut Vec<String>,
    ) {
        if let Some(&first) = iter.next() {
            let i = base.len();

            base.extend(&[b'+', first]);

            if first == b'0' {
                Self::add_operators_helper(lhs + rhs, 0, iter.clone(), target, base, result);

                base[i] = b'-';

                Self::add_operators_helper(lhs + rhs, 0, iter.clone(), target, base, result);

                base[i] = b'*';

                Self::add_operators_helper(lhs, 0, iter.clone(), target, base, result);
            } else {
                let mut value = (first - b'0').into();

                loop {
                    Self::add_operators_helper(lhs + rhs, value, iter.clone(), target, base, result);

                    base[i] = b'-';

                    Self::add_operators_helper(lhs + rhs, -value, iter.clone(), target, base, result);

                    base[i] = b'*';

                    Self::add_operators_helper(lhs, rhs * value, iter.clone(), target, base, result);

                    if let Some(&next) = iter.next() {
                        base[i] = b'+';
                        base.push(next);
                        value = value * 10 + i64::from(next - b'0');
                    } else {
                        break;
                    }
                }
            }

            base.truncate(i);
        } else if lhs + rhs == target {
            result.push(String::from_utf8(base.clone()).unwrap());
        }
    }

    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut iter = num.as_bytes().iter();

        if let Some(&first) = iter.next() {
            let mut base = Vec::with_capacity(num.len() * 2 - 1);

            base.push(first);

            if first == b'0' {
                Self::add_operators_helper(0, 0, iter.clone(), target.into(), &mut base, &mut result);
            } else {
                let mut value = (first - b'0').into();

                loop {
                    Self::add_operators_helper(0, value, iter.clone(), target.into(), &mut base, &mut result);

                    if let Some(&next) = iter.next() {
                        base.push(next);
                        value = value * 10 + i64::from(next - b'0');
                    } else {
                        break;
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn add_operators(num: String, target: i32) -> Vec<String> {
        Self::add_operators(num, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
