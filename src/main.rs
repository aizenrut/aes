use std::{env, process};
use aes::{Aes, Config, ExpansaoDeChave};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(args).unwrap_or_else(|erro| {
        eprintln!("Erro ao processar os argumentos: {}", erro);
        process::exit(1);
    });

    let _key_schedule = ExpansaoDeChave::expandir(config.matriz_chave);

    let blocos = Aes::get_state_matrix(String::from("DESENVOLVIMENTO!").as_str());

    for b in blocos {
        for l in b {
            println!("{:?}", l);
        }
        println!();
    }
}