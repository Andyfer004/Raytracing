use crate::material::Material;
use nalgebra_glm::Vec3;

#[derive(Debug, Clone)]
pub struct Intersect {
    pub distance: f32,         // Distancia desde el origen del rayo al punto de intersección
    pub is_intersecting: bool, // Indica si hubo intersección
    pub material: Material,    // Material del objeto en la intersección
    pub u: f32,                // Coordenada U para mapeo de texturas
    pub v: f32,                // Coordenada V para mapeo de texturas
    pub position: Vec3,           // Punto de intersección en el espacio 3D
    pub normal: Vec3,          // Normal en el punto de intersección
}

impl Intersect {
    // Constructor para crear un nuevo objeto Intersect con valores válidos
    pub fn new(position: Vec3, normal: Vec3, distance: f32, material: &Material, u: f32, v: f32) -> Self {
        Intersect {
            distance,
            is_intersecting: true,        // Se establece en true porque se detectó una intersección
            material: material.clone(),   // Clonar el material para evitar moverlo
            u,
            v,
            position,   // Asignar el punto de intersección
            normal,  // Asignar la normal en el punto de intersección
        }
    }

    // Constructor para un objeto Intersect vacío (sin intersección)
    pub fn empty() -> Self {
        Intersect {
            distance: 0.0,
            is_intersecting: false,       // No hubo intersección
            material: Material::default(), // Usar el material por defecto
            u: 0.0,
            v: 0.0,
            position: Vec3::zeros(),         // Punto por defecto (0, 0, 0)
            normal: Vec3::zeros(),        // Normal por defecto (0, 0, 0)
        }
    }
}
