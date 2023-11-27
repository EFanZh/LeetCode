pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

struct Window {
    start: usize,
    weight: u32,
    trips: u32,
}

impl Window {
    fn shift_start(&mut self, boxes: &[(u32, u32)]) {
        let (old_port, old_weight) = boxes[self.start];

        self.weight -= old_weight;
        self.trips -= u32::from(old_port != boxes[self.start + 1].0);
        self.start += 1;
    }
}

impl Solution {
    pub fn box_delivering(boxes: Vec<Vec<i32>>, ports_count: i32, max_boxes: i32, max_weight: i32) -> i32 {
        let boxes = boxes
            .into_iter()
            .map(|box_| {
                let [port, weight]: [_; 2] = box_.try_into().ok().unwrap();

                (port as u32, weight as u32)
            })
            .collect::<Box<_>>();

        let _ = ports_count;
        let max_boxes = max_boxes as u32 as usize;
        let max_weight = max_weight as u32;
        let mut cache = vec![0_u32; boxes.len()].into_boxed_slice();

        let mut window = Window {
            start: 0,
            weight: 0,
            trips: 1,
        };

        let mut prev_port = u32::MAX;

        for (i, &(port, weight)) in boxes.iter().enumerate() {
            window.weight += weight;
            window.trips += u32::from(port != prev_port);
            prev_port = port;

            while window.weight > max_weight {
                window.shift_start(&boxes);
            }

            if i - window.start == max_boxes {
                window.shift_start(&boxes);
            }

            let prev_trips = cache.get(window.start.wrapping_sub(1)).copied().unwrap_or_default();

            for &cost in &cache[window.start..i] {
                if cost == prev_trips {
                    window.shift_start(&boxes);
                } else {
                    break;
                }
            }

            cache[i] = prev_trips + window.trips;
        }

        *cache.last().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn box_delivering(boxes: Vec<Vec<i32>>, ports_count: i32, max_boxes: i32, max_weight: i32) -> i32 {
        Self::box_delivering(boxes, ports_count, max_boxes, max_weight)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
