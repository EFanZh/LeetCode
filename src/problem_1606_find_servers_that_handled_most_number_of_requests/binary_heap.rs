pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

struct Item {
    finish: u32,
    server: u32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.finish.cmp(&self.finish)
    }
}

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as u32;
        let mut busy = BinaryHeap::new();
        let mut idle = (0..k).map(|i| Reverse((0, i))).collect::<BinaryHeap<_>>();
        let mut candidate = (0_u32, 0);
        let mut counts = vec![0_u32; k as usize].into_boxed_slice();

        for (&start, &length) in arrival.iter().zip(&load) {
            let start = start as u32;
            let length = length as u32;

            // Finish all request till now.

            while let Some(&Item { finish, server }) = busy.peek() {
                if finish <= start {
                    busy.pop();
                    idle.push(Reverse((
                        if server < candidate.1 {
                            candidate.0 + 1
                        } else {
                            candidate.0
                        },
                        server,
                    )));
                } else {
                    break;
                }
            }

            // Select a server for the request.

            if let Some(Reverse((_, server))) = idle.pop() {
                counts[server as usize] += 1;

                busy.push(Item {
                    finish: start + length,
                    server,
                });
            }

            candidate.1 += 1;

            if candidate.1 == k {
                candidate.0 += 1;
                candidate.1 = 0;
            }
        }

        drop((busy, idle));

        let mut max_count = 0;
        let mut result = Vec::new();

        for (i, &count) in (0..).zip(&*counts) {
            match count.cmp(&max_count) {
                Ordering::Less => {}
                Ordering::Equal => result.push(i),
                Ordering::Greater => {
                    max_count = count;
                    result.clear();
                    result.push(i);
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        Self::busiest_servers(k, arrival, load)
    }
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { finish: 2, server: 3 } == Item { finish: 2, server: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
