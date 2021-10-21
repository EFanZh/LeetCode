// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BTreeMap;

pub struct SummaryRanges {
    intervals: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
        }
    }

    #[allow(clippy::if_then_some_else_none)]
    fn add_num(&mut self, val: i32) {
        if let Some((right_from, right_to)) =
            self.intervals
                .range(val + 1..)
                .next()
                .and_then(|(&from, &to)| if from == val + 1 { Some((from, to)) } else { None })
        {
            if let Some(left_to) =
                self.intervals.range_mut(..val).next_back().and_then(
                    |(_, to)| {
                        if *to == val - 1 {
                            Some(to)
                        } else {
                            None
                        }
                    },
                )
            {
                *left_to = right_to;
            } else {
                self.intervals.insert(val, right_to);
            }

            self.intervals.remove(&right_from);
        } else if let Some((_, left_to)) = self.intervals.range_mut(..val).next_back() {
            match (*left_to).cmp(&(val - 1)) {
                Ordering::Less => {
                    self.intervals.entry(val).or_insert(val);
                }
                Ordering::Equal => *left_to = val,
                Ordering::Greater => {}
            }
        } else {
            self.intervals.entry(val).or_insert(val);
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals.iter().map(|(&from, &to)| vec![from, to]).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::SummaryRanges for SummaryRanges {
    fn new() -> Self {
        Self::new()
    }

    fn add_num(&mut self, val: i32) {
        self.add_num(val);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.get_intervals()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::SummaryRanges>();
    }
}
