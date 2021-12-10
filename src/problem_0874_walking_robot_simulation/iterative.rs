pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::convert::TryInto;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut position = (0, 0);
        let mut direction = (0, 1);

        let obstacles = obstacles
            .into_iter()
            .map(|position| {
                let [x, y]: [i32; 2] = position.as_slice().try_into().unwrap();

                (x, y)
            })
            .collect::<HashSet<_>>();

        let mut result = 0;

        for command in commands {
            match command {
                -2 => direction = (-direction.1, direction.0),
                -1 => direction = (direction.1, -direction.0),
                mut distance => loop {
                    let next_position = (position.0 + direction.0, position.1 + direction.1);

                    if obstacles.contains(&next_position) {
                        break;
                    }

                    position = next_position;
                    distance -= 1;

                    result = result.max(position.0.pow(2) + position.1.pow(2));

                    if distance == 0 {
                        break;
                    }
                },
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        Self::robot_sim(commands, obstacles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
