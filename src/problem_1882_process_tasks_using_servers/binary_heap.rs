pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut idle_servers = (0_u32..)
            .zip(servers)
            .map(|(index, weight)| Reverse((weight as u32, index)))
            .collect::<BinaryHeap<_>>();

        let mut busy_servers = BinaryHeap::with_capacity(idle_servers.len());
        let mut tasks = tasks;

        for (time, target) in (0_u32..).zip(&mut tasks) {
            while let Some(&Reverse((idle_time, weight, index))) = busy_servers.peek() {
                if idle_time <= time {
                    busy_servers.pop();
                    idle_servers.push(Reverse((weight, index)));
                } else {
                    break;
                }
            }

            *target = (if let Some(Reverse((weight, index))) = idle_servers.pop() {
                busy_servers.push(Reverse((time + *target as u32, weight, index)));

                index
            } else {
                let mut busy_server = busy_servers.peek_mut().unwrap();
                let busy_server = &mut busy_server.0;

                busy_server.0 += *target as u32;

                busy_server.2
            }) as _;
        }

        tasks
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        Self::assign_tasks(servers, tasks)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
