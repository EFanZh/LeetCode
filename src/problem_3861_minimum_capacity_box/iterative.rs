pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
        let item_size = item_size.cast_unsigned();
        let mut result = u32::MAX;
        let mut min_capacity = u32::MAX;

        (0..).zip(capacity).for_each(|(i, capacity)| {
            let capacity = capacity.cast_unsigned();

            if capacity >= item_size && capacity < min_capacity {
                min_capacity = capacity;
                result = i;
            }
        });

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
        Self::minimum_index(capacity, item_size)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
