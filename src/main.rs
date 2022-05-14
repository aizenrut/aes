use std::{env, process};
use aes::{Config, ExpansaoDeChave};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(args).unwrap_or_else(|erro| {
        eprintln!("Erro ao processar os argumentos: {}", erro);
        process::exit(1);
    });

    let key_schedule = ExpansaoDeChave::expandir(config.matriz_chave);

    let mut counter = 0;

    for k in key_schedule {
        println!("{})", counter);
        for l in k.get_chave() {
            println!("{:?}", l);
        }
        counter += 1;
        println!();
    }
}