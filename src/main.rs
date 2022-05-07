use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(args).unwrap_or_else(|erro| {
        eprintln!("Erro ao processar os argumentos: {}", erro);
        process::exit(1);
    });

    println!("config.arquivo_origem: {}", config.arquivo_origem);
    println!("config.arquivo_destino: {}", config.arquivo_destino);
    println!("config.matriz_chave: {:?}", config.matriz_chave);
}


pub struct Config {
    pub arquivo_origem: String,
    pub arquivo_destino: String,
    pub matriz_chave: [[u8; 4]; 4]
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Quantidade de argumentos insuficiente")
        }

        let arquivo_origem = args[1].clone();
        let arquivo_destino = args[2].clone();
        let str_chave = args[3].clone();

        let str_chave: Vec<&str> = str_chave.split(",")
            .map(|s| s.trim())
            .collect();

        let mut bytes_chave = vec![];

        for s in str_chave {
            let byte = match s.parse() {
                Ok(b) => b,
                Err(_) => return Err("A chave possui bytes inválidos")
            };

            bytes_chave.push(byte)
        }

        if bytes_chave.len() != 16 {
            return Err("A chave deve ter exatamente 16 bytes");
        }
        
        let mut matriz_chave: [[u8; 4]; 4] = [[0; 4]; 4];

        for i in 0..4 {
            matriz_chave[0][i] = bytes_chave[i*4];
            matriz_chave[1][i] = bytes_chave[i*4+1];
            matriz_chave[2][i] = bytes_chave[i*4+2];
            matriz_chave[3][i] = bytes_chave[i*4+3];
        }

        Ok(Config {
            arquivo_origem,
            arquivo_destino,
            matriz_chave
        })
    }
}