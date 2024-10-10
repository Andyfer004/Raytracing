use crate::material::Material;
use nalgebra_glm::Vec3;

#[derive(Debug, Clone)]
pub struct Intersect {
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
    pub u: f32,
    pub v: f32,
    pub point: Vec3,    // Agregar el campo point para almacenar la posición de la intersección
    pub normal: Vec3,   // Agregar el campo normal para almacenar la normal en la intersección
}

impl Intersect {
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: &Material, u: f32, v: f32) -> Self {
        Intersect {
            distance,
            is_intersecting: true,
            material: material.clone(), // Clonamos el material para evitar moverlo
            u,
            v,
            point,   // Asignar el valor del punto de intersección
            normal,  // Asignar el valor de la normal en el punto de intersección
        }
    }

    pub fn empty() -> Self {
        Intersect {
            distance: 0.0,
            is_intersecting: false,
            material: Material::default(),
            u: 0.0,
            v: 0.0,
            point: Vec3::zeros(),   // Valor por defecto (0, 0, 0) para el punto
            normal: Vec3::zeros(),  // Valor por defecto (0, 0, 0) para la normal
        }
    }
}
