pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let target = arr.len() / 4;
        let mut current = 0;
        let mut length = 0;

        for value in arr {
            if value == current {
                length += 1;
            } else {
                current = value;
                length = 1;
            }

            if length > target {
                break;
            }
        }

        current
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_special_integer(arr: Vec<i32>) -> i32 {
        Self::find_special_integer(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
