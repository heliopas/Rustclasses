// declaração de constante não é variavel
const PI:f32 = 3.1415;
// declaração de uma variavel global imutavel
static GLOBAL_VAR:u8 = 1;
// declaração de uma variavel global mutavel
static mut GLOBAL_VAR_MUT:u8 = 1;

// '->' especifica o retorno da função e o tipo
fn soma(a:i32, b:i32) -> i32{
    
println!("{} + {} = {}", a, b, a + b); // quando coloca ';' não armazena o resutado
return a + b // sem ';' retorna valor, não precisa de return para retornar valores da função

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


fn main() {
println!("Working!");

//println!("Soma = {}", soma(2, 2));
//var();

println!("PI: {}", PI);
println!("Variavel global: {}", GLOBAL_VAR);

//uso de variaveis globais é inseguro usando unsafe a responsabilidade é do programador
unsafe {
    println!("Valor CTE mutavel: {}", GLOBAL_VAR_MUT);
}



}
