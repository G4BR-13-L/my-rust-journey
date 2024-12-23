use crate::produtos::persistence::buscar_todos;

pub fn listar_produtos() -> String {
    let produtos = buscar_todos();
    produtos.join(", ") // Retorna a lista de produtos como uma string
}
