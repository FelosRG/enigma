pub mod mapeos;
pub mod enigma;
pub mod configuracion;

use std::env;


fn main() {
    // Obtenemos las opciones de comandos.
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("No se ha ingresado ningun texto por codificar :c");
    }
    else if args.len() == 2 {
        let texto = &args[1];
        let contraseña = String::from("abc"); // Default
        let mut enigma = enigma::Enigma::new(contraseña);
        let texto_out  = enigma.codificar(&texto);
        println!("{}",texto_out);
    }
    else if args.len() == 3 {
        let contraseña = String::from(&args[1]);
        let texto = &args[2];
        let mut enigma = enigma::Enigma::new(contraseña.clone());
        let texto_out  = enigma.codificar(&texto);
        println!("{}",texto_out);
    }
    else {
        println!("Solo se permiten máximo dos parámetros (contraseña,texto).\nSi el texto consta de mas de una palabra encerrar en comillas asi: \"ejemplo\"");
    }

}
