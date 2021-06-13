pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;
use std::convert::TryInto;

impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses
            .iter()
            .map(|course| {
                let [duration, deadline]: [_; 2] = course.as_slice().try_into().unwrap();

                (duration as u16, deadline as u16)
            })
            .collect::<Vec<_>>();

        courses.sort_unstable_by_key(|&(_, deadline)| deadline);

        let mut queue = BinaryHeap::with_capacity(courses.len());
        let mut time = 0;

        for (duration, deadline) in courses {
            let new_time = time + duration;

            if new_time <= deadline {
                time = new_time;
                queue.push(duration);
            } else if let Some(mut top) = queue.peek_mut() {
                if *top > duration {
                    time -= *top - duration;
                    *top = duration;
                }
            }
        }

        queue.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        Self::schedule_course(courses)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
