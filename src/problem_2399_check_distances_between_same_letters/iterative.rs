pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut indices = [0_u8; 26];
        let mut i = 0;

        s.bytes().all(|c| {
            let c = usize::from(c) - usize::from(b'a');
            let state = &mut indices[c];

            i += 1;

            if *state == 0 {
                *state = i;
            } else if i - *state - 1 != distance[c] as u8 {
                return false;
            }

            true
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_distances(s: String, distance: Vec<i32>) -> bool {
        Self::check_distances(s, distance)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
