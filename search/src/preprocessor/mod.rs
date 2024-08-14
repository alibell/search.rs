fn lower_case(s: String) -> String {
    return s.to_lowercase()
}

fn to_lf(s: String) -> String {
    return s.replace("\r", "")
}

fn line_break_to_space(s: String) -> String {
    return s.replace("\n", " ")
}

pub fn trim_text(s: String) -> String {
    return s.trim().to_owned();
}

fn replace_special_char(s: String) -> String {
    return s.chars().map(|x| match x { 
        'à' => 'a',
        'â' => 'a',
        'ä' => 'a',
        'é' => 'e',
        'ê' => 'e',
        'ë' => 'e',
        'è' => 'e',
        '€' => 'e',
        'ù' => 'u',
        'ü' => 'u',
        'û' => 'u',
        'ï' => 'i',
        'ç' => 'c',
        '@' => 'a',
        _ => x
    }).collect();    
}

pub fn preprocess (mut s: String, ignore_case: bool, ignore_special_char: bool) -> String {
    s = to_lf(s);
    s = line_break_to_space(s);

    if ignore_case {
        s = lower_case(s);
    }


    if ignore_special_char {
        s = replace_special_char(s);
    }

    return s
}