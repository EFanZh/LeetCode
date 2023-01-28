pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let n = n as usize;

        n.checked_sub(4).map_or(1, |mut n| {
            let mut cache = VecDeque::new();
            let mut result = 2;

            cache.extend([true, false]);

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
        })
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
