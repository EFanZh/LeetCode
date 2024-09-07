// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct Bitset {
    data: Box<[u64]>,
    length: usize,
}

impl Bitset {
    fn new(size: i32) -> Self {
        let length = size as u32 as usize;

        Self {
            data: vec![0; (length + 63) / 64].into_boxed_slice(),
            length,
        }
    }

    fn fix(&mut self, idx: i32) {
        let idx = idx as u32 as usize;

        self.data[idx / 64] |= 1 << (idx % 64);
    }

    fn unfix(&mut self, idx: i32) {
        let idx = idx as u32 as usize;

        self.data[idx / 64] &= !(1 << (idx % 64));
    }

    fn flip(&mut self) {
        for x in &mut self.data[..self.length / 64] {
            *x = !*x;
        }

        let remainder = self.length % 64;

        if remainder != 0 {
            let mask = (1 << remainder) - 1;
            let last = self.data.last_mut().unwrap();

            *last = !*last & mask;
        }
    }

    fn all(&self) -> bool {
        self.data[..self.length / 64].iter().all(|&x| x == u64::MAX) && {
            let remainder = self.length % 64;

            remainder == 0 || {
                let mask = (1 << remainder) - 1;

                *self.data.last().unwrap() == mask
            }
        }
    }

    fn one(&self) -> bool {
        self.data.iter().any(|&x| x != 0)
    }

    fn count(&self) -> i32 {
        self.data.iter().map(|x| x.count_ones()).sum::<u32>() as _
    }

    #[expect(clippy::inherent_to_string, reason = "required")]
    fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.length);

        for &x in &self.data[..self.length / 64] {
            result.extend((0..64).map(|i| char::from(b'0' + u8::from(x & (1 << i) != 0))));
        }

        let remainder = self.length % 64;

        if remainder != 0 {
            let x = *self.data.last().unwrap();

            result.extend((0..remainder).map(|i| char::from(b'0' + u8::from(x & (1 << i) != 0))));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Bitset for Bitset {
    fn new(size: i32) -> Self {
        Self::new(size)
    }

    fn fix(&mut self, idx: i32) {
        self.fix(idx);
    }

    fn unfix(&mut self, idx: i32) {
        self.unfix(idx);
    }

    fn flip(&mut self) {
        self.flip();
    }

    fn all(&self) -> bool {
        self.all()
    }

    fn one(&self) -> bool {
        self.one()
    }

    fn count(&self) -> i32 {
        self.count()
    }

    fn to_string(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Bitset>();
    }
}
