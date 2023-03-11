// Formulas
// ( F - 32 ) * ( 5 / 9 ) = C 
// ( C * (9 / 5) + 32 = F

use std::io;

fn main() {
    println!("Convertir a (F/C)-(0 salir): ");
    let mut farcel = String::new();

    io::stdin().read_line(&mut farcel)
        .expect("Error");

    let mut grados = String::new();

    if farcel.trim() == "C" {
        let celsius: f32;
        
        io::stdin()
            .read_line(&mut grados)
            .expect("Numero invalido");
        
        let grados: f32 = grados.trim().parse().expect("Error to convert"); 

        celsius = (grados - 32.0) * (5.0 / 9.0);
        println!("Los grados son: {}°C", celsius);
    }
}
