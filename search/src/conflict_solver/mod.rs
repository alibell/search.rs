use std::cmp;

fn is_overlap(e1: (usize, usize), e2: (usize, usize)) -> bool {
    let (e1_start, e1_end) = e1;
    let (e2_start, e2_end) = e2;

    let overlap_left: bool = e1_end >= e2_start && e1_start <= e2_start;
    let overlap_right: bool = e1_start <= e2_end && e1_end >= e2_end;
    let inner: bool = e1_start >= e2_start && e1_end <= e2_end;

    let overlap: bool = overlap_left || overlap_right || inner;

    return overlap
}

fn get_best_option(s: &String, options: &mut Vec<(usize, usize, f64)>, strategies: &Vec<String>) -> (usize, usize, f64) {    
    // We reduce the list of candidates
    for strategy in strategies {
        if options.len() <= 1 {
            break;
        }

        if strategy == "best_match" {
            let best_score: f64 = match options.into_iter().map(|x| x.2).min_by(|a, b| a.total_cmp(b))  {
                Some(min) => min,
                None => 0.0
            };

            for idx in (0..options.len()).rev() {
                if options[idx].2 != best_score {
                    options.remove(idx);
                }
            }
        } else if strategy == "minimum_isolated_character" {
            let mut number_isolated_character: Vec<usize> = Vec::new();

            for option_idx in 0..options.len() {            
                let start: usize = options[option_idx].0;
                let end: usize = options[option_idx].1;
                let mut n_isolated: usize = 0;

                // We loop over the string
                for idx in start..end {
                    let prev_char_idx: usize;
                    let next_char_idx: usize;

                    if idx > start {
                        prev_char_idx = idx - 1;
                    } else {
                        prev_char_idx = start;
                    }

                    if idx < end-1 {
                        next_char_idx = idx + 1;
                    } else {
                        next_char_idx = end-1;
                    }

                    if idx == start && next_char_idx < end && s.chars().nth(next_char_idx).unwrap() == ' ' {
                        n_isolated += 1;
                    } else if idx == end-1 && s.chars().nth(prev_char_idx).unwrap() == ' ' {
                        n_isolated += 1;
                    } else if next_char_idx < end && s.chars().nth(prev_char_idx).unwrap() == ' ' && s.chars().nth(next_char_idx).unwrap() == ' ' {
                        n_isolated += 1;
                    }
                }
    
                number_isolated_character.push(n_isolated);
            }

            let min_number_isolated_character: usize = match number_isolated_character.iter().min() {
                Some(min) => *min,
                None => 0
            };

            for idx in (0..options.len()).rev() {
                if number_isolated_character[idx] != min_number_isolated_character {
                    options.remove(idx);
                }
            }
        } else if strategy == "shortest" || strategy == "longest" {
            let size_vec: Vec<usize> = options.into_iter().map(|x| x.1 - x.0).collect();
            let size_threshold: usize;

            if strategy == "shortest" {
                size_threshold = match size_vec.iter().min() {
                    Some(min) => *min,
                    None => 0
                };
            } else {
                size_threshold = match size_vec.iter().max() {
                    Some(min) => *min,
                    None => 0
                };
            }

            for idx in (0..options.len()).rev() {
                if size_vec[idx] != size_threshold {
                    options.remove(idx);
                }
            }
        } else {
            for idx in (0..options.len()).rev() {
                if idx != 0 {
                    options.remove(idx);
                }
            }
        }
    }

    return options[0];
}

fn deduplicate(
    offsets: Vec<(usize, usize, f64)>
) -> Vec<(usize, usize, Vec<(usize, usize, f64)>)> {
    let mut offsets_deduplicated: Vec<(usize, usize, Vec<(usize, usize, f64)>)> = offsets
        .into_iter()
        .map(|x| {
            let (start, end, _) = x;
            let mut vec_list: Vec<(usize, usize, f64)> = Vec::new();
            vec_list.push(x);

            return (start, end, vec_list)
        }).collect();

    loop {
        // We loop over offsets untile there is no duplicate
        // We check for groups to fusion
        // We loop over groups until we cannont fusionate them
        let mut fusion_indexes: Option<(usize, usize)> = None;
        let mut previous_index: Option<usize> = None;
        let nb_offsets: usize = offsets_deduplicated.len();

        if nb_offsets == 0 {
            break;
        }

        for index in 0..offsets_deduplicated.len() {
            if previous_index.is_none() == false {
                let previous_index_value: usize = previous_index.unwrap();

                if is_overlap(
                    (offsets_deduplicated[index].0, offsets_deduplicated[index].1),
                    (offsets_deduplicated[previous_index_value].0, offsets_deduplicated[previous_index_value].1)
                ) {
                    fusion_indexes = Some(
                        (previous_index_value, index)
                    );

                    break;
                }
            }

            previous_index = Some(index);
        }

        // Break the loop if not changes performed
        // Otherwise, we perform the fusion
        if fusion_indexes.is_none() {
            break;
        } else {
            let (previous_index, index): (usize, usize) = fusion_indexes.unwrap();

            // Rewrite previous_index entry
            offsets_deduplicated[previous_index].0 = cmp::min(
                offsets_deduplicated[previous_index].0,
                offsets_deduplicated[index].0
            );

            offsets_deduplicated[previous_index].1 = cmp::max(
                offsets_deduplicated[previous_index].1,
                offsets_deduplicated[index].1
            );

            for val in offsets_deduplicated[index].2.clone() {
                offsets_deduplicated[previous_index].2.push(val);
            }

            // Remove index entry
            offsets_deduplicated.remove(index);
        }
    }

    return offsets_deduplicated
}

// solver
/// Resolve collision when many offsets has been extracted
/// Compute intersection between offsets and the apply prioritization rules in order to keep the best suitable entity
pub fn solver (s: &String, offsets: Vec<(usize, usize, f64)>, strategies: &Vec<String>) -> Vec<(usize, usize, f64)> {
    // Deduplicating offsets
    let offsets_deduplicated: Vec<(usize, usize, Vec<(usize, usize, f64)>)> = deduplicate(
        offsets
    );

    // For each deduplicated entry, we select the best option
    let mut output: Vec<(usize, usize, f64)> = Vec::new();
    
    for offset_deduplicated in offsets_deduplicated {
        output.push(
            get_best_option(&s, &mut offset_deduplicated.2.clone(), &strategies)
        );
    }

    return output
}