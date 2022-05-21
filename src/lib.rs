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

pub struct ExpansaoDeChave;

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

static L_TABLE: &'static [[&str; 16]; 16] = &[["00", "00", "19", "01", "32", "02", "1a", "c6", "4b", "c7", "1b", "68", "33", "ee", "df", "03"],
                                              ["64", "04", "e0", "0e", "34", "8d", "81", "ef", "4c", "71", "08", "c8", "f8", "69", "1c", "c1"],
                                              ["7d", "c2", "1d", "b5", "f9", "b9", "27", "6a", "4d", "e4", "a6", "72", "9a", "c9", "09", "78"],
                                              ["65", "2f", "8a", "05", "21", "0f", "e1", "24", "12", "f0", "82", "45", "35", "93", "da", "8e"],
                                              ["96", "8f", "db", "bd", "36", "d0", "ce", "94", "13", "5c", "d2", "f1", "40", "46", "83", "38"],
                                              ["66", "dd", "fd", "30", "bf", "06", "8b", "62", "b3", "25", "e2", "98", "22", "88", "91", "10"],
                                              ["7e", "6e", "48", "c3", "a3", "b6", "1e", "42", "3a", "6b", "28", "54", "fa", "85", "3d", "ba"],
                                              ["2b", "79", "0a", "15", "9b", "9f", "5e", "ca", "4e", "d4", "ac", "e5", "f3", "73", "a7", "57"],
                                              ["af", "58", "a8", "50", "f4", "ea", "d6", "74", "4f", "ae", "e9", "d5", "e7", "e6", "ad", "e8"],
                                              ["2c", "d7", "75", "7a", "eb", "16", "0b", "f5", "59", "cb", "5f", "b0", "9c", "a9", "51", "a0"],
                                              ["7f", "0c", "f6", "6f", "17", "c4", "49", "ec", "d8", "43", "1f", "2d", "a4", "76", "7b", "b7"],
                                              ["cc", "bb", "3e", "5a", "fb", "60", "b1", "86", "3b", "52", "a1", "6c", "aa", "55", "29", "9d"],
                                              ["97", "b2", "87", "90", "61", "be", "dc", "fc", "bc", "95", "cf", "cd", "37", "3f", "5b", "d1"],
                                              ["53", "39", "84", "3c", "41", "a2", "6d", "47", "14", "2a", "9e", "5d", "56", "f2", "d3", "ab"],
                                              ["44", "11", "92", "d9", "23", "20", "2e", "89", "b4", "7c", "b8", "26", "77", "99", "e3", "a5"],
                                              ["67", "4a", "ed", "de", "c5", "31", "fe", "18", "0d", "63", "8c", "80", "c0", "f7", "70", "07"]];

static E_TABLE: &'static [[&str; 16]; 16] = &[["01", "03", "05", "0f", "11", "33", "55", "ff", "1a", "2e", "72", "96", "a1", "f8", "13", "35"],
                                              ["5f", "e1", "38", "48", "d8", "73", "95", "a4", "f7", "02", "06", "0a", "1e", "22", "66", "aa"],
                                              ["e5", "34", "5c", "e4", "37", "59", "eb", "26", "6a", "be", "d9", "70", "90", "ab", "e6", "31"],
                                              ["53", "f5", "04", "0c", "14", "3c", "44", "cc", "4f", "d1", "68", "b8", "d3", "6e", "b2", "cd"],
                                              ["4c", "d4", "67", "a9", "e0", "3b", "4d", "d7", "62", "a6", "f1", "08", "18", "28", "78", "88"],
                                              ["83", "9e", "b9", "d0", "6b", "bd", "dc", "7f", "81", "98", "b3", "ce", "49", "db", "76", "9a"],
                                              ["b5", "c4", "57", "f9", "10", "30", "50", "f0", "0b", "1d", "27", "69", "bb", "d6", "61", "a3"],
                                              ["fe", "19", "2b", "7d", "87", "92", "ad", "ec", "2f", "71", "93", "ae", "e9", "20", "60", "a0"],
                                              ["fb", "16", "3a", "4e", "d2", "6d", "b7", "c2", "5d", "e7", "32", "56", "fa", "15", "3f", "41"],
                                              ["c3", "5e", "e2", "3d", "47", "c9", "40", "c0", "5b", "ed", "2c", "74", "9c", "bf", "da", "75"],
                                              ["9f", "ba", "d5", "64", "ac", "ef", "2a", "7e", "82", "9d", "bc", "df", "7a", "8e", "89", "80"],
                                              ["9b", "b6", "c1", "58", "e8", "23", "65", "af", "ea", "25", "6f", "b1", "c8", "43", "c5", "54"],
                                              ["fc", "1f", "21", "63", "a5", "f4", "07", "09", "1b", "2d", "77", "99", "b0", "cb", "46", "ca"],
                                              ["45", "cf", "4a", "de", "79", "8b", "86", "91", "a8", "e3", "3e", "42", "c6", "51", "f3", "0e"],
                                              ["12", "36", "5a", "ee", "29", "7b", "8d", "8c", "8f", "8a", "85", "94", "a7", "f2", "0d", "17"],
                                              ["39", "4b", "dd", "7c", "84", "97", "a2", "fd", "1c", "24", "6c", "b4", "c7", "52", "f6", "01"]];

pub struct RoundKey {
    chave: [[u8; 4]; 4]
}

fn get_linha_coluna(byte: u8) -> (usize, usize) {
    let hex = format!("{:01$x}", byte, 2);
    let mut chars = hex.chars();

    let linha = usize::from_str_radix(&chars.next().unwrap().to_string(), 16).unwrap();
    let coluna = usize::from_str_radix(&chars.next().unwrap().to_string(), 16).unwrap();

    (linha, coluna)
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

    fn sub_word(word: [u8; 4]) -> [u8; 4] {
        let mut sub_word = [0; 4];

        for i in 0..4 {
            let hex = format!("{:01$x}", word[i], 2);
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

pub struct Aes;

impl Aes {

    pub fn encrypt(texto: &str, key_schedule: Vec<RoundKey>) -> Vec<[[u8; 4]; 4]> {
        let matriz_estado = Aes::get_matriz_estado(texto);
        let mut blocos = vec![];

        for mut bloco in matriz_estado {
            let round_key = &key_schedule[0];
            bloco = Aes::add_round_key(bloco, round_key);

            for i in 1..10 {
                let round_key_atual = &key_schedule[i];

                let sub_bytes = Aes::sub_bytes(bloco);
                let shift_rows = Aes::shift_rows(sub_bytes);
                let mix_columns = Aes::mix_columns(shift_rows);
                
                bloco = Aes::add_round_key(mix_columns, round_key_atual);
            }

            let ultima_round_key = &key_schedule[10];
            let sub_bytes = Aes::sub_bytes(bloco);
            let shift_rows = Aes::shift_rows(sub_bytes);
            
            bloco = Aes::add_round_key(shift_rows, ultima_round_key);

            blocos.push(bloco);
        }

        blocos
    }

    fn get_matriz_estado(texto: &str) -> Vec<[[u8; 4]; 4]> {
        let bytes = texto.as_bytes().to_vec();
        
        Aes::to_pkcs5_blocks(bytes)
    }

    fn to_pkcs5_blocks(bytes: Vec<u8>) -> Vec<[[u8; 4]; 4]> {
        let mut blocos = vec![];
        let qtd_ate_16 = 16 - bytes.len() % 16;
        let bytes_totais = bytes.len() + qtd_ate_16;
        let qtd_blocos = bytes_totais/16;

        for i in 0..qtd_blocos {
            let mut matriz_estado = [[0; 4]; 4];

            for j in 0..4 {
                for k in 0..4 {
                    let byte_atual = i*16 + j*4 + k;

                    if byte_atual >= bytes.len() {
                        matriz_estado[k][j] = qtd_ate_16 as u8;    
                    }
                    else {
                        matriz_estado[k][j] = bytes[byte_atual];
                    }
                }
            }

            blocos.push(matriz_estado);
        }

        blocos
    }

    fn xor(bloco_x: [[u8; 4]; 4], bloco_y: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
        let mut xor = [[0; 4]; 4]; 

        for i in 0..4 {
            for j in 0..4 {
                xor[i][j] = bloco_x[i][j] ^ bloco_y[i][j];
            }
        }

        xor
    }

    fn sub_bytes(bloco: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
        let mut sub_bytes = [[0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                let hex = format!("{:01$x}", bloco[i][j], 2);
                let mut chars = hex.chars();

                let linha = usize::from_str_radix(&chars.next().unwrap().to_string(), 16).unwrap();
                let coluna = usize::from_str_radix(&chars.next().unwrap().to_string(), 16).unwrap();

                sub_bytes[i][j] = u8::from_str_radix(S_BOX[linha][coluna], 16).unwrap();
            }
        }

        sub_bytes
    }

    fn shift_rows(mut bloco: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
        for i in 1..4 {
            bloco[i].rotate_left(i);
        }
        
        bloco
    }

    fn mix_columns(bloco: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
        let mut matriz_estado = [[0; 4]; 4];
        let matriz_multiplicacao = [[2, 3, 1, 1],
                                    [1, 2, 3, 1],
                                    [1, 1, 2, 3],
                                    [3, 1, 1, 2]];

        for i in 0..4 {
            for j in 0..4 {
                let mut resultado_galois = 0;

                for k in 0..4 {
                    resultado_galois ^= Aes::multiplicar_galois(bloco[k][i], matriz_multiplicacao[j][k]);
                }

                matriz_estado[j][i] = resultado_galois;
            }
        }

        matriz_estado        
    }

    fn multiplicar_galois(x: u8, y: u8) -> u8 {
        if x == 0 || y == 0 {
            return 0;
        }

        if x == 1 {
            return y;
        }

        if y == 1 {
            return x;
        }

        let (linha_x, coluna_x) = get_linha_coluna(x);
        let (linha_y, coluna_y) = get_linha_coluna(y);

        let valor_x = u8::from_str_radix(L_TABLE[linha_x][coluna_x], 16).unwrap();
        let valor_y = u8::from_str_radix(L_TABLE[linha_y][coluna_y], 16).unwrap();

        let teste_soma = valor_x as u16 + valor_y as u16;

        let soma = if teste_soma > 255 {
            (teste_soma - 255) as u8
        } else {
            teste_soma as u8
        };

        let (linha_e, coluna_e) = get_linha_coluna(soma);

        u8::from_str_radix(E_TABLE[linha_e][coluna_e], 16).unwrap()
    }

    fn add_round_key(bloco: [[u8; 4]; 4], round_key: &RoundKey) -> [[u8; 4]; 4] {
        Aes::xor(bloco, round_key.get_chave())
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

    #[test]
    fn round_key_new_passando_chave() {
        let chave = [[1, 5, 9, 13],
                     [2, 6, 10, 14],
                     [3, 7, 11, 15],
                     [4, 8, 12, 16]];
        
        let round_key = RoundKey::new(chave); 

        assert_eq!(chave, round_key.get_chave());
    }

    #[test]
    fn round_key_from_anterior() {
        let chave = [[65, 69, 73, 77],
                     [66, 70, 74, 78],
                     [67, 71, 75, 79],
                     [68, 72, 76, 80]];
        
        let anterior = RoundKey::new(chave);
        let nova = RoundKey::from(&anterior, 1);

        let nova_chave = [[111, 42, 99, 46],
                          [198, 128, 202, 132],
                          [16, 87, 28, 83],
                          [167, 239, 163, 243]];

        assert_eq!(nova_chave, nova.get_chave());
    }

    #[test]
    fn sub_word_inicio_sbox() {
        let word = [0, 1, 2, 3];

        let sub = RoundKey::sub_word(word);

        assert_eq!([99, 124, 119, 123], sub);
    }

    #[test]
    fn get_round_constant_todas_round_keys() {
        assert_eq!(Ok([0, 0, 0, 0]), RoundKey::get_round_constant(0));
        assert_eq!(Ok([1, 0, 0, 0]), RoundKey::get_round_constant(1));
        assert_eq!(Ok([2, 0, 0, 0]), RoundKey::get_round_constant(2));
        assert_eq!(Ok([4, 0, 0, 0]), RoundKey::get_round_constant(3));
        assert_eq!(Ok([8, 0, 0, 0]), RoundKey::get_round_constant(4));
        assert_eq!(Ok([16, 0, 0, 0]), RoundKey::get_round_constant(5));
        assert_eq!(Ok([32, 0, 0, 0]), RoundKey::get_round_constant(6));
        assert_eq!(Ok([64, 0, 0, 0]), RoundKey::get_round_constant(7));
        assert_eq!(Ok([128, 0, 0, 0]), RoundKey::get_round_constant(8));
        assert_eq!(Ok([27, 0, 0, 0]), RoundKey::get_round_constant(9));
        assert_eq!(Ok([54, 0, 0, 0]), RoundKey::get_round_constant(10));
    }

    #[test]
    #[should_panic(expected = "Round constant inexistente")]
    fn get_round_constant_round_key_invalida() {
        RoundKey::get_round_constant(11).unwrap();
    }

    #[test]
    fn get_w_todas_words() {
        let chave = [[65, 69, 73, 77],
                     [66, 70, 74, 78],
                     [67, 71, 75, 79],
                     [68, 72, 76, 80]];

        let round_key = RoundKey::new(chave);

        assert_eq!(Ok([65, 66, 67, 68]), round_key.get_w(0));
        assert_eq!(Ok([69, 70, 71, 72]), round_key.get_w(1));
        assert_eq!(Ok([73, 74, 75, 76]), round_key.get_w(2));
        assert_eq!(Ok([77, 78, 79, 80]), round_key.get_w(3));
    }

    #[test]
    #[should_panic(expected = "Uma round key só possui 4 palavras")]
    fn get_w_indice_invalido() {
        let chave = [[65, 69, 73, 77],
                     [66, 70, 74, 78],
                     [67, 71, 75, 79],
                     [68, 72, 76, 80]];

        let round_key = RoundKey::new(chave);

        round_key.get_w(4).unwrap();
    }

    #[test]
    fn get_word_from_todas_words() {
        let chave = [[65, 69, 73, 77],
                     [66, 70, 74, 78],
                     [67, 71, 75, 79],
                     [68, 72, 76, 80]];

        assert_eq!(Ok([65, 66, 67, 68]), RoundKey::get_word_from(chave, 0));
        assert_eq!(Ok([69, 70, 71, 72]), RoundKey::get_word_from(chave, 1));
        assert_eq!(Ok([73, 74, 75, 76]), RoundKey::get_word_from(chave, 2));
        assert_eq!(Ok([77, 78, 79, 80]), RoundKey::get_word_from(chave, 3));
    }

    #[test]
    #[should_panic(expected = "Uma round key só possui 4 palavras")]
    fn get_word_from_indice_invalido() {
        let chave = [[65, 69, 73, 77],
                     [66, 70, 74, 78],
                     [67, 71, 75, 79],
                     [68, 72, 76, 80]];

        RoundKey::get_word_from(chave, 4).unwrap();
    }

    #[test]
    fn expansao_de_chave_expandir_validando_ultima_round_key() {
        let chave = [[65, 69, 73, 77],
                     [66, 70, 74, 78],
                     [67, 71, 75, 79],
                     [68, 72, 76, 80]];

        let ultima = [[136, 191, 187, 157],
                      [46, 107, 62, 233],
                      [17, 151, 61, 183],
                      [219, 8, 218, 239]];

        let key_schedule = ExpansaoDeChave::expandir(chave);

        assert_eq!(chave, key_schedule[0].get_chave());
        assert_eq!(ultima, key_schedule[10].get_chave());
    }
}