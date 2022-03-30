pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::convert::TryInto;
use std::iter;

impl Solution {
    fn iter_bitmap(mut map: u32) -> impl Iterator<Item = u8> {
        iter::from_fn(move || {
            (map != 0).then(|| {
                let result = map.trailing_zeros() as _;

                map &= map - 1;

                result
            })
        })
    }

    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut nodes = 0u32;
        let mut graph = [0_u32; 26];
        let mut not_equals = Vec::new();

        for equation in &equations {
            let [lhs, operator, _, rhs]: [_; 4] = equation.as_bytes().try_into().unwrap();
            let (lhs, rhs) = (lhs - b'a', rhs - b'a');

            if operator == b'=' {
                graph[usize::from(lhs)] |= 1 << rhs;
                graph[usize::from(rhs)] |= 1 << lhs;
                nodes |= 1 << lhs;
                nodes |= 1 << rhs;
            } else {
                if lhs == rhs {
                    return false;
                }

                not_equals.push((lhs, rhs));
            }
        }

        let mut queue = VecDeque::new();
        let mut components = [0_u8; 26];
        let mut i = 0;

        for mut node in Self::iter_bitmap(nodes) {
            nodes &= nodes - 1;

            if let component @ 0 = &mut components[usize::from(node)] {
                i += 1;

                *component = i;

                loop {
                    for next in Self::iter_bitmap(graph[usize::from(node)]) {
                        if let neighbor_component @ 0 = &mut components[usize::from(next)] {
                            *neighbor_component = i;

                            queue.push_back(next);
                        }
                    }

                    if let Some(next) = queue.pop_front() {
                        node = next;
                    } else {
                        break;
                    }
                }
            }
        }

        for (lhs, rhs) in not_equals {
            let lhs_component = components[usize::from(lhs)];
            let rhs_component = components[usize::from(rhs)];

            if lhs_component != 0 && rhs_component != 0 && lhs_component == rhs_component {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn equations_possible(equations: Vec<String>) -> bool {
        Self::equations_possible(equations)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
