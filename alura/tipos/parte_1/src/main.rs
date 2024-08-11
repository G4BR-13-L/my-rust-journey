fn main() {
    // array de f32
    let notas: [f32; 4] = [1f32, 2f32, 3f32, 4f32];

    for nota in notas {
        println!("Array1 = {}", nota);
    }

    // um array com 4 numeros 2
    let notas2: [f32; 4] = [2.3; 4];

    for nota in notas2 {
        println!("Array2 = {}", nota);
    }

    // Matrizes
    matriz();

    //Acessando elementos do array
    acessa_array();

    // Tipos de dados
    println!("é fim de semana? {}", is_fim_de_semana(DiaSemana::Sabado));

    cores();
}

fn matriz() {
    let matriz = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ];

    for linha in matriz {
        for coluna in linha {
            print!("{} ", coluna);
        }
        println!("")
    }
}

fn acessa_array() {
    // indices de array sempre são do tipo usize
    // usize são valores sem sinal. Por padrão, eles tem o limite padrão do sistema operacional: 64bits ou 32bits
    let notas3: [f32; 4] = [2.3; 4];
    let index: usize = 2;

    println!("Array3 = {}", notas3[index]);
}


enum DiaSemana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo
}
fn is_fim_de_semana(dia_da_semana: DiaSemana) -> bool {
    match dia_da_semana {
        DiaSemana::Sabado | DiaSemana::Domingo => true,
        _ => false
    }
}

enum Color{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8)
}

fn cores() {

    let cor = Color::Red;

    println!("Cor = {}", match cor {
        Color::Red => "Vermelho",
        Color::Green => "Verde",
        Color::Blue => "Azul",
        Color::RgbColor(0, 0, 0) => "Preto",
        Color::RgbColor(_, _, _) => "RGB({}, {}, {})",
        _ => "Não sei"
    })
}