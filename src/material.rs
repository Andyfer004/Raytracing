use crate::color::Color;
use image::{DynamicImage, GenericImageView};

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,           // Color difuso del material
    pub albedo: f32,              // Coeficiente de reflexión
    pub specular: f32,            // Coeficiente especular
    pub reflectivity: f32,        // Coeficiente de reflectividad
    pub transparency: f32,        // Transparencia del material
    pub has_texture: bool,        // Si el material tiene una textura
    pub texture: Option<DynamicImage>, // La textura opcional (cargada de un archivo)
}

impl Material {
    // Constructor para un material, con opción de cargar una textura desde el disco
    pub fn new(
        diffuse: Color,
        albedo: f32,
        specular: f32,
        reflectivity: f32,
        transparency: f32,
        has_texture: bool,
        texture_path: Option<&str>,
    ) -> Self {
        let texture = if let Some(path) = texture_path {
            Some(image::open(path).expect("Failed to load texture"))
        } else {
            None
        };

        Material {
            diffuse,
            albedo,
            specular,
            reflectivity,
            transparency,
            has_texture,
            texture,
        }
    }

    // Método para obtener el color de una textura en base a coordenadas UV
    pub fn get_color_from_texture(&self, u: f32, v: f32) -> Color {
        if let Some(texture) = &self.texture {
            // Calcular las coordenadas del píxel dentro de la imagen de la textura
            let x = ((u * texture.width() as f32).min(texture.width() as f32 - 1.0)) as u32;
            let y = ((v * texture.height() as f32).min(texture.height() as f32 - 1.0)) as u32;

            if x < texture.width() && y < texture.height() {
                let pixel = texture.get_pixel(x, y);
                return Color::new(pixel[0], pixel[1], pixel[2]);
            }
        }

        // Si no hay textura, devuelve el color difuso por defecto
        self.diffuse
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0), // Color negro por defecto
            albedo: 1.0,
            specular: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            has_texture: false,
            texture: None,
        }
    }
}

// Implementación personalizada de PartialEq para Material, ignorando la textura
impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.diffuse == other.diffuse &&
        self.albedo == other.albedo &&
        self.specular == other.specular &&
        self.reflectivity == other.reflectivity &&
        self.transparency == other.transparency &&
        self.has_texture == other.has_texture
        // Ignoramos la comparación de `texture` ya que `DynamicImage` no implementa `PartialEq`.
    }
}
