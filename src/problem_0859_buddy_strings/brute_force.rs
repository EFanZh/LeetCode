pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() == goal.len() {
            let mut iter = s.bytes().zip(goal.bytes());

            while let Some((lhs_1, rhs_1)) = iter.next() {
                if lhs_1 != rhs_1 {
                    while let Some((lhs_2, rhs_2)) = iter.next() {
                        if lhs_2 != rhs_2 {
                            return lhs_1 == rhs_2 && lhs_2 == rhs_1 && iter.all(|(lhs, rhs)| lhs == rhs);
                        }
                    }

                    return false;
                }
            }

            let mut count = [false; 26];

            for c in s.bytes() {
                if let value @ false = &mut count[usize::from(c) - usize::from(b'a')] {
                    *value = true;
                } else {
                    return true;
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn buddy_strings(s: String, goal: String) -> bool {
        Self::buddy_strings(s, goal)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
