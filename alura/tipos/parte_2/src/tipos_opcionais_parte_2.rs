pub(crate) fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));

    // Let é como se fosse um match com uma unica opção
    if let Some(valor) = conteudo_arquivo {
        println!("Conteudo 2 por if let: {}", valor);
    }
}

// Option é como se fosse uma Enum
// enum Option{
//     Some(String),
//     None
// }

fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    // retorno 1
    Some(String::from("conteudo"))

    // retorno 2
    // None
}
