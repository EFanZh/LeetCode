pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut stack = Vec::new();

        for (right_height, right_index) in height.into_iter().zip(0..) {
            if let Some(&(_, mut middle_height)) = stack.last() {
                if right_height >= middle_height {
                    loop {
                        stack.pop();

                        if let Some(&(left_index, left_height)) = stack.last() {
                            if right_height >= left_height {
                                result += (left_height - middle_height) * (right_index - left_index - 1);

                                middle_height = left_height;

                                continue;
                            }

                            result += (right_height - middle_height) * (right_index - left_index - 1);
                        }

                        break;
                    }
                }
            }

            stack.push((right_index, right_height));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn trap(height: Vec<i32>) -> i32 {
        Self::trap(height)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
