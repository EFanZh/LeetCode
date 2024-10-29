pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(nodes: &[(u8, Vec<u32>)], node: &(u8, Vec<u32>), result: &mut u32) -> u32 {
        let mut max_1 = 0;
        let mut max_2 = 0;

        node.1.iter().for_each(|&child| {
            let child = &nodes[child as usize];
            let child_length = Self::helper(nodes, child, result);

            if child.0 != node.0 && child_length > max_2 {
                if child_length > max_1 {
                    max_2 = max_1;
                    max_1 = child_length;
                } else {
                    max_2 = child_length;
                }
            }
        });

        let length = max_1 + 1;

        *result = (*result).max(length + max_2);

        length
    }

    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let mut nodes = s.into_bytes().into_iter().map(|c| (c, Vec::new())).collect::<Box<_>>();

        (0..).zip(parent).for_each(|(child, parent)| {
            if let Some(children) = nodes.get_mut(parent as usize) {
                children.1.push(child);
            }
        });

        let mut result = 0;

        Self::helper(&nodes, &nodes[0], &mut result);

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        Self::longest_path(parent, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
