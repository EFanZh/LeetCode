pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut prev_color = 0;
        let mut max_time = 0_u32;
        let mut total_save = 0;
        let mut total_time = 0;

        for (color, time) in colors.bytes().zip(needed_time) {
            let time = time as u32;

            max_time = if color == prev_color {
                max_time.max(time)
            } else {
                total_save += max_time;
                prev_color = color;

                time
            };

            total_time += time;
        }

        (total_time - (total_save + max_time)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        Self::min_cost(colors, needed_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
