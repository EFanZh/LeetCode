pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        let filter = |&(_, c): &(usize, u8)| c != b'X';
        let mut left_iter = start.bytes().enumerate().filter(filter);
        let mut right_iter = end.bytes().enumerate().filter(filter);

        loop {
            match (left_iter.next(), right_iter.next()) {
                (None, None) => return true,
                (Some((left_index, b'L')), Some((right_index, b'L'))) => {
                    if left_index < right_index {
                        return false;
                    }
                }
                (Some((left_index, b'R')), Some((right_index, b'R'))) => {
                    if right_index < left_index {
                        return false;
                    }
                }
                _ => return false,
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_transform(start: String, end: String) -> bool {
        Self::can_transform(start, end)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
