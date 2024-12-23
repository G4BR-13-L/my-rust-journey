use rocket::{get, launch, routes};
use std::fs;
use tokio_postgres::{Client, NoTls};
use crate::utils::sha3::sha3_256_of_file;

// Conecta ao banco de dados e retorna um cliente
pub async fn connect_to_db() -> Result<Client, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::Config::new()
        .host("localhost")
        .port(5453)
        .user("postgres")
        .password("postgres")
        .dbname("postgres")
        .connect(NoTls)
        .await?;

    // Inicia o gerenciador de conexão em uma nova tarefa
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Erro na conexão: {}", e);
        }
    });

    Ok(client)
}

// Verifica se a tabela t_migration existe
pub  async fn check_table_exists(client: &Client) -> Result<bool, tokio_postgres::Error> {
    let rows = client
        .query(
            "SELECT EXISTS (
                SELECT 1 FROM information_schema.tables 
                WHERE table_schema = 'public' AND table_name = 't_migration'
            )",
            &[],
        )
        .await?;

    Ok(rows[0].get(0))
}

// Cria a tabela t_migration, se necessário
pub  async fn create_migration_table(client: &Client) -> Result<(), tokio_postgres::Error> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS t_migration (
                id SERIAL PRIMARY KEY,
                nome_arquivo TEXT NOT NULL UNIQUE,
                checksum_sha3 TEXT NOT NULL,
                executado_em TIMESTAMP DEFAULT now()
            )",
            &[],
        )
        .await?;

    Ok(())
}

// Executa os scripts SQL na pasta migrations
pub  async fn run_migrations(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let migration_files = fs::read_dir("./migrations")?;

    for entry in migration_files {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "sql") {
            let file_name = path
                .file_name()
                .ok_or("Nome de arquivo inválido")?
                .to_string_lossy()
                .to_string();

            let file_sha3 = sha3_256_of_file(path.to_str().unwrap())?;

            let rows = client
                .query(
                    "SELECT checksum_sha3 FROM t_migration WHERE nome_arquivo = $1",
                    &[&file_name],
                )
                .await?;

            if let Some(row) = rows.get(0) {
                let db_sha3: String = row.get(0);
                if db_sha3 != file_sha3 {
                    panic!(
                        "O checksum SHA3 do arquivo {} não corresponde ao registrado na tabela.",
                        file_name
                    );
                }
                println!("Já executado e verificado: {}", file_name);
                continue;
            }

            let sql_content = fs::read_to_string(&path)?;
            client.batch_execute(&sql_content).await?;

            client
                .execute(
                    "INSERT INTO t_migration (nome_arquivo, checksum_sha3) VALUES ($1, $2)",
                    &[&file_name, &file_sha3],
                )
                .await?;

            println!("Executado: {}", file_name);
        }
    }

    Ok(())
}
