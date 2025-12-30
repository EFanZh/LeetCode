pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut position = 0;

        for command in commands {
            match command.bytes().next() {
                Some(b'D') => position += n,
                Some(b'L') => position -= 1,
                Some(b'R') => position += 1,
                _ => position -= n,
            }
        }

        position
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        Self::final_position_of_snake(n, commands)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
