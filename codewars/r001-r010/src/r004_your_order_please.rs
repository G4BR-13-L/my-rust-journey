// Your task is to sort a given string. Each word in the string will contain a single number. This number is the position the word should have in the result.

// Note: Numbers can be from 1 to 9. So 1 will be the first word (not 0).

// If the input string is empty, return an empty string. The words in the input String will only contain valid consecutive numbers.
// Examples

// "is2 Thi1s T4est 3a"  -->  "Thi1s is2 3a T4est"
// "4of Fo1r pe6ople g3ood th5e the2"  -->  "Fo1r the2 g3ood 4of th5e pe6ople"
// ""  -->  ""


fn order(sentence: &str) -> String {
    if sentence == "" {
        return "".to_string();
    }
    let words: Vec<String> = sentence.split(" ").map(|s| s.to_string()).collect();
    let mut sort_words: Vec<(i8,String)> = vec![];

    for i in 0..words.len() {
        let word_position: i8 = words[i].chars().find_map(|c| c.to_digit(10).map(|d| d as i8)).unwrap_or_default();
        sort_words.push((word_position, words[i].to_string()))
    }
    sort_words.sort_by_key(|w| w.0);
    return sort_words.iter().map(|s| s.1.clone()).collect::<Vec<String>>().join(" ");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
        assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
        assert_eq!(order(""), "");
    }
}
