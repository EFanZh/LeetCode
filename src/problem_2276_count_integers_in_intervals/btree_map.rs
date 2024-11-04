// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BTreeMap;

pub struct CountIntervals {
    intervals: BTreeMap<u32, u32>,
    count: u32,
    buffer: Vec<u32>,
}

impl CountIntervals {
    fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
            count: 0,
            buffer: Vec::new(),
        }
    }

    fn add(&mut self, left: i32, right: i32) {
        let left = left as u32;
        let mut right = right as u32 + 1;

        let mut to_remove_iter = self.intervals.range(left + 1..=right).map(|(&start, &end)| {
            self.count -= end - start;

            (start, end)
        });

        if let Some((last_start, last_end)) = to_remove_iter.next_back() {
            self.buffer.extend(to_remove_iter.map(|(start, _)| start));

            for &start in &self.buffer {
                self.intervals.remove(&start);
            }

            self.intervals.remove(&last_start);

            self.buffer.clear();

            right = right.max(last_end);
        }

        if let Some(previous_node) = self.intervals.range_mut(..=left).next_back() {
            if *previous_node.1 >= left {
                if *previous_node.1 < right {
                    self.count += right - *previous_node.1;
                    *previous_node.1 = right;
                }

                return;
            }
        }

        self.count += right - left;
        self.intervals.insert(left, right);
    }

    fn count(&self) -> i32 {
        self.count as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::CountIntervals for CountIntervals {
    fn new() -> Self {
        Self::new()
    }

    fn add(&mut self, left: i32, right: i32) {
        self.add(left, right);
    }

    fn count(&self) -> i32 {
        self.count()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::CountIntervals>();
    }
}
