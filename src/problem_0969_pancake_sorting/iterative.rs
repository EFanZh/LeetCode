pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let n = arr.len();
        let mut result = Vec::new();
        let mut length = n;

        while length > 1 {
            let i = arr.iter().position(|&x| x == length as i32).unwrap();

            if i + 1 != length {
                let reverse = i + 1;

                result.extend([reverse as i32, length as i32]);

                arr[..reverse].reverse();
                arr[..length].reverse();
            }

            length -= 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        Self::pancake_sort(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
