pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut iter = arr.iter().map(|&x| x & 1 != 0);

        while let Some(is_odd) = iter.next() {
            if is_odd {
                if let Some(is_odd) = iter.next() {
                    if is_odd {
                        if let Some(is_odd) = iter.next() {
                            if is_odd {
                                return true;
                            }
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        Self::three_consecutive_odds(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
