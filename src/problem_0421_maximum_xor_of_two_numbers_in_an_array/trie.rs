pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct Node {
    zero: Option<Box<Node>>,
    one: Option<Box<Node>>,
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut root = Node::default();

        for num in &nums {
            let mut node = &mut root;
            let mut probe = 1 << 30;

            loop {
                if num & probe == 0 {
                    node = node.zero.get_or_insert_with(Box::default);
                } else {
                    node = node.one.get_or_insert_with(Box::default);
                }

                if probe == 1 {
                    break;
                }

                probe >>= 1;
            }
        }

        let mut result = 0;

        for num in nums {
            let mut current = 0;
            let mut node = &mut root;
            let mut probe = 1 << 30;

            loop {
                if num & probe == 0 {
                    if let Some(child) = node.one.as_deref_mut() {
                        node = child;
                        current |= probe;
                    } else {
                        node = node.zero.as_deref_mut().unwrap();
                    }
                } else if let Some(child) = node.zero.as_deref_mut() {
                    node = child;
                    current |= probe;
                } else {
                    node = node.one.as_deref_mut().unwrap();
                }

                if probe == 1 {
                    break;
                }

                probe >>= 1;
            }

            result = result.max(current);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        Self::find_maximum_xor(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
