use core::num;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut caracteres: Vec<char> = Vec::new();
    let mut vec_contador: Vec<i32> = Vec::new();
    let mut contador: i32 = 0;
    let s_list: Vec<char> = s.chars().collect();
    if s_list.len() == 1 {
        return 1;
    }

    for i in 0..s_list.len() {
        if caracteres.contains(&s_list[i]) {
            caracteres.clear();
            vec_contador.push(contador);
            contador = 0;
        }
        contador += 1;
        caracteres.push(s_list[i].clone());
    }

    let max = *vec_contador.iter().max().unwrap_or(&0);
    for n in vec_contador {
        print!("\n\n\n\n{}\n\n\n\n", n);
    }
    print!("\n\n\n\n{}\n\n\n\n", max);
    return max;
}

#[cfg(test)]
mod tests {
    use crate::other::leet003_longest_substring_without_repeating_characters::length_of_longest_substring;

    #[test]
    fn sample_tests() {
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);

        // TODO Caso de teste ainda não resolvido
        assert_eq!(length_of_longest_substring(String::from("au")), 2);
        assert_eq!(length_of_longest_substring(String::from(" ")), 1);
        assert_eq!(length_of_longest_substring(String::from("c")), 1);
    }
}
