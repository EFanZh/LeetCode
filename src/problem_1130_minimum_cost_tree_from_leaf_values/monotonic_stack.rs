pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/discuss/339959/One-Pass-O(N)-Time-and-Space>.

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut middle = i32::MAX;
        let mut result = 0;

        for right in arr {
            if middle <= right {
                loop {
                    let left = stack.pop().unwrap();

                    if left <= right {
                        result += middle * left;
                        middle = left;
                    } else {
                        result += middle * right;
                        middle = left;

                        break;
                    }
                }
            }

            stack.push(middle);
            middle = right;
        }

        while stack.len() > 1 {
            let left = stack.pop().unwrap();

            result += middle * left;
            middle = left;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        Self::mct_from_leaf_values(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
