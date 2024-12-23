pub(crate) fn vetores() {
    let mut notas: Vec<f32> = vec![1.2, 3.4, 5.6, 7.8];

    for nota in &notas {
        println!("Nota = {}", nota);
    } 
    println!("{:?}", notas);
}
