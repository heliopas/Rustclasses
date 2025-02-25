//cargo new, cria novo projeto
//cargo init, cria novo projeto em uma pasta existente
//cargo help, help do cargo

use core::time;
use std::thread::sleep;

// declaração de constante não é variavel
const PI:f32 = 3.1415;
// declaração de uma variavel global imutavel
static GLOBAL_VAR:u8 = 1;
// declaração de uma variavel global mutavel
//static mut GLOBAL_VAR_MUT:u8 = 1; não existe mais

// '->' especifica o retorno da função e o tipo
fn soma(a:i32, b:i32) -> i32{
    
println!("{} + {} = {}", a, b, a + b); // quando coloca ';' não armazena o resutado
return a + b // sem ';' retorna valor, não precisa de return para retornar valores da função

}
fn vetores() {

    let vec_notas = [1f32, 10.50, 8f32, 8f32, 10f32]; // a notação xxf32 realiza um typecast
    let vec_notas2 = [6.5; 50]; // cria um vetor de 50 elementos com o valor '6.5'
    
    
    for aux in 1..vec_notas.len() {
    
        println!("numero do vetor: {}", vec_notas[aux]);
            
    }
    
    for aux in 1..vec_notas2.len() {
    
        println!("numero do vetor: {}", vec_notas2[aux]);
            
    }

}



fn var() {

    let var8:i8 = 127; // maximo aceito pelo i8
    println!("tamanho i8: {}, valor armazenado: {}", std::mem::size_of_val(&var8), var8);

    let var16:i16 = 32767; // maximo aceito pelo i16
    println!("tamanho i16: {}, valor armazenado: {}", std::mem::size_of_val(&var16), var16);

    let var32:i32 = 2147483647; // maximo aceito pelo i32
    println!("tamanho i32: {}, valor armazenado: {}", std::mem::size_of_val(&var32), var32);

    let letra:char = 'C';
    println!("tamanho i32: {}, Letra: {}", std::mem::size_of_val(&letra), letra);
    
}

fn idade (idade:i8, responsavel:bool) { //condicionais
    
    let check = idade > 18;
    println!("Maior de 18: {}", check);
    
    if idade > 18 || idade>= 16 && responsavel{
        println!("Pode entrar")
    }else {
        println!("Não pode")
    }
    
    let result  = if idade > 18 || idade>= 16 && responsavel { println!("Pode entrar"); } else { println!("Não pode"); };

}

fn counter(aux:i8) { // repetições

    let mut rep:i8 = 0;

    while rep < aux {
        println!("Contando: {}", rep);
        sleep(time::Duration::from_millis(10));
        rep += 1;
    }

    rep = 0;

    loop { // = while true
        
        sleep(time::Duration::from_millis(10));
        
        if  rep == 10 {
            break;
        }
        println!("Contando2:    {}", rep);
        rep += 1;
    }

    
}

fn interval(aux:i8) {

    for a in 1..aux  { // mesma notação for a in 1..=aux
        println!("Contando: {}", a);
        sleep(time::Duration::from_millis(10));
    }

}

fn check_statment(aux: &str) { 

    match aux {
        "PHP" => println!("Linguagem WEB."), 
        "Kotlin" => println!("Android"),
        _ => println!("Other case"),
    }
}

fn matriz () {
    let mat = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
        ];

for linha in mat {
    for columm in linha {
        println!("Numeros da matriz: {}", columm);
    }
}

}

#[allow(dead_code)] // permite código morto
enum Color{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8}
}

fn cores(){
    //let cor = Color::Red;

    let cor = Color::CymkColor { cyan: 0, magenta: 0, yellow: 0, black: 255 };


    println!("Cor = {}", match cor {
        Color::Blue => "Azul",
        Color::Green => "Verde",
        Color::Red => "Vermelho",
        Color::RgbColor(_ , _, _) => "Rgb Desconhecida", // '_' não importa
        Color::RgbColor(0, 0, 0) | Color::CymkColor { cyan: _ , magenta: _ , yellow: _ , black: 255 }=> "Preto",
        Color::CymkColor { cyan: _ , magenta: _ , yellow: _ , black: _ } => "CYMK desconhecido"
        
    });


}



fn main() {
println!("Working!");

//println!("Soma = {}", soma(2, 2));
//var();

println!("PI: {}", PI);
println!("Variavel global: {}", GLOBAL_VAR);

//uso de variaveis globais é inseguro usando unsafe a responsabilidade é do programador
//unsafe {
//    println!("Valor CTE mutavel: {}", GLOBAL_VAR_MUT);  não existe mais
//}

//idade(16, true);
//vetores();

matriz();
cores();


//counter(15);

interval(15);
check_statment("PHP");

}
