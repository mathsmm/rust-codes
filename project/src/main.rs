// Declarações de módulos
pub mod constant;
pub mod vec3;

// Declarações de uso
use vec3::Vec3;
use constant::PI;

fn main() {
    let mut v1 = Vec3 { x: 1.0, y: 1.0 };

    let v1_mod = v1.module();
    println!("Módulo do vetor: {:.5}", v1_mod);

    v1.rotate(PI);
    println!("Vetor: {:?}", v1);

    let mut v2 = Vec3 { x: 2.0, y: 3.0 };

    println!("Produto escalar: {:.5}", v1.dot_prod(&v2));
}