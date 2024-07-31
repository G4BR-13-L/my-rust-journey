fn main(){
    ownership();
}

fn ownership(){
    let mut uma_string = String::from("Gabriel");
    rouba(&mut uma_string); // Emprestando valor para a função
    println!("{}", uma_string);
}

fn rouba(string: &mut String){
    string.push_str(" Victor");
    println!("{}",string);
}