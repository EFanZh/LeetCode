pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut mapping = [0; 256];
        let mut used = [false; 256];

        for (left, right) in s.bytes().zip(t.bytes()) {
            let mapping_slot = &mut mapping[usize::from(left)];

            if *mapping_slot == 0 {
                if let used_slot @ false = &mut used[usize::from(right)] {
                    *used_slot = true;
                } else {
                    return false;
                }

                *mapping_slot = right;
            } else if *mapping_slot != right {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_isomorphic(s: String, t: String) -> bool {
        Self::is_isomorphic(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
