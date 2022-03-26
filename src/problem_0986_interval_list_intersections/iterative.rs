pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::convert::TryInto;

enum State {
    BothClosed,
    Open1,
    Open2,
    BothOpen(i32),
}

impl State {
    fn open_1(&self) -> bool {
        matches!(self, State::Open1 | State::BothOpen(_))
    }

    fn open_2(&self) -> bool {
        matches!(self, State::Open2 | State::BothOpen(_))
    }
}

impl Solution {
    fn iter_list(list: &[Vec<i32>]) -> impl Iterator<Item = i32> + '_ {
        list.iter().flat_map(|interval| {
            let interval: [_; 2] = interval.as_slice().try_into().unwrap();

            interval
        })
    }

    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut iter_1 = Self::iter_list(&first_list);
        let mut iter_2 = Self::iter_list(&second_list);
        let mut state = State::BothClosed;

        // Both unknown.

        'outer: while let Some(mut value_1) = iter_1.next() {
            loop {
                // Got value 1.

                if let Some(value_2) = iter_2.next() {
                    loop {
                        // Got both.

                        match (value_1, state.open_1()).cmp(&(value_2, state.open_2())) {
                            Ordering::Less => {
                                match state {
                                    State::BothClosed => state = State::Open1,
                                    State::Open1 => state = State::BothClosed,
                                    State::Open2 => state = State::BothOpen(value_1),
                                    State::BothOpen(start) => {
                                        result.push(vec![start, value_1]);
                                        state = State::Open2;
                                    }
                                }

                                if let Some(next_value_1) = iter_1.next() {
                                    value_1 = next_value_1;
                                } else {
                                    break 'outer;
                                }
                            }
                            Ordering::Equal => {
                                if let State::BothOpen(start) = state {
                                    result.push(vec![start, value_1]);
                                    state = State::BothClosed;
                                } else {
                                    state = State::BothOpen(value_1);
                                }

                                continue 'outer;
                            }
                            Ordering::Greater => {
                                match state {
                                    State::BothClosed => state = State::Open2,
                                    State::Open1 => state = State::BothOpen(value_2),
                                    State::Open2 => state = State::BothClosed,
                                    State::BothOpen(start) => {
                                        result.push(vec![start, value_2]);
                                        state = State::Open1;
                                    }
                                };

                                break;
                            }
                        }
                    }
                } else {
                    break 'outer;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::interval_intersection(first_list, second_list)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
