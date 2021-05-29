pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut stack = 1;
        let mut iter = preorder.split(',');

        loop {
            match iter.next() {
                None => return false,
                Some("#") => {
                    stack -= 1;

                    if stack == 0 {
                        break;
                    }
                }
                Some(_) => stack += 1,
            }
        }

        iter.next().is_none()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_valid_serialization(preorder: String) -> bool {
        Self::is_valid_serialization(preorder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
