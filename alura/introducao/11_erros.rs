fn main(){

    match resultado() {
        Ok(s) => println!("Resultado sucesso: {}", s),
        Err(numero) => println!("Codigo de erro = {}", numero)
    };
}

fn resultado() -> Result<String, u8>{
    Ok(String::from("Tudo deu certo"))
}