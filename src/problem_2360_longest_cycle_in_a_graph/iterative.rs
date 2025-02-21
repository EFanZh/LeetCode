pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        const MAX_BIT: u32 = 1 << 31;

        let mut edges = edges;
        let n = edges.len();
        let mut result = 0;
        let mut current_id = MAX_BIT - 1;

        for i in 0..n {
            let mut state = &mut edges[i];

            if ((*state + 1) as u32) < MAX_BIT {
                current_id += 1;

                let start_id = current_id;

                loop {
                    let next = mem::replace(state, current_id as _) as usize;

                    if let Some(next_state) = edges.get_mut(next) {
                        if ((*next_state + 1) as u32) < MAX_BIT {
                            state = next_state;
                            current_id += 1;

                            continue;
                        }

                        if *next_state as u32 >= start_id {
                            result = result.max(current_id - *next_state as u32);
                        }
                    }

                    break;
                }
            }
        }

        if result == 0 { -1 } else { (result + 1) as _ }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_cycle(edges: Vec<i32>) -> i32 {
        Self::longest_cycle(edges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
