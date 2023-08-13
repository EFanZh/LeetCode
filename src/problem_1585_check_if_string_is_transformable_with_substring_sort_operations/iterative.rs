pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn is_transformable(s: String, t: String) -> bool {
        let mut indices = [(); 10].map(|()| VecDeque::new());

        for (i, digit) in (0_u32..).zip(s.into_bytes()) {
            indices[usize::from(digit) - usize::from(b'0')].push_back(i);
        }

        for digit in t.into_bytes() {
            let digit = usize::from(digit) - usize::from(b'0');

            if let Some(index) = indices[digit].pop_front() {
                if indices[..digit]
                    .iter()
                    .any(|indices| indices.front().map_or(false, |&index_2| index_2 < index))
                {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_transformable(s: String, t: String) -> bool {
        Self::is_transformable(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
