// exact_token_extract
// Perform an exact extract of a token according to its match
/// 
/// * `s` - Text containing values to extract
/// * `pattern` - Pattern to extract
/// * `tokenizer_pattern` - Pattern for the tokenizer
/// * `case_insensitive` - If true, we perform a case insensitive search
/// * `special_char_insensitive` - If true, we normalize special char and accentuated characters
///
/// Return a vector containing start, end offset and match score, here always 1
pub fn extract(
    mut s: String, 
    mut pattern: String, 
    mut tokenizer_pattern: String,
    case_insensitive: bool, 
    special_char_insensitive: bool
) -> Vec<(usize, usize, f64)> {
    // Pre-processing of the string
    s = crate::preprocessor::preprocess(s, case_insensitive, special_char_insensitive);
    pattern = crate::preprocessor::preprocess(pattern, case_insensitive, special_char_insensitive);
    tokenizer_pattern = crate::preprocessor::preprocess(tokenizer_pattern, case_insensitive, special_char_insensitive);

    // We iterate over the string and we store the offsets
    let mut offsets: Vec<(usize, usize, f64)> = Vec::new();
    let n_tokens: usize = pattern
                    .split(&tokenizer_pattern)
                    .filter(|s| *s != tokenizer_pattern)
                    .collect::<Vec<&str>>()
                    .len();

    let iterator = crate::iterator::tokens::TokenIterator::new(
        &s,
        &tokenizer_pattern,
        n_tokens
    );

    pattern = crate::preprocessor::trim_text(pattern);
    for (mut candidate, start, end) in iterator {
        candidate = crate::preprocessor::trim_text(candidate);

        let (res, score) = crate::comparator::exact::search(
            &candidate,
            &pattern
        );

        // We store the responses
        if res {
            offsets.push(
                (start, end, score)
            )
        }
        
    }

    return offsets
}