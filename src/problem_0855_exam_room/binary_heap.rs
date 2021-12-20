// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Clone)]
struct Chunk {
    start: u32,
    end: u32,
}

impl Chunk {
    fn end(&self) -> u32 {
        self.end & 0x7fff_ffff
    }

    fn seat(&self) -> u32 {
        if self.start == 0 {
            0
        } else if self.end & 0x8000_0000 == 0 {
            (self.start + self.end) / 2
        } else {
            self.end()
        }
    }

    fn key(&self) -> (Reverse<u32>, u32) {
        let distance = if self.start == 0 {
            self.end
        } else if self.end & 0x8000_0000 == 0 {
            (self.end - self.start) / 2
        } else {
            self.end() - self.start
        };

        (Reverse(distance), self.start)
    }
}

pub struct ExamRoom {
    heap: Vec<Chunk>,
    starts: HashMap<u32, usize>,
    ends: HashMap<u32, usize>,
    n: u32,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        let n = n as u32 - 1;

        Self {
            heap: vec![Chunk {
                start: 0,
                end: n | 0x8000_0000,
            }],
            starts: Some((0, 0)).into_iter().collect(),
            ends: Some((n, 0)).into_iter().collect(),
            n,
        }
    }

    fn set_chunk(&mut self, index: usize, value: Chunk) {
        self.starts.insert(value.start, index);
        self.ends.insert(value.end(), index);
        self.heap[index] = value;
    }

    fn heap_sift_up(&mut self, mut index: usize, value: Chunk) {
        loop {
            let parent_index = index.wrapping_sub(1) / 2;

            if let Some(parent) = self.heap.get(parent_index).cloned() {
                if value.key() < parent.key() {
                    self.set_chunk(index, parent);

                    index = parent_index;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        self.set_chunk(index, value);
    }

    fn heap_sift_down(&mut self, mut index: usize, value: Chunk) {
        loop {
            let left_index = index * 2 + 1;

            if let Some(left_value) = self.heap.get(left_index).cloned() {
                let right_index = left_index + 1;

                if let Some(right_value) = self.heap.get(right_index).cloned() {
                    let (child_index, child_value) = if right_value.key() < left_value.key() {
                        (right_index, right_value)
                    } else {
                        (left_index, left_value)
                    };

                    if child_value.key() < value.key() {
                        self.set_chunk(index, child_value);

                        index = child_index;
                    } else {
                        break;
                    }
                } else {
                    if left_value.key() < value.key() {
                        self.set_chunk(index, left_value);

                        index = left_index;
                    }

                    break;
                }
            } else {
                break;
            }
        }

        self.set_chunk(index, value);
    }

    fn heap_remove(&mut self, index: usize) -> Chunk {
        let replace_value = self.heap.pop().unwrap();

        let result = if index == self.heap.len() {
            replace_value
        } else {
            let result = self.heap[index].clone();

            if replace_value.key() < result.key() {
                self.heap_sift_up(index, replace_value);
            } else {
                self.heap_sift_down(index, replace_value);
            }

            result
        };

        self.starts.remove(&result.start);
        self.ends.remove(&result.end());

        result
    }

    fn heap_insert(&mut self, value: Chunk) {
        let index = self.heap.len();

        self.heap.push(Chunk { start: 0, end: 0 }); // The value here does not matter.
        self.heap_sift_up(index, value);
    }

    fn seat(&mut self) -> i32 {
        let chunk = self.heap[0].clone();
        let seat = chunk.seat();

        if seat == chunk.start {
            if seat == chunk.end() {
                self.heap_remove(self.starts[&seat]);
            } else {
                self.starts.remove(&seat);

                self.heap_sift_down(
                    0,
                    Chunk {
                        start: chunk.start + 1,
                        ..chunk
                    },
                );
            }
        } else if seat == chunk.end() {
            self.ends.remove(&chunk.end());

            self.heap_sift_down(
                0,
                Chunk {
                    end: chunk.end() - 1,
                    ..chunk
                },
            );
        } else {
            self.heap_sift_down(0, Chunk { end: seat - 1, ..chunk });

            self.heap_insert(Chunk {
                start: seat + 1,
                ..chunk
            });
        }

        seat as _
    }

    fn leave(&mut self, p: i32) {
        let p = p as u32;

        if let Some(&left_chunk_index) = self.ends.get(&(p.wrapping_sub(1))) {
            if self.starts.contains_key(&(p + 1)) {
                let left_chunk = self.heap_remove(left_chunk_index);
                let right_chunk_index = self.starts.remove(&(p + 1)).unwrap();

                self.heap_sift_up(
                    right_chunk_index,
                    Chunk {
                        end: self.heap[right_chunk_index].end,
                        ..left_chunk
                    },
                );
            } else {
                self.ends.remove(&(p - 1));

                self.heap_sift_up(
                    left_chunk_index,
                    Chunk {
                        start: self.heap[left_chunk_index].start,
                        end: if p == self.n { p | 0x8000_0000 } else { p },
                    },
                );
            }
        } else if let Some(right_chunk_index) = self.starts.remove(&(p + 1)) {
            let right_chunk_end = self.heap[right_chunk_index].end;

            self.heap_sift_up(
                right_chunk_index,
                Chunk {
                    start: p,
                    end: right_chunk_end,
                },
            );
        } else {
            self.heap_insert(Chunk {
                start: p,
                end: if p == self.n { p | 0x8000_0000 } else { p },
            });
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::ExamRoom for ExamRoom {
    fn new(n: i32) -> Self {
        Self::new(n)
    }

    fn seat(&mut self) -> i32 {
        self.seat()
    }

    fn leave(&mut self, p: i32) {
        self.leave(p);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::ExamRoom>();
    }
}
