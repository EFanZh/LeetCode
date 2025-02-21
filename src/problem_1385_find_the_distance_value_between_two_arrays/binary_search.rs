pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut arr2 = arr2;

        arr2.sort_unstable();

        let mut result = 0;

        for value in arr1 {
            let low = value - d;
            let high = value + d;
            let i = arr2.partition_point(|&x| x < low);

            #[expect(clippy::unnecessary_map_or, reason = "compatibility")]
            if arr2.get(i).map_or(true, |&x| x > high) {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        Self::find_the_distance_value(arr1, arr2, d)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
