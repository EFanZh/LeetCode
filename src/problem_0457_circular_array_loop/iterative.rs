pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn find_cycle(
        nums: &[i32],
        root: i32,
        mut node: i32,
        roots: &mut [i32],
        mut check: impl FnMut(i32) -> bool,
    ) -> bool {
        let n = nums.len() as i32;
        let mut node_root = &mut roots[node as usize];

        while *node_root == -1 {
            let next_step = nums[node as usize];

            if check(next_step) {
                let next = (node + next_step).rem_euclid(n);

                if next == node {
                    break;
                }

                *node_root = root;
                node = next;
                node_root = &mut roots[node as usize];

                if *node_root == root {
                    return true;
                }
            } else {
                break;
            }
        }

        false
    }

    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let n = nums.len() as i32;
        let mut roots = vec![-1; nums.len()];

        for (node, &step) in (0..).zip(&nums) {
            let root = &mut roots[node as usize];

            if *root == -1 {
                *root = node;

                let next = (node + step).rem_euclid(n);

                if next != node {
                    if step < 0 {
                        if Self::find_cycle(&nums, node, next, &mut roots, i32::is_negative) {
                            return true;
                        }
                    } else if Self::find_cycle(&nums, node, next, &mut roots, i32::is_positive) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn circular_array_loop(nums: Vec<i32>) -> bool {
        Self::circular_array_loop(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
