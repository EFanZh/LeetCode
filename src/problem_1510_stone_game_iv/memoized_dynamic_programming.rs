pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Frame {
    n: u32,
    x: u32,
}

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as u32;
        let mut cache = vec![0; n as usize + 1].into_boxed_slice();

        let mut frame = Frame {
            n,
            x: f64::from(n).sqrt() as _,
        };

        let mut stack = Vec::new();

        cache[0] = 1;

        let state = 'outer: loop {
            let next_n = frame.n - frame.x * frame.x;
            let mut result = cache[next_n as usize];

            if result == 0 {
                stack.push(Frame { n: frame.n, x: frame.x });

                frame.n = next_n;
                frame.x = f64::from(next_n).sqrt() as _;

                continue;
            }

            loop {
                result = if result == 1 {
                    2
                } else {
                    frame.x -= 1;

                    if frame.x == 0 {
                        1
                    } else {
                        break;
                    }
                };

                cache[frame.n as usize] = result;

                if let Some(new_frame) = stack.pop() {
                    frame = new_frame;
                } else {
                    break 'outer result;
                }
            }
        };

        state == 2
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn winner_square_game(n: i32) -> bool {
        Self::winner_square_game(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
