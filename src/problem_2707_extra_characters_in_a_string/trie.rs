pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ops::ControlFlow;

struct Node {
    has_value: bool,
    children: [u16; 26],
}

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut root = Node {
            has_value: false,
            children: [u16::MAX; 26],
        };

        let mut memory = Vec::with_capacity(dictionary.iter().map(String::len).sum::<usize>() / 2);
        let mut memory_len = 0;

        for word in dictionary {
            let mut node = &mut root;

            for c in word.into_bytes() {
                let child_index = &mut node.children[usize::from(c) - usize::from(b'a')];
                let child_index_value = *child_index;

                node = if child_index_value < memory_len {
                    &mut memory[usize::from(child_index_value)]
                } else {
                    *child_index = memory_len;
                    memory_len += 1;

                    memory.push_mut(Node {
                        has_value: false,
                        children: [u16::MAX; 26],
                    })
                };
            }

            node.has_value = true;
        }

        let s = s.into_bytes();
        let n = s.len();
        let mut cache = vec![0_u8; n + 1].into_boxed_slice();

        for start in (0..n).rev() {
            let mut node = &root;

            let result = s[start..].iter().zip(&cache[start + 1..]).try_fold(
                cache[start + 1] + 1,
                |min_extra, (&c, &prev_count)| {
                    memory
                        .get(usize::from(node.children[usize::from(c) - usize::from(b'a')]))
                        .map_or(ControlFlow::Break(min_extra), |next_node| {
                            node = next_node;

                            ControlFlow::Continue(if node.has_value {
                                min_extra.min(prev_count)
                            } else {
                                min_extra
                            })
                        })
                },
            );

            cache[start] = match result {
                ControlFlow::Continue(min_extra) | ControlFlow::Break(min_extra) => min_extra,
            };
        }

        i32::from(cache[0])
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        Self::min_extra_char(s, dictionary)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
