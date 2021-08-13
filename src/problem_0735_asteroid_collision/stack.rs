pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut asteroids = asteroids;

        if let Some(mut stack_base) = asteroids.iter().position(|&x| x > 0) {
            if let Some(mut stack_top) = asteroids[stack_base + 1..].iter().position(|&x| x < 0) {
                stack_top += stack_base + 1;

                let mut i = stack_top;

                while let Some(&x) = asteroids.get(i) {
                    if x < 0 {
                        loop {
                            if stack_base == stack_top {
                                asteroids[stack_base] = x;
                                stack_base += 1;
                                stack_top += 1;

                                break;
                            }

                            match (-x).cmp(&asteroids[stack_top - 1]) {
                                Ordering::Less => break,
                                Ordering::Equal => {
                                    stack_top -= 1;

                                    break;
                                }
                                Ordering::Greater => stack_top -= 1,
                            }
                        }
                    } else {
                        asteroids[stack_top] = x;
                        stack_top += 1;
                    }

                    i += 1;
                }

                asteroids.truncate(stack_top);
            }
        }

        asteroids
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        Self::asteroid_collision(asteroids)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
