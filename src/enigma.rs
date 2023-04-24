use crate::mapeos;
use crate::mapeos::N;
use crate::configuracion::ConfiguraciónInicial;

use std::collections::HashMap;

type Encoder = HashMap<char,usize>;
type Decoder = HashMap<usize ,char>;
type Mapeo   = HashMap<usize,usize>;

struct Rotor{
    pos_o:usize,
    mapeo_ida: Mapeo,
    mapeo_vuelta: Mapeo,
}

impl Rotor{
    fn new(pos_o:usize, mapeo:[usize;N]) -> Self {
        let mut mapeo_ida    = Mapeo::new();
        let mut mapeo_vuelta = Mapeo::new();
        for (i,u) in mapeo.iter().enumerate(){
            mapeo_ida.insert(i as usize, *u);
            mapeo_vuelta.insert(*u, i as usize);
        }

        Rotor{
            pos_o:pos_o,
            mapeo_ida: mapeo_ida,
            mapeo_vuelta:mapeo_vuelta,
        }
    }

    fn rotación(&self,u:usize,pos:usize) -> usize {
        // Emula junto con rotación_r la rotación del rotor.
        let n_u = (u + self.pos_o + pos) % N;
        return n_u;
    }

    fn rotación_r(&self,u:usize,pos:usize) -> usize {
        // Deshace la acción de  la función "rotación" para emular el efecto de
        // de la rotación del rotor. Observar que es posible el caso
        // de pos > u, dado que pos depende de cuantas veces se haya llamado
        // a codificar_char,mientras que u es el indice del caracter es decir
        // máximo es N. Por eso se necesita la logica siguiente:

        let n_u:usize;
        if u < self.pos_o + pos {
            let dif = (self.pos_o + pos - u) % N;
            n_u =(N - dif) % N;
        }
        else {
            n_u = (u - self.pos_o - pos) % N;
        }
        return n_u;
    }

    fn ida(&self,u:usize,pos:usize) -> usize {
        let n_u:usize = self.rotación(u, pos);
        let n_u:usize = self.mapeo_ida[&n_u];
        return self.rotación_r(n_u, pos);
    }

    fn vuelta(&self,u:usize,pos:usize) -> usize {
        let n_u:usize = self.rotación(u, pos); 
        let n_u:usize = self.mapeo_vuelta[&n_u];
        return self.rotación_r(n_u, pos);
    }
}

struct Reflector{
    hash : HashMap<usize,usize>,
}
impl Reflector{
    fn new(mapeo1:[usize;N/2],mapeo2:[usize;N/2]) -> Self{
        let mut hash: Mapeo = HashMap::new();
        for (m1,m2) in mapeo1.iter().zip(mapeo2){
            hash.insert(*m1,m2);
            hash.insert(m2,*m1);
        }

        Reflector{
            hash:hash,
        }


    }
}

pub struct Enigma{
    estado : usize,
    pub pares  : Mapeo, 
    encoder: Encoder,
    decoder: Decoder,
    rotor1:Rotor,
    rotor2:Rotor,
    rotor3:Rotor,
    reflector: Reflector,
}
impl Enigma{
    pub fn new(codigo:String) -> Self {

        let conf = ConfiguraciónInicial::new(codigo);

        // Inicializamos los rotores.
        let rotor1 = Rotor::new(conf.pos1,mapeos::MAPEO1);
        let rotor2 = Rotor::new(conf.pos2,mapeos::MAPEO2);
        let rotor3 = Rotor::new(conf.pos3,mapeos::MAPEO3);
        let reflector = Reflector::new(mapeos::R1,mapeos::R2);

        // Generamos el encoder/decoder.
        let mut encoder = Encoder::new();
        let mut decoder = Decoder::new();
        for (i,c) in mapeos::CARACTERES.chars().enumerate(){
            encoder.insert(c,i);
            decoder.insert(i,c);
        };

        Enigma {
            estado: 0,
            pares: conf.pares,
            encoder:encoder,
            decoder:decoder,
            rotor1:rotor1,
            rotor2:rotor2,
            rotor3:rotor3,
            reflector:reflector,
        } 
    }

    fn codificar_char(&mut self,c:char) -> char{

        let index1 = self.estado;
        let index2 = self.estado / N;
        let index3 = self.estado / (N*N);

        let c_out:char;
        if self.encoder.contains_key(&c) {
            let mut u = self.encoder[&c];
            if self.pares.contains_key(&u){
                u = self.pares[&u];
            }
            u = self.rotor1.ida(u,index1);
            u = self.rotor2.ida(u,index2);
            u = self.rotor3.ida(u,index3);
            u = self.reflector.hash[&u];
            u = self.rotor3.vuelta(u,index3);
            u = self.rotor2.vuelta(u,index2);
            u = self.rotor1.vuelta(u,index1);
            if self.pares.contains_key(&u){
                u = self.pares[&u];
            }
            c_out = self.decoder[&u];
        }
        else {c_out = c }

        // Avanzamos el estado de la maquina para el siguiente caracter.
        self.estado += 1;

        return c_out;
    }

    pub fn codificar(&mut self,texto:&String) -> String{

        // Reiniciamos el estado de la maquina al original.
        self.estado = 0;

        let mut out = String::new();
        for c in texto.chars(){
            out += &self.codificar_char(c).to_string();
        }
        return out;
    }
}


// -----
// Tests
// -----

#[cfg(test)]
mod tests {
    use crate::enigma::Enigma;

    #[test]
    fn reflector() {
        let enigma = Enigma::new(String::from("abc"));
        let mut u: usize = 10;
        u = enigma.reflector.hash[&u];
        u = enigma.reflector.hash[&u];
        assert_eq!(u,10);
    }

    #[test]
    fn rotor(){
        let enigma = Enigma::new(String::from("abc"));
        let mut u: usize = 4;
        u = enigma.rotor1.ida(u,2);
        assert_eq!(u,84);
        u = enigma.rotor1.vuelta(u,2);
        assert_eq!(u,10);

    }

    #[test]
    fn encoder_decoder(){
        let enigma = Enigma::new(String::from("abc"));
        let c_in = 'a';
        let u = enigma.encoder[&c_in];
        let c_out = enigma.decoder[&u];
        assert_eq!(c_in,c_out);

    }
}
