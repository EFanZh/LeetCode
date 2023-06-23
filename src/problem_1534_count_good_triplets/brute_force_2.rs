pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut left_iter = arr.iter().copied();
        let mut result = 0;

        while let Some(left) = left_iter.next() {
            let mut middle_iter = left_iter.clone();
            let min_middle = left - a;
            let max_middle = left + a;
            let min_right = left - c;
            let max_right = left + c;

            while let Some(middle) = middle_iter.next() {
                if middle >= min_middle && middle <= max_middle {
                    let min_right = min_right.max(middle - b);
                    let max_right = max_right.min(middle + b);

                    for right in middle_iter.clone() {
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
