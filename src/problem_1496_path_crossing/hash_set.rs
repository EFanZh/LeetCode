pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visited = HashSet::from([(0_u16, 0_u16)]);
        let mut current = (0_u16, 0_u16);

        for c in path.bytes() {
            match c {
                b'E' => current.0 += 1,
                b'N' => current.1 += 1,
                b'S' => current.1 -= 1,
                _ => current.0 -= 1,
            }

            if !visited.insert(current) {
                return true;
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_path_crossing(path: String) -> bool {
        Self::is_path_crossing(path)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
