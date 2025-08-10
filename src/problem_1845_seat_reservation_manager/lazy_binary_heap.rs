// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct SeatManager {
    heap: BinaryHeap<Reverse<i32>>,
    free: i32,
}

impl SeatManager {
    fn new(_: i32) -> Self {
        Self {
            heap: BinaryHeap::new(),
            free: 1,
        }
    }

    fn reserve(&mut self) -> i32 {
        if let Some(Reverse(result)) = self.heap.pop() {
            result
        } else {
            let result = self.free;

            self.free += 1;

            result
        }
    }

    fn unreserve(&mut self, seat_number: i32) {
        if seat_number + 1 == self.free {
            self.free -= 1;
        } else {
            self.heap.push(Reverse(seat_number));
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::SeatManager for SeatManager {
    fn new(n: i32) -> Self {
        Self::new(n)
    }

    fn reserve(&mut self) -> i32 {
        self.reserve()
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.unreserve(seat_number);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::SeatManager>();
    }
}
