pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;

        if let Some((mut left, rest)) = height.split_first() {
            if let Some((mut right, mut rest)) = rest.split_last() {
                'outer: loop {
                    if left < right {
                        while let Some((middle, new_rest)) = rest.split_first() {
                            rest = new_rest;

                            if middle < left {
                                result += left - middle;
                            } else {
                                left = middle;

                                continue 'outer;
                            }
                        }
                    } else {
                        while let Some((middle, new_rest)) = rest.split_last() {
                            rest = new_rest;

                            if middle < right {
                                result += right - middle;
                            } else {
                                right = middle;

                                continue 'outer;
                            }
                        }
                    }

                    break;
                }
            }
        }

        result
    }
}

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
