pub(crate) fn vetores() {
    let mut notas: Vec<f32> = vec![1.2, 3.4, 5.6, 7.8];

    // get() Verifica se a posição no vetor existe para não dar erro
    println!("{}", match notas.get(7) {
        Some(n) => *n,
        None => 0.0
    })
}
