// Um codigo em rust pra RECEBER ARGUMENTOS

// vamos ter que usar a lib env

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // primeiro eu recebir o valor no argumento depois coletei  e transformei em um Vec.

    println!("{}", args[1]);
}
