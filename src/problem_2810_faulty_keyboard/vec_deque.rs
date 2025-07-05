pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut s = s.into_bytes();
        let mut deque = VecDeque::new();
        let mut iter = s.iter().copied();

        'outer: loop {
            if let Some(c) = iter.next() {
                if c == b'i' {
                    loop {
                        if let Some(c) = iter.next() {
                            if c == b'i' {
                                break;
                            }

                            deque.push_front(c);
                        } else {
                            s.clear();
                            s.extend(deque.iter().rev());

                            break 'outer;
                        }
                    }
                } else {
                    deque.push_back(c);
                }
            } else {
                s.clear();
                s.extend(&deque);

                break;
            }
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn final_string(s: String) -> String {
        Self::final_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
