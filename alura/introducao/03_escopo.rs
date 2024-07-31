



fn sombra(){

    let a = 10; // variavel assombrada

    { // Escopo freestyle

        let b = 10;
        println!("b = {}", b);


        let a = 20; // Variavel A que asombra a variavel A declarada anteriormente
        //uma faz sombvra sobre a outra
        println!("a = {}", a);
    }

    println!("a = {}", a);

}
fn escopo(){
    println!("Escopo")
}

fn main(){
    escopo();
    sombra();
}