pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(Clone)]
struct State {
    buffer: [u8; 20],
    length: u8,
}

impl State {
    fn from_slice(s: &[u8]) -> Self {
        let mut buffer = [0; 20];

        buffer[..s.len()].copy_from_slice(s);

        Self {
            buffer,
            length: s.len() as _,
        }
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer[..usize::from(self.length)]
    }

    fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer[..usize::from(self.length)]
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.buffer().hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.buffer() == other.buffer()
    }
}

impl Eq for State {}

impl Solution {
    pub fn k_similarity(s1: String, s2: String) -> i32 {
        let mut result = 0;

        if s1 != s2 {
            let mut queue = VecDeque::from(vec![State::from_slice(s1.as_bytes())]);
            let mut visited = HashSet::new();
            let target = s2.as_bytes();

            loop {
                result += 1;

                for _ in 0..queue.len() {
                    let mut state = queue.pop_front().unwrap();
                    let buffer = state.buffer_mut();
                    let i = buffer.iter().zip(target).position(|(lhs, rhs)| lhs != rhs).unwrap();
                    let expected = target[i];

                    for j in i + 1..buffer.len() {
                        if buffer[j] == expected {
                            buffer.swap(i, j);

                            if buffer == target {
                                return result;
                            }

                            let next_state = State::from_slice(buffer);

                            if visited.insert(next_state.clone()) {
                                queue.push_back(next_state);
                            }

                            buffer.swap(i, j);
                        }
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_similarity(s1: String, s2: String) -> i32 {
        Self::k_similarity(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
