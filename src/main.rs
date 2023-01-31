// colocando dados de entrada pelo usuario

use std::io;

// fn = function

fn convert_to_int(data_input: & String) -> i32{ //& String (valor precisa ser uma string); -> i32 (O Resultado será um inteiro 32bits)
    //trim = recorta a string; parse = converte para i32; unwrap = se tiver um none, vai ocorrer um erro
    let x = data_input.trim().parse::<i32>().unwrap();
    x    
}


fn main() {

    let mut number1 = String::new(); // criando uma forma do usuario inserir um valor, no caso, uma string
    io::stdin().read_line(&mut number1).expect("Erro ao ler o number1"); //stdin = standard input

    let mut number2 = String::new(); // criando uma forma do usuario inserir um valor, no caso, uma string
    io::stdin().read_line(&mut number2).expect("Erro ao ler o number2"); //stdin = standard input

    if convert_to_int(&number1) > convert_to_int(&number2){ // precisa colocar o & para haver a mudança do tipo
        println!("O numero {} eh maior que {}", number1, number2);
    } 
    else{
        println!("O numero {} eh menor ou igual que {}", number1, number2);
    }

}