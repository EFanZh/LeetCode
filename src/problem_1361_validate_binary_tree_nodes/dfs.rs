pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn dfs(children_data: &[&[i32]; 2], node: usize, states: &mut [u8]) -> bool {
        for children in children_data {
            let child = children[node] as usize;

            if let Some(child_state) = states.get_mut(child) {
                match child_state {
                    0 => {
                        *child_state = 2;

                        if !Self::dfs(children_data, child, states) {
                            return false;
                        }
                    }
                    1 => *child_state = 2,
                    _ => return false,
                }
            }
        }

        true
    }

    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let n = n as _;
        let children_data = [left_child.as_slice(), right_child.as_slice()];
        let mut states = vec![0; n];

        for node in 0..n {
            if let state @ 0 = &mut states[node] {
                *state = 2;

                if !Self::dfs(&children_data, node, &mut states) {
                    return false;
                }

                states[node] = 1;
            }
        }

        let mut found_root = false;

        for state in states {
            if state == 1 {
                if found_root {
                    return false;
                }

                found_root = true;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        Self::validate_binary_tree_nodes(n, left_child, right_child)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
