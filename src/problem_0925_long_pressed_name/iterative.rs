pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut prev_left = 0;
        let mut iter = typed.bytes();

        for left in name.bytes() {
            loop {
                if let Some(right) = iter.next() {
                    if right == left {
                        break;
                    } else if right != prev_left {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            prev_left = left;
        }

        iter.all(|c| c == prev_left)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_long_pressed_name(name: String, typed: String) -> bool {
        Self::is_long_pressed_name(name, typed)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
