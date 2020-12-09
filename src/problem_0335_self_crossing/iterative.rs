pub struct Solution;

impl Solution {
    pub fn is_self_crossing(x: Vec<i32>) -> bool {
        let mut iter = x.into_iter();
        let mut l1 = 0;
        let mut l2 = 0;
        let mut l3 = 0;
        let mut l4 = 0;

        while let Some(length) = iter.next() {
            if length <= l3 {
                let mut spiral_width = length;
                let mut spiral_height = if length < (l3 - l1) { l4 } else { l4 - l2 };

                for length in iter {
                    if length < spiral_height {
                        spiral_height = spiral_width;
                        spiral_width = length;
                    } else {
                        return true;
                    }
                }

                break;
            } else {
                l1 = l2;
                l2 = l3;
                l3 = l4;
                l4 = length;
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn is_self_crossing(x: Vec<i32>) -> bool {
        Self::is_self_crossing(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
