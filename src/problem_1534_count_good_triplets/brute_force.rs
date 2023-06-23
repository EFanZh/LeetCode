pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let arr = arr.as_slice();
        let mut result = 0;

        for (j, &middle) in arr.iter().enumerate() {
            let min_left = middle - a;
            let max_left = middle + a;
            let min_right = middle - b;
            let max_right = middle + b;

            for &left in &arr[..j] {
                if left >= min_left && left <= max_left {
                    let min_right = min_right.max(left - c);
                    let max_right = max_right.min(left + c);

                    for &right in &arr[j + 1..] {
                        result += i32::from(right >= min_right && right <= max_right);
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        Self::count_good_triplets(arr, a, b, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
