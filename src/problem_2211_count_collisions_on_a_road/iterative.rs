pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

enum State {
    AllLeft,
    Stop,
    Right(u32),
}

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut result = 0;
        let mut state = State::AllLeft;
        let mut iter = directions.bytes();

        loop {
            match state {
                State::AllLeft => {
                    if let Some(c) = iter.next() {
                        match c {
                            b'L' => {}
                            b'R' => state = State::Right(1),
                            _ => state = State::Stop,
                        }
                    } else {
                        break;
                    }
                }
                State::Stop => {
                    if let Some(c) = iter.next() {
                        match c {
                            b'L' => result += 1,
                            b'R' => state = State::Right(1),
                            _ => {}
                        }
                    } else {
                        break;
                    }
                }
                State::Right(count) => {
                    if let Some(c) = iter.next() {
                        match c {
                            b'L' => {
                                result += count + 1;
                                state = State::Stop;
                            }
                            b'R' => state = State::Right(count + 1),
                            _ => {
                                result += count;
                                state = State::Stop;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_collisions(directions: String) -> i32 {
        Self::count_collisions(directions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
