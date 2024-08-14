use std::cmp;

pub struct TextIterator<'a>  {
    s: &'a String,
    pattern_len: usize,
    delta: i8,
    char_idx: usize, 
    delta_idx: i8,
    len: usize
}

impl TextIterator<'_> {
    pub fn new <'a> (s: &'a String, pattern_len: usize, delta: i8) -> TextIterator<'a>  {
        let len: usize = s.chars().count();

        TextIterator{
            s: s,
            delta: delta,
            pattern_len: pattern_len,
            char_idx: 0,
            delta_idx: 0,
            len: len
        }
    }
}

/// Iterator for TextIterator
///
/// Generate a list of string to evaluate for string match search
/// To do this, we iterate over the string and according to the delta parameter

impl<'a> Iterator for TextIterator<'a> {
    type Item = (String, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.char_idx < self.len {
            // Slicing text
            let start = self.char_idx;
            let end_theorical: usize;

            if self.delta_idx.signum() == 1 {
                end_theorical = self.char_idx + self.pattern_len + self.delta_idx.abs() as usize;
            } else {
                end_theorical = self.char_idx + self.pattern_len - self.delta_idx.abs() as usize;
            }

            let end = cmp::min(
                cmp::max(
                    start,
                    end_theorical
                ),
                self.len
            );

            if end != start {
                let text_slice = self.s.chars().skip(start).take(end-start).collect();

                // Update char_idx and delta_idx
                if self.delta_idx >= self.delta {
                    self.char_idx += 1;
                    self.delta_idx = -1*cmp::min(self.pattern_len-1, self.delta as usize) as i8;
                } else {
                    if end != end_theorical || end_theorical == self.len-1 {
                        self.char_idx += 1;
                        self.delta_idx = -1*cmp::min(self.pattern_len-1, self.delta as usize) as i8;
                    } else {
                        self.delta_idx += 1;
                    }
                }
    
                Some((text_slice, start, end))
            } else {
                return None
            }
       } else {
            return None
        }
    }
}