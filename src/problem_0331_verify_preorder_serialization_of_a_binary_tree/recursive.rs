pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper<I: Iterator<Item = bool>>(mut iter: I, k: &mut dyn FnMut(I) -> bool) -> bool {
        match iter.next() {
            Some(false) => k(iter),
            Some(true) => Self::helper(iter, &mut |iter| Self::helper(iter, k)),
            None => false,
        }
    }

    pub fn is_valid_serialization(preorder: String) -> bool {
        Self::helper(preorder.split(',').map(|x| x != "#"), &mut |mut iter| {
            iter.next().is_none()
        })
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
