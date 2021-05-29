pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        let mut result = Vec::new();

        match num.cmp(&0) {
            Ordering::Less => {
                let mut num = -num;

                while num != 0 {
                    result.push(b'0' + (num % 7) as u8);
                    num /= 7;
                }

                result.push(b'-');
            }
            Ordering::Equal => result.push(b'0'),
            Ordering::Greater => {
                while num != 0 {
                    result.push(b'0' + (num % 7) as u8);
                    num /= 7;
                }
            }
        }

        result.reverse();

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn convert_to_base7(num: i32) -> String {
        Self::convert_to_base7(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
