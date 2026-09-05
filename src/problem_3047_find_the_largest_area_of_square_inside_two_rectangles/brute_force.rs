pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_overlap(left_1: i32, right_1: i32, left_2: i32, right_2: i32) -> u32 {
        let left = left_1.max(left_2);
        let right = right_1.min(right_2);

        if left < right {
            right.wrapping_sub(left).cast_unsigned()
        } else {
            0
        }
    }

    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let rectangles = bottom_left
            .into_iter()
            .zip(top_right)
            .map(|(bottom_left, top_right)| {
                let [left, bottom] = bottom_left.try_into().ok().unwrap();
                let [right, top] = top_right.try_into().ok().unwrap();

                (left, bottom, right, top)
            })
            .collect::<Box<_>>();

        let mut iter = rectangles.iter();
        let mut max_overlap = 0;

        while let Some(&(left_1, bottom_1, right_1, top_1)) = iter.next() {
            for &(left_2, bottom_2, right_2, top_2) in iter.clone() {
                max_overlap = u32::max(
                    max_overlap,
                    u32::min(
                        Self::get_overlap(left_1, right_1, left_2, right_2),
                        Self::get_overlap(bottom_1, top_1, bottom_2, top_2),
                    ),
                );
            }
        }

        u64::from(max_overlap).pow(2).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        Self::largest_square_area(bottom_left, top_right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
