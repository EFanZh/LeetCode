pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut iter = height.into_iter();

        if let (Some(mut left), Some(mut right)) = (iter.next(), iter.next_back()) {
            'outer: loop {
                if left < right {
                    while let Some(middle) = iter.next() {
                        if middle < left {
                            result += left - middle;
                        } else {
                            left = middle;

                            continue 'outer;
                        }
                    }
                } else {
                    while let Some(middle) = iter.next_back() {
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
