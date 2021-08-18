pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BTreeMap;

struct RangeModule {
    ranges: BTreeMap<i32, i32>,
    remove_buffer: Vec<i32>,
}

impl RangeModule {
    fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
            remove_buffer: Vec::new(),
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let mut left = left;
        let mut right = right;
        let mut iter = self.ranges.range(..=right);

        if let Some((&last_left, &last_right)) = iter.next_back() {
            if last_right >= left {
                if last_left > left {
                    self.remove_buffer.push(last_left);

                    for (&current_left, &current_right) in iter.rev() {
                        if current_left > left {
                            self.remove_buffer.push(current_left);
                        } else {
                            if current_right >= left {
                                left = current_left;
                            }

                            break;
                        }
                    }

                    for value in self.remove_buffer.drain(..) {
                        self.ranges.remove(&value);
                    }
                } else {
                    left = last_left;
                }

                if last_right > right {
                    right = last_right;
                }
            }
        }

        self.ranges.insert(left, right);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        self.ranges
            .range(..=left)
            .next_back()
            .map_or(false, |(_, &last_right)| last_right >= right)
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let mut iter = self.ranges.range(..=right);

        if let Some((&last_left, &last_right)) = iter.next_back() {
            if last_right >= left {
                if last_left >= left {
                    self.remove_buffer.push(last_left);

                    for (&current_left, &current_right) in iter.rev() {
                        if current_left >= left {
                            self.remove_buffer.push(current_left);
                        } else {
                            if current_right >= left {
                                self.ranges.insert(current_left, left);
                            }

                            break;
                        }
                    }

                    for value in self.remove_buffer.drain(..) {
                        self.ranges.remove(&value);
                    }
                } else {
                    self.ranges.insert(last_left, left);
                }

                if last_right > right {
                    self.ranges.insert(right, last_right);
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::RangeModule for RangeModule {
    fn new() -> Self {
        Self::new()
    }

    fn add_range(&mut self, left: i32, right: i32) {
        self.add_range(left, right);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        self.query_range(left, right)
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        self.remove_range(left, right);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::RangeModule>();
    }
}
