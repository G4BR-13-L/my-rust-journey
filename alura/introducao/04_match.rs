


fn main(){
    let linguagem = "fdasf";
    let proposito = match linguagem {
        "Java" => "Backend",
        "Kotlin" => "Android",
        "Python" => "Dados",
        "Scratch" => "Didatica",
        _ => "desconhecido" 
    };

    println!("Proposito = {}", proposito);

}