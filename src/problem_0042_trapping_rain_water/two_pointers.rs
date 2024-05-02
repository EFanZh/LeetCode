pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;

        if let [left_max, rest @ .., right_max] = height.as_slice() {
            let mut left_max = *left_max as u32;
            let mut right_max = *right_max as u32;
            let mut iter = rest.iter().map(|&x| x as u32);

            'outer: loop {
                if right_max < left_max {
                    for right in iter.by_ref().rev() {
                        if right <= right_max {
                            result += right_max - right;
                        } else {
                            right_max = right;

                            continue 'outer;
                        }
                    }
                } else {
                    for left in iter.by_ref() {
                        if left <= left_max {
                            result += left_max - left;
                        } else {
                            left_max = left;

                            continue 'outer;
                        }
                    }
                }

                break;
            }
        }

        result as _
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
