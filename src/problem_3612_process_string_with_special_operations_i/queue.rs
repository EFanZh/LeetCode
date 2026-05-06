pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn process_str(s: String) -> String {
        let mut queue = VecDeque::<u8>::new();
        let mut reversed = false;

        for c in s.bytes() {
            match (c, reversed) {
                (b'*', false) => {
                    queue.pop_back();
                }
                (b'*', true) => {
                    queue.pop_front();
                }
                (b'#', false) => {
                    for i in 0..queue.len() {
                        queue.push_back(queue[i]);
                    }
                }
                (b'#', true) => {
                    let i = queue.len().wrapping_sub(1);

                    for _ in 0..queue.len() {
                        queue.push_front(queue[i]);
                    }
                }
                (b'%', _) => reversed = !reversed,
                (c, false) => queue.push_back(c),
                (c, true) => queue.push_front(c),
            }
        }

        let s = s.into_bytes();
        let mut result = s;

        result.clear();

        if reversed {
            result.extend(queue.into_iter().rev());
        } else {
            result.extend(queue);
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn process_str(s: String) -> String {
        Self::process_str(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
