fn digital_root(n: i64) -> i64 {
    let numbers: Vec<i64> = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
    let mut sum: i64 = 0;
    for digit in &numbers {
        sum += *digit;
    }
    if sum > 9 {
        digital_root(sum)
    }else{
        return sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
      assert_eq!(digital_root(16), 7);
    }    
}