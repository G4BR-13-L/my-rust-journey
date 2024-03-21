pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut left: i32 = 0;
    let mut right: i32 = (letters.len() - 1) as i32;

    while left <= right {
        let middle = left + (right - left) / 2;
        if letters[middle as usize] <= target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }
    return letters[(left % letters.len() as i32) as usize];
}