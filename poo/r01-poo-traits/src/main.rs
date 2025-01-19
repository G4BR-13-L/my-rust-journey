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

trait Item {
    fn get_preco(&self) -> f64;
    fn get_nome(&self) -> &str;
}

impl Item for Produto {
    fn get_preco(&self) -> f64 {
        self.preco
    }

    fn get_nome(&self) -> &str {
        &self.nome
    }
}

impl Item for Carro {
    fn get_preco(&self) -> f64 {
        self.produto.get_preco()
    }

    fn get_nome(&self) -> &str {
        &self.produto.nome
    }
}

fn main() {
    let produto = Produto::novo("Produto 1".to_string(), 10.0);
    let carro = Carro::novo("Carro 1".to_string(), 50000.0, "Toyota".to_string());

    let items: Vec<&dyn Item> = vec![&produto, &carro]; // Vetor de itens usando polimorfismo

    for item in items {
        println!("Item: {}", item.get_nome());
        println!("Preço: {}", item.get_preco());
    }
}
