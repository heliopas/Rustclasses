
// '->' especifica o retorno da função
fn soma(a:i32, b:i32) -> i32{
    
println!("{} + {} = {}", a, b, a + b); // quando coloca ';' não armazena o resutado
return a + b // sem ';' retorna valor 

}




fn main() {
println!("Hello, world!");

println!("Soma = {}", soma(2, 2));

}
