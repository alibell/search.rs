use rapidfuzz::distance::{damerau_levenshtein, levenshtein, hamming, indel, jaro, jaro_winkler, lcs_seq, osa, postfix, prefix};
use std::cmp;

// @TODO :
// Use score cut-off to drop computation when there is a big difference
// Use batch comparator to optimize computation

fn apply_method(
    text1: &String,
    text2: &String,
    metric: &String,
    threshold_kind: u8
) -> f64 {
    let text_size: usize = cmp::max(
        text1.len(),
        text2.len()
    );

    if metric == "damerau_levenshtein" {
        if threshold_kind == 0 {
            return damerau_levenshtein::distance(text1.chars(), text2.chars()) as f64;
        } else {
            return damerau_levenshtein::normalized_distance(text1.chars(), text2.chars());
        }
    }
    else if metric == "levenshtein" {
        if threshold_kind == 0 {
            return levenshtein::distance(text1.chars(), text2.chars()) as f64;
        } else {
            return levenshtein::normalized_distance(text1.chars(), text2.chars());
        }
    }
    else if metric == "hamming" {
        if threshold_kind == 0 {
            return match hamming::distance(text1.chars(), text2.chars()) {
                Ok(val) => val,
                Err(_) => text_size
            } as f64;
        } else {
            return match hamming::normalized_distance(text1.chars(), text2.chars()) {
                Ok(val) => val,
                Err(_) => 1.0
            };
        }
    }
    else if metric == "indel" {
        if threshold_kind == 0 {
            return indel::distance(text1.chars(), text2.chars()) as f64;
        } else {
            return indel::normalized_distance(text1.chars(), text2.chars());
        }
    }
    else if metric == "jaro" {
        if threshold_kind == 0 {
            return jaro::distance(text1.chars(), text2.chars()) as f64;
        } else {
            return jaro::normalized_distance(text1.chars(), text2.chars());
        }
    }
    else if metric == "jaro_winkler" {
        if threshold_kind == 0 {
            return jaro_winkler::distance(text1.chars(), text2.chars()) as f64;
        } else {
            return jaro_winkler::normalized_distance(text1.chars(), text2.chars());
        }
    }
    else if metric == "lcs_seq" {
        if threshold_kind == 0 {
            return lcs_seq::distance(text1.chars(), text2.chars()) as f64;
        } else {
            return lcs_seq::normalized_distance(text1.chars(), text2.chars());
        }
    }
    else if metric == "osa" {
        if threshold_kind == 0 {
            return osa::distance(text1.chars(), text2.chars()) as f64;
        } else {
            return osa::normalized_distance(text1.chars(), text2.chars());
        }
    }
    else if metric == "postfix" {
        if threshold_kind == 0 {
            return postfix::distance(text1.chars(), text2.chars()) as f64;
        } else {
            return postfix::normalized_distance(text1.chars(), text2.chars());
        }
    }
    else if metric == "prefix" {
        if threshold_kind == 0 {
            return prefix::distance(text1.chars(), text2.chars()) as f64;
        } else {
            return prefix::normalized_distance(text1.chars(), text2.chars());
        }
    }
    else if metric == "levenshtein" {
        if threshold_kind == 0 {
            return levenshtein::distance(text1.chars(), text2.chars()) as f64;
        } else {
            return prefix::normalized_distance(text1.chars(), text2.chars());
        }
    }
    else {
        panic!("Invalid distance metric")
    }
}

/// search
///
/// Perform a fuzzy search
/// 
/// * `string1` - Text to compare
/// * `string2` - Text to compare with
/// * `metric` - Metric to use, one of the followwing: damerau_levenshtein, levenshtein, hamming, indel, jaro, jaro_winkler, lcs_seq, osa, postfix, prefix
/// * `threshold` - Threshold for fuzzy search
/// * `threshold_kind` - Kind of providen threshold, 0 if absolute, 1 if normalized (between 0 and one)
///
/// Return a tuple of result (bool) and score
pub fn search(
    text1: &String,
    text2: &String,
    metric: &String,
    threshold: f64,
    threshold_kind: u8
) -> (bool, f64) {
    // Compute distance
    let res: f64 = apply_method(&text1, &text2, &metric, threshold_kind);

    // Result result and score
    let output: (bool, f64) = (res <= threshold, res);
    output 
}
