use rocket::{get, post, routes, Route};
use crate::produtos::service::listar_produtos;

#[get("/produtos")]
fn listar() -> String {
    listar_produtos() // Chama o serviço
}

#[post("/produtos")]
fn criar() -> &'static str {
    "Produto criado"
}

// Exporta as rotas deste módulo
pub fn produtos_routes() -> Vec<Route> {
    routes![listar, criar]
}