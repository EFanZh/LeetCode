pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct State {
    rows: u32,
    columns: u32,
    result: u32,
}

impl State {
    fn horizontal_cut(&mut self, cost: u32) {
        self.result += cost * self.columns;
        self.rows += 1;
    }

    fn vertical_cut(&mut self, cost: u32) {
        self.result += cost * self.rows;
        self.columns += 1;
    }
}

impl Solution {
    pub fn minimum_cost(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32 {
        _ = (m, n);

        let mut horizontal_cut = horizontal_cut.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut vertical_cut = vertical_cut.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();

        horizontal_cut.sort_unstable();
        vertical_cut.sort_unstable();

        let mut h_iter = horizontal_cut.iter().rev().copied();
        let mut v_iter = vertical_cut.iter().rev().copied();

        let mut state = State {
            rows: 1,
            columns: 1,
            result: 0,
        };

        'outer: {
            'v: {
                if let Some(mut h_cost) = h_iter.next() {
                    for v_cost in v_iter.by_ref() {
                        while h_cost >= v_cost {
                            state.horizontal_cut(h_cost);

                            if let Some(next_h_cost) = h_iter.next() {
                                h_cost = next_h_cost;
                            } else {
                                state.vertical_cut(v_cost);

                                break 'v;
                            }
                        }

                        state.vertical_cut(v_cost);
                    }

                    state.horizontal_cut(h_cost);
                    h_iter.for_each(|v_cost| state.horizontal_cut(v_cost));

                    break 'outer;
                }
            }

            v_iter.for_each(|v_cost| state.vertical_cut(v_cost));
        }

        state.result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_cost(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32 {
        Self::minimum_cost(m, n, horizontal_cut, vertical_cut)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
