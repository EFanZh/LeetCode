pub struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut stack = 1;
        let mut iter = preorder.split(',');

        for node in iter.by_ref() {
            if node == "#" {
                stack -= 1;

                if stack == 0 {
                    return iter.next().is_none();
                }
            } else {
                stack += 1
            }
        }

        false
    }
}

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
