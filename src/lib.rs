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
        
        let mut matriz_chave = [[0; 4]; 4];

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

// ++ expansão de chave

pub struct ExpansaoDeChave {
    //pub key_schedule: [RoundKey; 11]
}

impl ExpansaoDeChave {
    pub fn expandir(chave: [[u8; 4]; 4]) -> Vec<RoundKey> {

        let mut key_schedule = vec![];

        let primeira = RoundKey::new(chave);
        key_schedule.push(primeira);

        for i in 1..11 {
            let round_key_anterior = &key_schedule[i-1];
            key_schedule.push(RoundKey::from(round_key_anterior, i as u8));
        }
        
        key_schedule
    }
}

static S_BOX: &'static [[&str; 16]; 16] = &[["63", "7c", "77", "7b", "f2", "6b", "6f", "c5", "30", "01", "67", "2b", "fe", "d7", "ab", "76"],
                                            ["ca", "82", "c9", "7d", "fa", "59", "47", "f0", "ad", "d4", "a2", "af", "9c", "a4", "72", "c0"],
                                            ["b7", "fd", "93", "26", "36", "3f", "f7", "cc", "34", "a5", "e5", "f1", "71", "d8", "31", "15"],
                                            ["04", "c7", "23", "c3", "18", "96", "05", "9a", "07", "12", "80", "e2", "eb", "27", "b2", "75"],
                                            ["09", "83", "2c", "1a", "1b", "6e", "5a", "a0", "52", "3b", "d6", "b3", "29", "e3", "2f", "84"],
                                            ["53", "d1", "00", "ed", "20", "fc", "b1", "5b", "6a", "cb", "be", "39", "4a", "4c", "58", "cf"],
                                            ["d0", "ef", "aa", "fb", "43", "4d", "33", "85", "45", "f9", "02", "7f", "50", "3c", "9f", "a8"],
                                            ["51", "a3", "40", "8f", "92", "9d", "38", "f5", "bc", "b6", "da", "21", "10", "ff", "f3", "d2"],
                                            ["cd", "0c", "13", "ec", "5f", "97", "44", "17", "c4", "a7", "7e", "3d", "64", "5d", "19", "73"],
                                            ["60", "81", "4f", "dc", "22", "2a", "90", "88", "46", "ee", "b8", "14", "de", "5e", "0b", "db"],
                                            ["e0", "32", "3a", "0a", "49", "06", "24", "5c", "c2", "d3", "ac", "62", "91", "95", "e4", "79"],
                                            ["e7", "c8", "37", "6d", "8d", "d5", "4e", "a9", "6c", "56", "f4", "ea", "65", "7a", "ae", "08"],
                                            ["ba", "78", "25", "2e", "1c", "a6", "b4", "c6", "e8", "dd", "74", "1f", "4b", "bd", "8b", "8a"],
                                            ["70", "3e", "b5", "66", "48", "03", "f6", "0e", "61", "35", "57", "b9", "86", "c1", "1d", "9e"],
                                            ["e1", "f8", "98", "11", "69", "d9", "8e", "94", "9b", "1e", "87", "e9", "ce", "55", "28", "df"],
                                            ["8c", "a1", "89", "0d", "bf", "e6", "42", "68", "41", "99", "2d", "0f", "b0", "54", "bb", "16"]];

static R_CONSTANTS: [u8; 11] = [0, 1, 2, 4, 8, 16, 32, 64, 128, 27, 54];

pub struct RoundKey {
    chave: [[u8; 4]; 4]
}

impl RoundKey {    
    pub fn new(chave: [[u8; 4]; 4]) -> RoundKey {
        RoundKey {
            chave
        }
    }

    pub fn from(anterior: &RoundKey, i: u8) -> RoundKey {
        let mut primeira_palavra = anterior.get_w(3).unwrap();

        //Rot-word
        primeira_palavra.rotate_left(1);

        //Sub-word
        primeira_palavra = RoundKey::sub_word(primeira_palavra);
        
        //XOR com round constant
        let round_constant = RoundKey::get_round_constant(i as usize).unwrap();
        primeira_palavra = RoundKey::xor_words(primeira_palavra, round_constant);

        //XOR com primeira palavra anterior
        let primeira_palavra_anterior = anterior.get_w(0).unwrap();
        primeira_palavra = RoundKey::xor_words(primeira_palavra_anterior, primeira_palavra);

        let mut chave = [[0; 4]; 4];

        for i in 0..4 {
            chave[i][0] = primeira_palavra[i];    
        }

        for i in 1..4 {
            let palavra_anterior = RoundKey::get_word_from(chave, i-1).unwrap();
            let palavra_equivalente = anterior.get_w(i).unwrap();

            for j in 0..4 {
                chave[j][i] = palavra_anterior[j] ^ palavra_equivalente[j];
            }
        }

        RoundKey::new(chave)
    }

    pub fn get_chave(&self) -> [[u8; 4]; 4] {
        self.chave
    }

    fn sub_word(palavra: [u8; 4]) -> [u8; 4] {
        let mut sub_word = [0; 4];

        for i in 0..4 {
            let hex = format!("{:01$x}", palavra[i], 2);
            let mut chars = hex.chars();

            let linha = usize::from_str_radix(&chars.next().unwrap().to_string(), 16).unwrap();
            let coluna = usize::from_str_radix(&chars.next().unwrap().to_string(), 16).unwrap();

            sub_word[i] = u8::from_str_radix(S_BOX[linha][coluna], 16).unwrap();
        }

        sub_word
    }

    fn get_round_constant(i: usize) -> Result<[u8; 4], &'static str> {
        if i > 10 {
            return Err("Round constant inexistente");
        }

        Ok([R_CONSTANTS[i], 0, 0, 0])
    }

    pub fn get_w(&self, i: usize) -> Result<[u8; 4], &'static str> {
        RoundKey::get_word_from(self.chave, i)
    }

    fn get_word_from(chave: [[u8; 4]; 4], i: usize) -> Result<[u8; 4], &'static str> {
        if i > 3 {
            return Err("Uma round key só possui 4 palavras");
        }

        Ok([chave[0][i],
            chave[1][i],
            chave[2][i],
            chave[3][i]])
    }

    fn xor_words(w1: [u8; 4], w2: [u8; 4]) -> [u8; 4] {
        let mut xor = [0; 4];
        
        for i in 0..4 {
            xor[i] = w1[i] ^ w2[i];
        }

        xor
    }
}

// -- expansão de chave

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