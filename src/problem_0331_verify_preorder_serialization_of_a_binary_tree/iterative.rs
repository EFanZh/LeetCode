pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut stack = 1;

        for node in preorder.split(',') {
            if stack == 0 {
                return false;
            } else if node == "#" {
                stack -= 1;
            } else {
                stack += 1;
            }
        }

        stack == 0
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
