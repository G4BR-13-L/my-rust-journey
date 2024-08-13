use core::num;
use std::ops::Div;

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut mesclado = [nums1, nums2].concat();
    let comprimento = mesclado.len();
    mesclado.sort();
    let indice_medio = comprimento / 2;
    if comprimento % 2 == 0 {
        let maior_valor_esquerda = mesclado[ indice_medio - 1 ];
        let menor_valor_direita = mesclado[ indice_medio ];
        return (maior_valor_esquerda as f64 + menor_valor_direita as f64) / 2.0;
    }else{
        return mesclado[indice_medio] as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::other::leet004_median_of_two_sorted_arrays::find_median_sorted_arrays;

    #[test]
    fn sample_tests() {
        assert_eq!(find_median_sorted_arrays(vec![1,3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
    }
}