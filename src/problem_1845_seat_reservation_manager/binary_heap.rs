// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct SeatManager {
    heap: BinaryHeap<Reverse<i32>>,
}

impl SeatManager {
    fn new(n: i32) -> Self {
        Self {
            heap: (1..=n).map(Reverse).collect(),
        }
    }

    fn reserve(&mut self) -> i32 {
        self.heap.pop().unwrap().0
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.heap.push(Reverse(seat_number));
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
