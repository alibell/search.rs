pub struct TokenIterator<'a>  {
    s: &'a String,
    split_idx: usize, 
    split_indexes: Vec<(usize, usize)>,
    n_idx: usize
}

impl TokenIterator<'_> {
    pub fn new <'a> (s: &'a String, split_pattern: &'a String) -> TokenIterator<'a>  {
        // Compute indexes
        let mut split_indexes: Vec<(usize, usize)> = Vec::new();
        let document_len: usize = s.chars().count();
        let pattern_size: usize = split_pattern.chars().count();
        let mut idx: usize = 0;

        let mut previous_idx: usize = 0;
        loop {
            let current_string: String = s.chars().skip(idx).take(pattern_size).collect();

            if current_string == *split_pattern {
                split_indexes.push(
                    (previous_idx, idx)
                );

                previous_idx = idx + pattern_size;
            }

            idx += 1;

            if idx > document_len {
                if previous_idx != document_len {
                    split_indexes.push(
                        (previous_idx, document_len)
                    );    
                }

                break;
            }
        }

        let n_idx = split_indexes.len();

        TokenIterator{
            s: s,
            split_idx: 0,
            split_indexes: split_indexes,
            n_idx: n_idx
        }
    }
}

/// Iterator for TextIterator
///
/// Generate a list of tokens to evaluate for string match search

impl<'a> Iterator for TokenIterator<'a> {
    type Item = (String, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.split_idx < self.n_idx {
            let entry: (usize, usize) = self.split_indexes[self.split_idx];
            let content = self.s.chars().skip(entry.0).take(entry.1 - entry.0).collect();
            
            self.split_idx += 1;

            Some( 
                (content, entry.0, entry.1)
            )
        } else {
            return None
        }
    }
}