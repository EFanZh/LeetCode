pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut obstacles = obstacles;
        let mut lengths = Vec::with_capacity(obstacles.len());

        for height in &mut obstacles {
            let height_u32 = *height as u32;
            let i = lengths.partition_point(|&x| x <= height_u32);

            if let Some(value) = lengths.get_mut(i) {
                *value = height_u32;
            } else {
                lengths.push(height_u32);
            }

            *height = i as i32 + 1;
        }

        obstacles
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        Self::longest_obstacle_course_at_each_position(obstacles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
