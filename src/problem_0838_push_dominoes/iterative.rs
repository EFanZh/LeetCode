pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut distances = vec![0; dominoes.len()];
        let mut prev_left = usize::MAX;

        for (distance, (i, c)) in distances.iter_mut().zip(dominoes.bytes().enumerate()).rev() {
            match c {
                b'L' => prev_left = i,
                b'R' => {
                    *distance = usize::MAX;
                    prev_left = usize::MAX;
                }
                _ => {
                    *distance = if prev_left == usize::MAX {
                        usize::MAX
                    } else {
                        prev_left - i
                    }
                }
            }
        }

        let mut dominoes = dominoes.into_bytes();
        let mut prev_right = usize::MAX;

        for (i, (target, right_distance)) in dominoes.iter_mut().zip(distances).enumerate() {
            match target {
                b'L' => prev_right = usize::MAX,
                b'R' => prev_right = i,
                _ => {
                    if prev_right == usize::MAX {
                        if right_distance != usize::MAX {
                            *target = b'L';
                        }
                    } else {
                        match (i - prev_right).cmp(&right_distance) {
                            Ordering::Less => *target = b'R',
                            Ordering::Equal => {}
                            Ordering::Greater => *target = b'L',
                        }
                    }
                }
            }
        }

        String::from_utf8(dominoes).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn push_dominoes(dominoes: String) -> String {
        Self::push_dominoes(dominoes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
