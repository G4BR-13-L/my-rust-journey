fn main(){
    ownership();
}

fn ownership(){
    // Uma "String" com s maiusculo é como um objeto, capaz de apontar para um espaço de mémória HEAP que contem um Str estátic
    let uma_string = String::from("Gabriel");

    // Strings "dinamicas" como String::from("Gabriel") são armazenadas no heap por não terem um tamanho fixo. podendo varia em consumo de memória
    // Portanto essas Strings dinamicas são armazendas no Heap para não causa StackOverflow

    // Utilizando de referencia
    // Referencias & por padrão são imutáveis
    rouba(&uma_string); // Emprestando valor para a função

    println!("{}", uma_string);
}

fn rouba(string: &String){
    println!("{}",string);
}