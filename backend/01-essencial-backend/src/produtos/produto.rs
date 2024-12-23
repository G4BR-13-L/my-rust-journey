use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};


#[derive(Debug)]
pub struct Produto {
    pub uuid: uuid::Uuid,
    pub nome: String,
    pub descricao: String,
    pub valor: i32,
    pub data_cadastro: chrono::NaiveDateTime,
    pub ativo: bool,
    pub disponivel: bool,
}