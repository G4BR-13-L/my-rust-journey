

// Não é possivel omitir o retorno de uma função;
// Sempre explicitar o retorno

fn main(){
    println!("Soma = {}", soma(2,2))
}


fn soma(a:i32, b:i32) -> i32{
    println!("{} + {} = {}", a, b, a+b);
    a + b // Omissão de ponto e virgula significa declaração de um retorno;
}