struct Produto {
    nome: String,
    preco: f64,
}

impl Produto {
    fn novo(nome: String, preco: f64) -> Produto {
        Produto { nome, preco }
    }

    fn get_preco(&self) -> f64 {
        self.preco
    }
}

struct Carro {
    produto: Produto, // Composição
    marca: String,
}

impl Carro {
    fn novo(nome: String, preco: f64, marca: String) -> Carro {
        Carro {
            produto: Produto::novo(nome, preco),
            marca,
        }
    }

    fn get_preco(&self) -> f64 {
        self.produto.get_preco() // Reutilizando o método de Produto
    }

    fn get_nome(&self) -> &str {
        &self.produto.nome // Reutilizando o campo de Produto
    }
}

fn main() {
    let carro = Carro::novo("Carro 1".to_string(), 50000.0, "Toyota".to_string());
    println!("Carro: {}", carro.get_nome());
    println!("Marca: {}", carro.marca);
    println!("Preço: {}", carro.get_preco());
}
