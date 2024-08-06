

pub fn media(lista: &Vec<f64>) -> f64 {
    let soma:f64 = lista.iter().sum();
    return soma / lista.len() as f64;
}

pub fn desvio_padrao(lista: &Vec<f64>) -> f64{

    let media = media(lista) as f64;
    let mut somatorio:f64  = 0.0;
    for numero in lista{
        somatorio += (numero - media).powf(2.0) as f64;
    }

    return f64::sqrt(somatorio/media);
}

pub fn coeficiente_variacao(lista: &Vec<f64>) -> f64{
    return desvio_padrao(lista) / media(lista)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_simple_tests() {
        assert_eq!(media(&vec![2.0, 2.0]), 2.0);
        assert_eq!(media(&vec![2.0, 2.0, 2.0]), 2.0);
        assert_eq!(media(&vec![10.0]), 10.0);
        assert_eq!(media(&vec![20.0, 2.0]), 11.0);
        assert_eq!(media(&vec![-2.0, 2.0]), 0.0);
        assert_eq!(media(&vec![1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
        assert_eq!(media(&vec![0.0, 0.0, 0.0]), 0.0);
        assert_eq!(media(&vec![100.0, 200.0, 300.0]), 200.0);
        assert_eq!(media(&vec![-1.0, -1.0, -1.0]), -1.0);
        assert_eq!(media(&vec![0.0]), 0.0);
        assert_eq!(media(&vec![5.0, 10.0, 15.0]), 10.0);
        assert_eq!(media(&vec![4.0, 8.0, 12.0, 16.0]), 10.0);
        assert_eq!(media(&vec![-10.0, 0.0, 10.0]), 0.0);
        assert_eq!(media(&vec![1.0, 2.0]), 1.5);
        assert_eq!(media(&vec![1.0, 3.0]), 2.0);
    }

    #[test]
    fn test_desvio_padrao() {
        assert_eq!(desvio_padrao(&vec![2.0, 2.0]), 0.0);
        assert_eq!(desvio_padrao(&vec![5.0, 5.0, 5.0, 5.0, 5.0]), 0.0);
        assert_eq!(desvio_padrao(&vec![10.0]), 0.0);
        assert_eq!(desvio_padrao(&vec![1.0, 100.0]), 9.850868183078893);
    }


    #[test]
    fn test_coeficiente_variacao() {
        assert_eq!(coeficiente_variacao(&vec![2.0, 2.0]), 0.0);
        assert_eq!(coeficiente_variacao(&vec![5.0, 5.0, 5.0, 5.0, 5.0]), 0.0);
        assert_eq!(coeficiente_variacao(&vec![10.0]), 0.0);
    }
}