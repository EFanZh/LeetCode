pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let sqrt_n = n.isqrt();
        let mut steps = 1;

        if sqrt_n * sqrt_n != n {
            let mut queue = (1..=sqrt_n).map(|x| (x * x, x)).collect::<VecDeque<_>>();
            let mut visited = vec![false; n as _];

            'outer: loop {
                steps += 1;

                for _ in 0..queue.len() {
                    let (current, min_sqrt) = queue.pop_front().unwrap();
                    let remaining = n - current;
                    let sqrt_remaining = remaining.isqrt();

                    if sqrt_remaining * sqrt_remaining == remaining {
                        break 'outer;
                    }

                    for i in min_sqrt..=sqrt_remaining {
                        let next = current + i * i;

                        if let value @ false = &mut visited[next as usize] {
                            *value = true;

                            queue.push_back((next, i));
                        }
                    }
                }
            }
        }

        steps
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_squares(n: i32) -> i32 {
        Self::num_squares(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
