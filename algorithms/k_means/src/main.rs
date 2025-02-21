use std::f32::INFINITY;

use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Coordenada {
    x: i32,
    y: i32,
}

fn selecionar_k_pontos_aleatorios(coordenadas: Vec<Coordenada>, k: i32) -> Vec<Coordenada> {
    let mut pontos_aleatorios: Vec<Coordenada> = Vec::with_capacity(k as usize);

    for i in 0..k {
        let index = rand::random_range(0..k);
        pontos_aleatorios.push(
            *coordenadas
                .get(index as usize)
                .expect("Tem que ter algum valor aqui, pelo amor..."),
        );
    }

    return pontos_aleatorios;
}

fn calcular_distancia(ponto: Coordenada, centroide: Coordenada) -> f64 {
    let d = (centroide.x - ponto.x).pow(2) + ().pow(2);
}

fn k_means(coordenadas: Vec<Coordenada>, k: i32, max_iteracoes: i32, tolerancia: f32) {
    // Passo 1
    let centroides = selecionar_k_pontos_aleatorios(coordenadas, k);

    for interacao in 0..max_iteracoes {
        let cluster: Vec<Vec<Coordenada>> = vec![vec![]; k as usize];

        for ponto in &coordenadas {
            // adicione & para referenciar os pontos
            let mut distancia_minima = f64::INFINITY; // adicione mut e use f64::INFINITY
            let mut cluster_mais_proximo = -1; // adicione mut
            for (indice, centroide) in centroides.iter().enumerate() {
                // use enumerate() para obter o índice e o valor
                distancia = calcular_distancia(ponto, centroide);
            }
        }
    }

    println!("K-means");
}

fn main() {
    println!("Hello, world!");
}
