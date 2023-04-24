use crate::mapeos::N;
use std::collections::HashMap;

pub struct ConfiguraciónInicial {
    pub pos1:usize,
    pub pos2:usize,
    pub pos3:usize,
    pub pares: HashMap<usize,usize>,
}

impl ConfiguraciónInicial{
    pub fn new(codigo:String) -> Self {

        // Convertimos los chars del string a un usize. Notar "%N"
        let ids_chars:Vec<usize> = codigo.chars().map(|x| x as usize % N).collect();

        // Los valores de inicialización de los rotores estará ligado a todos los 
        // caracteres del código de configuración.
        let mut pos1 = 0;
        let mut pos2 = 0;
        let mut pos3 = 0;
        for (i,u) in ids_chars.iter().enumerate(){
            if i%3 == 0 {pos1 += u};
            if i%3 == 1 {pos2 += u};
            if i%3 == 2 {pos3 += u};
        }

        // Formamos los pares de intercambio con los ids.
        let mut pares: HashMap<usize,usize>  = HashMap::new();
        for i in 0..ids_chars.len()-1{
            let u1 = ids_chars[i];
            let u2 = ids_chars[i+1];
            if !pares.contains_key(&u1){
                if !pares.contains_key(&u2){
                    pares.insert(u1,u2);
                    pares.insert(u2, u1);

                }
            }
        }

        ConfiguraciónInicial{
            pos1:pos1,
            pos2:pos2,
            pos3:pos3,
            pares:pares,
        }

    }
}