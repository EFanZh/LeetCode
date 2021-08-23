pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        const MODULUS: u64 = 1_000_000_007;

        let s = s.into_bytes();
        let n = s.len();

        if n < 3 {
            n as _
        } else {
            let columns = n - 2;
            let mut cache = vec![0; columns * columns];
            let mut window = [VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new()];

            for length in 3..=n {
                let (left, right) = s.split_at(length);

                for (i, &c) in left.iter().enumerate() {
                    window[usize::from(c) - usize::from(b'a')].push_back(i);
                }

                let mut window_start = 0;
                let mut iter = (length..).zip(s.iter().zip(right));

                loop {
                    let mut count = 0;

                    for indices in &window {
                        if let Some(&first) = indices.front() {
                            let last = *indices.back().unwrap();
                            let range = last - first;

                            count += if range < 4 {
                                range as u64 + 1
                            } else {
                                cache[columns * (range - 4) + first + 1] + 2
                            };
                        }
                    }

                    cache[columns * (length - 3) + window_start] = count % MODULUS;

                    if let Some((i, (&old, &new))) = iter.next() {
                        window[usize::from(old) - usize::from(b'a')].pop_front();
                        window[usize::from(new) - usize::from(b'a')].push_back(i);
                        window_start += 1;
                    } else {
                        break;
                    }
                }

                for indices in &mut window {
                    indices.clear();
                }
            }

            cache[columns * (n - 3)] as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_palindromic_subsequences(s: String) -> i32 {
        Self::count_palindromic_subsequences(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
