use uuid::Uuid;

struct Titular {
    nome: String,
    cpf: String,
    uuid: String,
}

struct Conta {
    saldo: f64,
    titular: Titular,
    uuid: String,
}

struct Banco {
    pub nome: String,
    uuid: String,
    contas: Vec<Conta>,
}

impl Banco {
    fn novo(nome: String) -> Banco {
        Banco {
            nome: nome,
            uuid: generate_uuid(),
            contas: Vec::with_capacity(5),
        }
    }

    fn get_contas(&self) -> &Vec<Conta> {
        return &self.contas;
    }

    fn get_nome(&self) -> String {
        return self.nome.clone();
    }

    fn add_conta(&mut self, conta: Conta) {
        self.contas.push(conta);
    }
}

impl Conta {
    fn new(titular: Titular) -> Conta {
        Conta {
            uuid: generate_uuid(),
            titular: titular,
            saldo: 0.0,
        }
    }
}

impl Titular {
    fn new(nome: String, cpf: String) -> Titular {
        Titular {
            nome: nome,
            cpf: cpf,
            uuid: generate_uuid(),
        }
    }
}

trait BaseEntity {
    fn get_uuid(&self) -> String;
    fn log_to_string(&self);
}

impl BaseEntity for Banco {
    fn get_uuid(&self) -> String {
        return self.uuid.clone();
    }
    fn log_to_string(&self) {
        println!("----------------------");
        println!("uuid: {}", self.get_uuid());
        println!("nome: {}", self.nome);
        println!("quantidade_contas: {}", self.contas.len());
        println!("----------------------");
    }
}

impl BaseEntity for Conta {
    fn get_uuid(&self) -> String {
        return self.uuid.clone();
    }
    fn log_to_string(&self) {
        println!("----------------------");
        println!("uuid: {}", self.get_uuid());
        println!("saldo: {}", self.saldo);
        println!("----------------------");
    }
}

impl BaseEntity for Titular {
    fn get_uuid(&self) -> String {
        return self.uuid.clone();
    }
    fn log_to_string(&self) {
        println!("----------------------");
        println!("uuid: {}", self.get_uuid());
        println!("nome: {}", self.nome);
        println!("cpf: {}", self.cpf);
        println!("----------------------");
    }
}

fn generate_uuid() -> String {
    return Uuid::new_v4().to_string();
}

fn main() {
    let mut banco = Banco::novo("Itaú".to_string());
    let titular = Titular::new("Gabriel".to_string(), "12345678910".to_string());
    let conta = Conta::new(titular);
    banco.add_conta(conta);

    banco.log_to_string();
    for conta in banco.get_contas() {
        println!("Conta: {} \n", conta.uuid,);
    }

    for conta in banco.get_contas() {
        conta.log_to_string();
    }
}
