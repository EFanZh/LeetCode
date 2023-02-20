pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut buffer = [0_u16; 1000];

        for x in target {
            let x = x as u16 - 1;
            buffer[usize::from(x)] += 1;
        }

        for y in arr {
            let y = y as u16 - 1;
            let count = &mut buffer[usize::from(y)];

            if let Some(new_count) = count.checked_sub(1) {
                *count = new_count;
            } else {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        Self::can_be_equal(target, arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
