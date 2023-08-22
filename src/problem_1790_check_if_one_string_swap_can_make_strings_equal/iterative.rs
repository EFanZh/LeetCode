pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut iter = s1.bytes().zip(s2.bytes());

        let (left_1, right_1) = loop {
            if let Some((left, right)) = iter.next() {
                if left != right {
                    break (left, right);
                }
            } else {
                return true;
            }
        };

        loop {
            if let Some((left, right)) = iter.next() {
                if left != right {
                    if left == right_1 && right == left_1 {
                        break;
                    }

                    return false;
                }
            } else {
                return false;
            }
        }

        iter.all(|(left, right)| left == right)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn are_almost_equal(s1: String, s2: String) -> bool {
        Self::are_almost_equal(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
