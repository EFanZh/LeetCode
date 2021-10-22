pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let c = c as u8;
        let mut result = vec![0; s.len()];
        let mut prev_position = i32::MIN;

        for (i, (distance, x)) in (0..).zip(result.iter_mut().zip(s.bytes())) {
            if x == c {
                prev_position = i;
            } else {
                *distance = i.saturating_sub(prev_position);
            }
        }

        prev_position = i32::MIN;

        for (i, (distance, x)) in (0..).zip(result.iter_mut().zip(s.bytes()).rev()) {
            if x == c {
                prev_position = i;
            } else {
                *distance = (*distance).min(i.saturating_sub(prev_position));
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        Self::shortest_to_char(s, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
