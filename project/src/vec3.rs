// Módulo vec3

use std::ops;
use std::fmt;


pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Métodos
impl Vec3 {
    // "Construtor". Chama-se Vec3::new(x, y, z) para se criar um novo vetor
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    // Módulo (comprimento) do vetor
    pub fn module(&self) -> f64 {
        f64::sqrt(
            f64::powi(self.x, 2) + 
            f64::powi(self.y, 2) + 
            f64::powi(self.z, 2)
        )
    }

    // Rotacionar em radianos. Utiliza matriz de rotação
    // REFATORAR! UTILIZAR MATRIZ DE ROTAÇÃO EM R3 OU (SUGESTÃO) ROTAÇÃO UTILIZANDO QUATERNIÕES
    pub fn rotate(&mut self, theta_rad: f64) {
        // variáveis cos e sin para evitar computações extras
        let cos = f64::cos(theta_rad);
        let sin = f64::sin(theta_rad);
        let px = self.x; // "Prior x". X anterior
        self.x = cos * self.x + sin * self.y;
        self.y = -sin * px + cos * self.y;
    }

    // Produto escalar
    pub fn dot_prod(&self, v: &Vec3) -> f64 {
        self.x * v.x + 
        self.y * v.y + 
        self.z * v.z
    }
}

// Sobrecarga do operador de adição "+"
impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self { 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Sobrecarga do operador de subtração "-"
impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self { 
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Sobrecarga do operador de negação "-"
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { 
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Sobrecarga do operador de multiplicação "*" entre vetores
// Produto vetorial
impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self { 
            x: self.y * other.z - other.y * self.z,
            y: self.x * other.z - self.z * other.x,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

// Sobrecarga do operador de multiplicação "*" entre vetor e f64
impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, n: f64) -> Self::Output {
        Self { 
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
}

// Sobrecarga do operador de divisão "/" entre vetor e f64
impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, n: f64) -> Self::Output {
        Self { 
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

// Implementação da trait Display (println!)
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}, {:.4})", self.x, self.y)
    }
}

// Implementação da trait Debug (dbg!)
impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}, {:.4})", self.x, self.y)
    }
}

// https://doc.rust-lang.org/rust-by-example/trait/ops.html
// http://blog.wolfire.com/2009/07/linear-algebra-for-game-developers-part-1/
