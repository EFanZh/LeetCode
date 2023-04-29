pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    fn kmp_prefix_function(s: &[u8], buffer: &mut Vec<usize>) {
        let base = buffer.len();

        buffer.extend(iter::repeat(0).take(s.len()));

        let buffer = &mut buffer[base..];
        let mut matched = 0;
        let mut i = 1;

        while let Some(&c) = s.get(i) {
            loop {
                if s[matched] == c {
                    matched += 1;
                    buffer[i] = matched;

                    break;
                } else if let Some(&next_matched) = buffer.get(matched.wrapping_sub(1)) {
                    matched = next_matched;
                } else {
                    break;
                }
            }

            i += 1;
        }
    }

    fn longest_overlap(mut left: &[u8], right: &[u8], right_prefix_function: &[usize]) -> usize {
        if right.len() < left.len() {
            left = &left[left.len() - right.len()..];
        };

        let mut matched = 0;

        for &c in &left[1..] {
            loop {
                if right[matched] == c {
                    matched += 1;

                    break;
                } else if let Some(&next_matched) = right_prefix_function.get(matched.wrapping_sub(1)) {
                    matched = next_matched;
                } else {
                    break;
                }
            }
        }

        matched
    }

    fn combinations(n: usize, k: usize, base: u16, f: &mut impl FnMut(u16)) {
        if k == 0 {
            f(base);
        } else {
            for i in (k - 1)..n {
                Self::combinations(i, k - 1, base | (1 << i), f);
            }
        }
    }

    pub fn shortest_superstring(words: Vec<String>) -> String {
        // Compute KMP prefix functions.

        let mut prefix_function_buffer = Vec::new();

        for word in &words {
            Self::kmp_prefix_function(word.as_bytes(), &mut prefix_function_buffer);
        }

        // Build overlap graph.

        let prefix_function_buffer = prefix_function_buffer.as_slice();
        let n = words.len();
        let mut total_length = 0;

        let mut iter = words
            .iter()
            .map(move |s| {
                let new_total_length = total_length + s.len();
                let prefix_function = &prefix_function_buffer[total_length..new_total_length];

                total_length = new_total_length;

                (s.as_bytes(), prefix_function)
            })
            .enumerate();

        let mut graph = vec![0; n * n];

        while let Some((i, (left, left_prefix_function))) = iter.next() {
            for (j, (right, right_prefix_function)) in iter.clone() {
                graph[n * i + j] = Self::longest_overlap(left, right, right_prefix_function);
                graph[n * j + i] = Self::longest_overlap(right, left, left_prefix_function);
            }
        }

        // Held-Karp algorithm.

        let mut cache = vec![(0, 0); n * (1 << n)];

        for bits in 2..=n {
            Self::combinations(n, bits, 0, &mut |state| {
                for first in 0..n {
                    let next_state = state & !(1 << first);

                    if next_state != state {
                        cache[n * usize::from(state) + first] = cache
                            [n * usize::from(next_state)..n * (usize::from(next_state) + 1)]
                            .iter()
                            .copied()
                            .enumerate()
                            .filter_map(|(next_first, (next_overlaps, _))| {
                                (next_state & (1 << next_first) != 0)
                                    .then(|| (graph[n * first + next_first] + next_overlaps, next_first))
                            })
                            .max_by_key(|&(overlaps, _)| overlaps)
                            .unwrap();
                    }
                }
            });
        }

        // Construct result.

        let mut state = (1 << n) - 1;

        let mut first = cache[n * ((1 << n) - 1)..]
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(_, (overlaps, _))| overlaps)
            .unwrap()
            .0;

        let mut result = words[first].clone();

        loop {
            let next_state = state ^ (1 << first);

            if next_state == 0 {
                break;
            }

            let next_first = cache[n * state + first].1;

            result.push_str(&words[next_first][graph[n * first + next_first]..]);
            state = next_state;
            first = next_first;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_superstring(words: Vec<String>) -> String {
        Self::shortest_superstring(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
