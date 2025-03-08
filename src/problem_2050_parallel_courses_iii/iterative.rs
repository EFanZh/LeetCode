pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::mem;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let _ = n;
        let states = time.into_iter().map(|x| Cell::new(x as u32)).collect::<Vec<_>>();
        let n = states.len();
        let mut graph = vec![Vec::new(); n].into_boxed_slice();

        for relation in relations {
            let [prev, next] = relation.try_into().ok().unwrap();

            graph[prev as u32 as usize - 1].push(next as u16 - 1);
        }

        let mut result = 0;
        let mut stack = Vec::new();

        for node in 0..n {
            let children = &mut graph[node];
            let mut state = &states[node];

            if !children.is_empty() {
                let mut max = 0;
                let mut iter = mem::take(children).into_iter();

                loop {
                    loop {
                        if let Some(child) = iter.next() {
                            stack.push((state, max, iter));

                            let node = usize::from(child);
                            let children = &mut graph[node];

                            state = &states[node];

                            if !children.is_empty() {
                                max = 0;
                                iter = mem::take(&mut graph[usize::from(child)]).into_iter();

                                continue;
                            }
                        } else {
                            state.set(state.get() + max);
                        }

                        break;
                    }

                    if let Some((top_state, top_max, top_iter)) = stack.pop() {
                        max = top_max.max(state.get());
                        state = top_state;
                        iter = top_iter;
                    } else {
                        break;
                    }
                }
            }

            result = result.max(state.get());
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        Self::minimum_time(n, relations, time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
