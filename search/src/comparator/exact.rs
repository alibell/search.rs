/// search
///
/// Perform an exact search
/// 
/// * `string1` - Text to compare
/// * `string2` - Text to compare with
///
/// Return a tuple of result (bool) and score

pub fn search(
    text1: &String,
    text2: &String
) -> (bool, f64) {
    let res: bool = text1.eq(text2);

    if res {
        return (res, 0.0);
    } else {
        return (res, 1.0);
    }
}
