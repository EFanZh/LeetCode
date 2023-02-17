pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct Queue {
    buffer: [u32; 16],
    start: u8,
}

impl Queue {
    const MASK: u8 = 16 - 1;

    fn push_front(&mut self, value: u32) {
        self.start = self.start.wrapping_sub(1) & Self::MASK;
        self.buffer[usize::from(self.start)] = value;
    }

    fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        (self.start..).map(move |i| self.buffer[usize::from(i & Self::MASK)])
    }

    fn first(&self) -> u32 {
        self.buffer[usize::from(self.start)]
    }
}

impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as u32;
        let mut cache = Queue::default();

        cache.push_front(1);

        for (i, c) in s.iter().copied().enumerate().rev() {
            let mut count = 0;

            if c != b'0' {
                let mut value = u32::from(c - b'0');

                if value <= k {
                    let mut iter = s[i..].iter().copied().zip(cache.iter());

                    count = iter.next().unwrap().1;

                    for (c, prev) in iter {
                        if let Some(next_value) = value
                            .checked_mul(10)
                            .and_then(|x| x.checked_add(u32::from(c - b'0')))
                            .filter(|&x| x <= k)
                        {
                            value = next_value;
                            count += prev;
                            count = count.checked_sub(1_000_000_007).unwrap_or(count);
                        } else {
                            break;
                        }
                    }
                }
            }

            cache.push_front(count);
        }

        cache.first() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_arrays(s: String, k: i32) -> i32 {
        Self::number_of_arrays(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
