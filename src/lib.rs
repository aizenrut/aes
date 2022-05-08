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

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn config_new_com_argumentos_corretos() {
        let arquivo_origem = String::from("teste/arquivo/origem");
        let arquivo_destino = String::from("teste/arquivo/destino");
        let chave = String::from("1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16");
        let matriz_chave = [[1, 5, 9, 13],
                            [2, 6, 10, 14],
                            [3, 7, 11, 15],
                            [4, 8, 12, 16]];

        let args = vec![String::new(), arquivo_origem.clone(), arquivo_destino.clone(), chave.clone()];
        let config = Config::new(args).unwrap();

        assert_eq!(arquivo_origem, config.arquivo_origem);
        assert_eq!(arquivo_destino, config.arquivo_destino);
        assert_eq!(matriz_chave, config.matriz_chave);
    }

    #[test]
    #[should_panic(expected = "Quantidade de argumentos insuficiente")]
    fn config_new_com_argumentos_insuficientes() {
        let arquivo_origem = String::from("teste/arquivo/origem");
        let arquivo_destino = String::from("teste/arquivo/destino");

        let args = vec![String::new(), arquivo_origem.clone(), arquivo_destino.clone()];
        Config::new(args).unwrap();
    }

    #[test]
    #[should_panic(expected = "A chave possui bytes inválidos")]
    fn config_new_com_bytes_invalidos() {
        let arquivo_origem = String::from("teste/arquivo/origem");
        let arquivo_destino = String::from("teste/arquivo/destino");
        let chave = String::from("1/,2,3,4,5,6,7,8,9,10,11.12,13,f,15,a");

        let args = vec![String::new(), arquivo_origem.clone(), arquivo_destino.clone(), chave.clone()];
        Config::new(args).unwrap();
    }

    #[test]
    #[should_panic(expected = "A chave deve ter exatamente 16 bytes")]
    fn config_new_com_chave_insuficiente() {
        let arquivo_origem = String::from("teste/arquivo/origem");
        let arquivo_destino = String::from("teste/arquivo/destino");
        let chave = String::from("1,2,3,4,5,6,7,8,9,10,11,12,13,14,15");

        let args = vec![String::new(), arquivo_origem.clone(), arquivo_destino.clone(), chave.clone()];
        Config::new(args).unwrap();
    }
}