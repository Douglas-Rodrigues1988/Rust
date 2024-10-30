const TIPO_DE_DADOS: i8 = 2;
static mut UMA_VARIAVEL_STATICA: i8 = 3;


fn main() {
    unsafe {
        UMA_VARIAVEL_STATICA = 4;
        println!("Constante: {}", TIPO_DE_DADOS);
        println!("Variável estática: {}", UMA_VARIAVEL_STATICA);
    }
    imprime();
}

fn imprime() {
    unsafe {
        UMA_VARIAVEL_STATICA = 4;
        println!("Constante: {}", TIPO_DE_DADOS);
        println!("Variável estática: {}", UMA_VARIAVEL_STATICA);
    }
    imprime();
}
