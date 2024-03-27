use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let mut map = HashMap::new();

    map.insert('2', vec!['a', 'b', 'c']);
    map.insert('3', vec!['d', 'e', 'f']);
    map.insert('4', vec!['g', 'h', 'i']);
    map.insert('5', vec!['j', 'k', 'l']);
    map.insert('6', vec!['m', 'n', 'o']);
    map.insert('7', vec!['p', 'q', 'r', 's']);
    map.insert('8', vec!['t', 'u', 'v']);
    map.insert('9', vec!['w', 'x', 'y', 'z']);

    let digits_arr: Vec<char> = digits.chars().collect();
    let mut result: Vec<String> = Vec::new();

    if digits.len() == 1 {
        let letters = map.get(&digits_arr[0]).unwrap();
        for i in 0..letters.len() {
            result.push(letters[i].to_string());
        }
        return result;
    }

    if digits.len() == 2 {
        let letters1 = map.get(&digits_arr[0]).unwrap();
        let letters2 = map.get(&digits_arr[1]).unwrap();
        for i in 0..letters1.len() {
            for j in 0..letters2.len() {
                result.push(format!("{}{}", letters1[i], letters2[j]));
            }
        }
        return result;
    }

    if digits.len() == 3 {
        let letters1 = map.get(&digits_arr[0]).unwrap();
        let letters2 = map.get(&digits_arr[1]).unwrap();
        let letters3 = map.get(&digits_arr[2]).unwrap();
        for i in 0..letters1.len() {
            for j in 0..letters2.len() {
                for k in 0..letters3.len() {
                    result.push(format!("{}{}{}", letters1[i], letters2[j], letters3[k]));
                }
            }
        }
        return result;
    }

    if digits.len() == 4 {
        let letters1 = map.get(&digits_arr[0]).unwrap();
        let letters2 = map.get(&digits_arr[1]).unwrap();
        let letters3 = map.get(&digits_arr[2]).unwrap();
        let letters4 = map.get(&digits_arr[3]).unwrap();
        for i in 0..letters1.len() {
            for j in 0..letters2.len() {
                for k in 0..letters3.len() {
                    for l in 0..letters4.len() {
                        result.push(format!("{}{}{}{}", letters1[i], letters2[j], letters3[k], letters4[l]));
                    }
                }
            }
        }
        return result;
    }
    vec![] // Se chegarmos aqui, retornamos um vetor vazio.
}