pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn find_root(mappings: &mut [u8; 26], c: u8) -> u8 {
        let parent = mappings[usize::from(c)];

        if parent == u8::MAX {
            c
        } else {
            let root = Self::find_root(mappings, parent);

            mappings[usize::from(c)] = root;

            root
        }
    }

    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut mappings = [u8::MAX; 26];

        for (lhs, rhs) in s1.bytes().zip(s2.bytes()) {
            let lhs_root = Self::find_root(&mut mappings, lhs - b'a');
            let rhs_root = Self::find_root(&mut mappings, rhs - b'a');

            match lhs_root.cmp(&rhs_root) {
                Ordering::Less => mappings[usize::from(rhs_root)] = lhs_root,
                Ordering::Equal => {}
                Ordering::Greater => mappings[usize::from(lhs_root)] = rhs_root,
            }
        }

        let mut result = base_str.into_bytes();

        for c in &mut result {
            *c = b'a' + Self::find_root(&mut mappings, *c - b'a');
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        Self::smallest_equivalent_string(s1, s2, base_str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
