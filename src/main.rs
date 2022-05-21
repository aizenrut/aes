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

    let bytes = get_bytes(&config.arquivo_origem).unwrap_or_else(|erro| {
        eprintln!("Erro ao ler o arquivo de origem: {}", erro);
        process::exit(1);
    });

    let key_schedule = ExpansaoDeChave::expandir(config.matriz_chave);

    let bloco = Aes::encrypt(bytes, key_schedule);

    let resultado = write_bytes(&config.arquivo_destino, &bloco);

    if let Err(erro) = resultado {
        eprintln!("Erro ao gravar o resultado: {}", erro);
        process::exit(1);
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

fn write_bytes(arquivo: &str, blocos: &Vec<[[u8; 4]; 4]>) -> Result<(), &'static str> {
    let mut f = match File::create(arquivo){
        Ok(f) => f,
        Err(_) => return Err("Não foi possível criar o arquivo de saída")
    };
    
    let mut bytes = vec![];

    for bloco in blocos {
        for i in 0..4 {
            for j in 0..4 {
                bytes.push(bloco[j][i]);
            }
        }
    }

    match f.write_all(&bytes) {
        Ok(_) => (),
        Err(_) => return Err("Não foi possível escrever no arquivo de saída")
    };

    Ok(())
}