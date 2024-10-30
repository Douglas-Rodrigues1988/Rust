fn main() {
    /*
    Dado que eu tenha um ano de nascimento, e faço a subtração pelo ano atual,
    então devi ter o valor da idade da pessoa.
     */
    let nome: &str = "Douglas";

    let ano_nascimento: u16 = 1988;
    let mes_nascimento: u16 = 6;
    let dia_nascimento: u16 = 23;

    let ano_atual: u16 = 2024;
    let mes_atual : u16 = 10;
    let dia_atual : u16 = 30;

    let mut idade: u16 = ano_atual - ano_nascimento;
    if mes_nascimento > mes_atual {
        idade -= 1;
    }
    else if dia_nascimento > dia_atual {  
            idade -= 1;
        
    }

    println!("A idade do {} calculada para o ano de {} é de {} anos", nome, ano_nascimento, idade);
}
