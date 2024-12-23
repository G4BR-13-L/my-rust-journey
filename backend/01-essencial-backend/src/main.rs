#[macro_use] extern crate rocket;

use rocket::{get, routes, Rocket};
use std::fs;
use std::path::Path;
use essencial_backend::produtos::controller::produtos_routes;
use essencial_backend::configuration::migrations::{run_migrations, create_migration_table, check_table_exists, connect_to_db};

#[launch]
async fn rocket() -> _ {
    let client = connect_to_db().await.expect("Erro ao conectar ao banco de dados");

    if !check_table_exists(&client).await.expect("Erro ao verificar tabela") {
        create_migration_table(&client)
            .await
            .expect("Erro ao criar tabela de migração");
    }

    run_migrations(&client)
        .await
        .expect("Erro ao executar migrações");

    rocket::build().mount("/", produtos_routes())
}
