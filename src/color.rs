use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)] // Agregamos PartialEq para permitir la comparación
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    // Convertir Color a formato float (de 0.0 a 1.0) si es necesario
    pub fn to_f32(&self) -> (f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }
}

// Implementación del trait Add para sumar dos colores
impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: (self.r as u16 + other.r as u16).min(255) as u8,
            g: (self.g as u16 + other.g as u16).min(255) as u8,
            b: (self.b as u16 + other.b as u16).min(255) as u8,
        }
    }
}

// Implementación de la multiplicación de un Color por un f32
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            r: (self.r as f32 * rhs).min(255.0).max(0.0) as u8,
            g: (self.g as f32 * rhs).min(255.0).max(0.0) as u8,
            b: (self.b as f32 * rhs).min(255.0).max(0.0) as u8,
        }
    }
}

// Implementación de la multiplicación de dos colores
impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: (self.r as f32 * rhs.r as f32 / 255.0).min(255.0) as u8,
            g: (self.g as f32 * rhs.g as f32 / 255.0).min(255.0) as u8,
            b: (self.b as f32 * rhs.b as f32 / 255.0).min(255.0) as u8,
        }
    }
}
