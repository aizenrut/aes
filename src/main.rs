use std::{env, process};
use aes::{Config, ExpansaoDeChave};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(args).unwrap_or_else(|erro| {
        eprintln!("Erro ao processar os argumentos: {}", erro);
        process::exit(1);
    });

    let key_schedule = ExpansaoDeChave::expandir(config.matriz_chave);
}