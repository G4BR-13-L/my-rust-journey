pub(crate) fn conteudo_opcional(){

    let conteudo_arquivo = ler_arquivo(String::from(""));

    match &conteudo_arquivo {
        Some(valor) => println!("Valor da string: {}", valor),
        None => println!("Arquivo não retornou nada")
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