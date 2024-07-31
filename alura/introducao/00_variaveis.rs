fn main(){
    let variavel = 300;
    println!("Valor: {}, Espaço memória: {}",  variavel, std::mem::size_of_val(&variavel));

    let decimal = 2.43256;
    println!("Valor: {}, Espaço memória: {}",  decimal, std::mem::size_of_val(&decimal));

    let booleana = false;
    println!("Valor: {}, Espaço memória: {}",  booleana, std::mem::size_of_val(&booleana));

    let mut variavel_mutavel = 10;
    println!("Valor variavel_mutavel: {}", variavel_mutavel);

    variavel_mutavel = 20;
    println!("Valor variavel_mutavel: {}", variavel_mutavel);

    let caractere:char = 'c';
    println!("Valor: {}, Espaço memória: {}",  caractere, std::mem::size_of_val(&caractere));
}