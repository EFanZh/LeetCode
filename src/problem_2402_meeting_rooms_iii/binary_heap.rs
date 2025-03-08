pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::binary_heap::PeekMut;
use std::collections::{BinaryHeap, VecDeque};

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as u32;

        let mut meetings = meetings
            .into_iter()
            .map(|meeting| {
                let [start, end] = meeting.try_into().ok().unwrap();

                (start as u32, end as u32)
            })
            .collect::<Vec<_>>();

        meetings.sort_unstable_by_key(|&(start, _)| start);

        let mut deallocate_queue = BinaryHeap::<Reverse<(u32, u8)>>::new();
        let mut allocate_queue = VecDeque::new();
        let usage = &mut [0_u32; 100][..n as usize];
        let mut free_rooms = (0..n as u8).map(Reverse).collect::<BinaryHeap<_>>();

        for (start, end) in meetings {
            while let Some(mut top) = deallocate_queue.peek_mut() {
                let Reverse((time, room)) = *top;

                if time <= start {
                    if let Some(allocate_request) = allocate_queue.pop_front() {
                        usage[usize::from(room)] += 1;
                        top.0.0 += allocate_request;
                    } else {
                        PeekMut::pop(top);
                        free_rooms.push(Reverse(room));
                    }
                } else {
                    break;
                }
            }

            if let Some(Reverse(free_room)) = free_rooms.pop() {
                usage[usize::from(free_room)] += 1;

                deallocate_queue.push(Reverse((end, free_room)));
            } else {
                allocate_queue.push_back(end - start);
            }
        }

        while let Some(allocate_request) = allocate_queue.pop_front() {
            let top = &mut *deallocate_queue.peek_mut().unwrap();

            usage[usize::from(top.0.1)] += 1;
            top.0.0 += allocate_request;
        }

        let mut max_room = 0;
        let mut max_room_count = 0;

        (0..).zip(usage).for_each(|(room, &mut count)| {
            if count > max_room_count {
                max_room_count = count;
                max_room = room;
            }
        });

        max_room
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        Self::most_booked(n, meetings)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
