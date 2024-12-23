-- Migration: Adicionar tabela t_produto
CREATE TABLE t_produto (
    uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nome VARCHAR NOT NULL,
    descricao VARCHAR NOT NULL,
    valor INT NOT NULL,
    data_cadastro TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ativo BOOLEAN DEFAULT TRUE,
    disponivel BOOLEAN DEFAULT TRUE
);
