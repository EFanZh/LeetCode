pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let n = n as usize;

        if let Some(mut n) = n.checked_sub(4) {
            let mut cache = VecDeque::new();
            let mut result = 2;

            cache.extend(&[true, false]);

            while n != 0 {
                if cache.pop_front().unwrap() {
                    cache.push_back(false);
                    n -= 1;
                    result += 1;

                    if n == 0 {
                        break;
                    }
                }

                cache.push_back(true);
                n -= 1;

                if n == 0 {
                    break;
                }

                if cache.pop_front().unwrap() {
                    cache.push_back(true);
                    n -= 1;

                    if n == 0 {
                        break;
                    }
                }

                cache.push_back(false);
                n -= 1;
                result += 1;
            }

            result
        } else {
            1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn magical_string(n: i32) -> i32 {
        Self::magical_string(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
