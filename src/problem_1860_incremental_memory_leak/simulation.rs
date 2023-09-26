pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        let mut memory1 = memory1;
        let mut memory2 = memory2;
        let mut time = 1;

        loop {
            if memory1 < memory2 {
                if time <= memory2 {
                    memory2 -= time;
                } else {
                    break;
                }
            } else if time <= memory1 {
                memory1 -= time;
            } else {
                break;
            }

            time += 1;
        }

        vec![time, memory1, memory2]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        Self::mem_leak(memory1, memory2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
