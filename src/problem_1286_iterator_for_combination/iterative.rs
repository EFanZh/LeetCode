// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct CombinationIterator {
    characters: Box<[u8]>,
    buffer: Box<[u8]>,
    i: usize,
}

impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let combination_length = combination_length as usize;
        let mut buffer = (0..combination_length as u8).collect::<Box<_>>();

        *buffer.last_mut().unwrap() = combination_length.wrapping_sub(2) as _;

        Self {
            characters: characters.into_bytes().into_boxed_slice(),
            buffer,
            i: combination_length - 1,
        }
    }

    fn next(&mut self) -> String {
        let characters = self.characters.as_ref();
        let buffer = self.buffer.as_mut();
        let i = &mut self.i;
        let max_first = (characters.len() - buffer.len()) as u8;

        loop {
            let value = &mut buffer[*i];

            if *value == max_first + *i as u8 {
                *i -= 1;
            } else {
                *value = value.wrapping_add(1);

                let mut prev = *value;

                for target in &mut buffer[*i + 1..] {
                    prev += 1;
                    *target = prev;
                }

                *i = buffer.len() - 1;

                break;
            }
        }

        self.buffer
            .iter()
            .map(|&i| char::from(self.characters[usize::from(i)]))
            .collect()
    }

    fn has_next(&self) -> bool {
        self.buffer[0] != (self.characters.len() - self.buffer.len()) as u8
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::CombinationIterator for CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        Self::new(characters, combination_length)
    }

    fn next(&mut self) -> String {
        self.next()
    }

    fn has_next(&self) -> bool {
        self.has_next()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::CombinationIterator>();
    }
}
