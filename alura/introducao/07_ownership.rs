fn main(){
    
    // Strings são tipos complexos em rust. Não são simplesmente arrays de chars

    // Uma simples e inocente declaração de string
    let _minha_string = "Olá mundo";
    //na verdade pe uma referencia (&) a um 'static str'
    // str é um pedaço de uma string
    ownership();
}

fn ownership(){
    // Uma "String" com s maiusculo é como um objeto, capaz de apontar para um espaço de mémória HEAP que contem um Str estátic
    let uma_string = String::from("Gabriel");

    // Strings "dinamicas" como String::from("Gabriel") são armazenadas no heap por não terem um tamanho fixo. podendo varia em consumo de memória
    // Portanto essas Strings dinamicas são armazendas no Heap para não causa StackOverflow

    let outra_string = rouba(uma_string);
    println!("{}", outra_string);
}

fn rouba(string: String) -> String{
    println!("{}",string);
    string // retronando a string de volta
}