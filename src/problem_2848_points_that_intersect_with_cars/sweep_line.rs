pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let mut events = Vec::with_capacity(nums.len());

        for num in nums {
            let [start, end] = num.try_into().ok().unwrap();

            events.extend([(start as u32) << 2, ((end as u32 + 1) << 2) + 2]);
        }

        events.sort_unstable();

        let mut result = 0;
        let mut prev_position = 0;
        let mut count = 0;

        for event in events {
            let position = event >> 2;
            let state = event & 0b_11;

            result += u32::from(count != 0) * (position - prev_position);
            prev_position = position;
            count = count + 1 - state;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        Self::number_of_points(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
