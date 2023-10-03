// Write a program that will calculate the number of trailing zeros in a factorial of a given number.

// N! = 1 * 2 * 3 *  ... * N

// Be careful 1000! has 2568 digits...

// For more info, see: http://mathworld.wolfram.com/Factorial.html
// Examples

// zeros(6) = 1
// # 6! = 1 * 2 * 3 * 4 * 5 * 6 = 720 --> 1 trailing zero

// zeros(12) = 2
// # 12! = 479001600 --> 2 trailing zeros

// Hint: You're not meant to calculate the factorial. Find another way to find the number of zeros.


// fn zeros(n: u64) -> u64 {
//     if n == 0 { return 0; }
//     let mut factorial: u128 = n.into();
//     let n128: u128 = n.into();
//     for i in (1..n128).rev(){
//         factorial *= i;
//     }
//     let factorial_str: Vec<char> = factorial.to_string().chars().collect();
//     let mut zeros_count: u64 = 0;
//     for c in (0..factorial_str.len()).rev(){
//         if factorial_str[c] == '0' {
//             zeros_count+=1;
//         }else{
//             break;
//         }
//     }
//     return zeros_count;   
// }

// fn zeros(n: u64) -> u64 {
//     if n == 0 { return 0; }
//     let mut factorial: u128 = n.into();
//     let n128: u128 = n.into();
//     let mut zeros_count: u64 = 0;
//     for i in (1..n128).rev(){
//         factorial *= i;
//         if factorial % 10 == 0 {
//             factorial /= 10;
//             zeros_count+=1;
//         }
//     }
//     return zeros_count;   
// }

fn zeros(n: u64) -> u64 {
    if n == 0 { return 0; }
    else { 
        return n / 5 + zeros( n/5 ) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        // // assert_eq!(zeros(0), 0);
        // assert_eq!(zeros(6), 1);
        // assert_eq!(zeros(14), 2);
        // assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }    
}