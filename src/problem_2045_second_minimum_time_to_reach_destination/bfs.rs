pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::num::NonZeroU32;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as u32 as usize;
        let time = time as u32;
        let change = change as u32;
        let cycle = NonZeroU32::new(change * 2).unwrap();
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for edge in edges {
            let [from, to] = <[_; 2]>::map(edge.try_into().ok().unwrap(), |x| x as u16 - 1);

            graph[usize::from(from)].push(to);
            graph[usize::from(to)].push(from);
        }

        let target = n as u16 - 1;
        let mut states = vec![0_u16; n].into_boxed_slice();
        let mut queue = VecDeque::from([0]);
        let mut step = 2;

        states[0] = 1;

        'outer: loop {
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                for &neighbor in &graph[usize::from(node)] {
                    let state = &mut states[usize::from(neighbor)];

                    if *state == u16::MAX || *state == step {
                        continue;
                    }

                    *state = if *state == 0 {
                        step
                    } else {
                        if neighbor == target {
                            break 'outer;
                        }

                        u16::MAX
                    };

                    queue.push_back(neighbor);
                }
            }

            step += 1;
        }

        let mut now = 0;

        for _ in 1..step {
            let remainder = now % cycle;

            if remainder >= change {
                now += cycle.get() - remainder;
            }

            now += time;
        }

        now as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        Self::second_minimum(n, edges, time, change)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
