#[derive(Debug)]
struct Dados {
    comparacoes: i16,
    trocas: i16,
}

impl std::fmt::Display for Dados {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(Comparacoes: {}, Trocas: {})",
            self.comparacoes, self.trocas
        )
    }
}
pub fn bubblesort() {
    let mut numeros_da_ada = [21, 5, 9, 2];

    let mut dados = Dados {
        comparacoes: 0,
        trocas: 0,
    };
    
    print!("\n\n===\nArray antes: {:?}\n", numeros_da_ada);
    
    for i in 0..numeros_da_ada.len() {
        for j in 0..numeros_da_ada.len() - 1 - i {
            dados.comparacoes += 1;
            if numeros_da_ada[j] > numeros_da_ada[j + 1] {
                numeros_da_ada.swap(j, j + 1);
                dados.trocas += 1;
            }
        }
    }
    print!("\n===\nResultado: {:?}", dados);

}

fn main() {
    bubblesort();
}
