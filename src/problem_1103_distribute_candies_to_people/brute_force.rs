pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut candies = candies as u32;
        let num_people = num_people as u32;
        let mut required = 1;
        let mut result = vec![0; num_people as usize];

        loop {
            for target in &mut result {
                if candies <= required {
                    *target += candies as i32;

                    return result;
                }

                *target += required as i32;
                candies -= required;
                required += 1;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        Self::distribute_candies(candies, num_people)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
