fn main() {
    const PI: f32 = 3.14;
    println!("PI = {}", PI);

    unsafe {
        static mut ESTATICO: u8 = 10;
        println!("static = {}", ESTATICO);

        ESTATICO = 20;
        println!("static = {}", ESTATICO);
    }
}
