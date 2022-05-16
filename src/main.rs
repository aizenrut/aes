use std::{env, process};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use aes::{Aes, Config, ExpansaoDeChave};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(args).unwrap_or_else(|erro| {
        eprintln!("Erro ao processar os argumentos: {}", erro);
        process::exit(1);
    });

    let key_schedule = ExpansaoDeChave::expandir(config.matriz_chave);

    let bloco = Aes::encrypt(String::from("DESENVOLVIMENTO!").as_str(), key_schedule);

    for l in bloco {
        println!("{:?}", l);
    }
}

fn get_bytes(arquivo: &str) -> Result<Vec<u8>, &'static str> {
    let f = match File::open(arquivo) {
        Ok(f) => f,
        Err(_) => return Err("Arquivo de entrada inválido")
    };

    let mut reader = BufReader::new(f);
    let mut buffer = vec![];

    match reader.read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err("Não foi possível ler o arquivo de entrada")
    };

    Ok(buffer)
}

fn write_bytes(arquivo: &str, mut bytes: &[u8]) -> Result<(), &'static str> {
    let mut f = match File::create(arquivo){
        Ok(f) => f,
        Err(_) => return Err("Não foi possível criar o arquivo de saída")
    };
    
    match f.write_all(bytes) {
        Ok(_) => (),
        Err(_) => return Err("Não foi possível escrever no arquivo de saída")
    };

    Ok(())
}