pub struct Solution;

use std::mem;

impl Solution {
    fn next_item(item: &[u8], target: &mut String) {
        use std::fmt::Write;

        let (mut first, rest) = item.split_first().unwrap();
        let mut length = 1;

        for next in rest {
            if next == first {
                length += 1;
            } else {
                write!(target, "{}{}", length, char::from(*first)).unwrap();

                first = next;
                length = 1;
            }
        }

        write!(target, "{}{}", length, char::from(*first)).unwrap();
    }

    pub fn count_and_say(n: i32) -> String {
        let mut result = "1".to_string();
        let mut temp = String::new();

        for _ in 1..n {
            Self::next_item(result.as_bytes(), &mut temp);
            result.clear();
            mem::swap(&mut result, &mut temp);
        }

        result
    }
}

impl super::Solution for Solution {
    fn count_and_say(n: i32) -> String {
        Self::count_and_say(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
