// exact_raw_extract
// Perform an exact extract of a text according to its match
/// 
/// * `s` - Text containing values to extract
/// * `pattern` - Pattern to extract
/// * `case_insensitive` - If true, we perform a case insensitive search
/// * `special_char_insensitive` - If true, we normalize special char and accentuated characters
/// * `metric` - Metric used to evaluate the distance between the candidates and the pattern
/// * `threshold` - Threshold of the metric for evaluate if the candidate if valid of not
/// * `threshold_kind` - Kind of providen threshold, 0 if absolute, 1 if normalized (between 0 and one)
/// * `strategies` - Vector of strategies for best match pick in applyied order, one of followings: best_match, minimum_isolated_character, shortest, longest
///
/// Return a vector containing start, end offset and match score, here always 1

pub fn extract(
    mut s: String, 
    mut pattern: String, 
    case_insensitive: bool, 
    special_char_insensitive: bool,
    metric: String,
    threshold: f64,
    threshold_kind: u8,
    strategies: Vec<String>
) -> Vec<(usize, usize, f64)> {
    // Pre-processing of the string
    s = crate::preprocessor::preprocess(s, case_insensitive, special_char_insensitive);
    pattern = crate::preprocessor::preprocess(pattern, case_insensitive, special_char_insensitive);
    let pattern_size: usize = pattern.chars().count();
    let delta: i8;

    if threshold_kind == 0 {
        delta = threshold_kind as i8;
    } else {
        delta = (threshold*(pattern_size as f64)).trunc() as i8;
    }

    // We iterate over the string and we store the offsets
    let mut offsets: Vec<(usize, usize, f64)> = Vec::new();
    let iterator = crate::iterator::text::TextIterator::new(
        &s,
        pattern_size,
        delta
    );

    pattern = crate::preprocessor::trim_text(pattern);
    for (mut candidate, start, end) in iterator {
        candidate = crate::preprocessor::trim_text(candidate);

        let (res, score) = crate::comparator::fuzzy::search(
            &candidate,
            &pattern,
            &metric,
            threshold,
            threshold_kind
        );

        // We store the responses
        if res {
            offsets.push(
                (start, end, score)
            )
        }
        
    }

    // Resolving conflicts
    offsets = crate::conflict_solver::solver(
        &s,
        offsets,
        &strategies
    );

    return offsets
}