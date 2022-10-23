pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;

        arr.sort_unstable_by_key(|&value| (value.count_ones(), value as u32));

        arr
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        Self::sort_by_bits(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
