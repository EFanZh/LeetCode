pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut counts = [0_u8; 101];

        for &height in &heights {
            counts[height as usize] += 1;
        }

        let mut result = 0;
        let mut i = 0;

        for height in heights {
            loop {
                let count = &mut counts[i];

                if *count == 0 {
                    i += 1;
                } else {
                    *count -= 1;

                    break;
                }
            }

            result += i32::from(height != i as i32);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn height_checker(heights: Vec<i32>) -> i32 {
        Self::height_checker(heights)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
