#[cfg(test)]
mod produtos_test {
    use crate::produtos::persistence; // Ajuste o caminho conforme necessário.

    #[test]
    fn test_buscar_todos_retorna_produtos() {
        let produtos = persistence::buscar_todos();
        assert_eq!(produtos.len(), 2);
        assert_eq!(produtos[0], "Produto A");
        assert_eq!(produtos[1], "Produto B");
    }

    #[test]
    fn test_buscar_todos_nao_retorna_lista_vazia() {
        let produtos = persistence::buscar_todos();
        assert!(!produtos.is_empty());
    }
}
