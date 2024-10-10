mod camera;
mod cube;
mod material;
mod color;
mod ray_intersect;
mod framebuffer;
mod intersect;

use nalgebra_glm::Vec3;
use crate::camera::Camera;
use crate::cube::Cube;
use crate::material::Material;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::RayIntersect;
use crate::intersect::Intersect;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent, DeviceEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Scene with Complete Frame")
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut framebuffer = Framebuffer::new(800, 600);
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(800, 600, surface_texture).expect("Failed to create pixel buffer");

    let mut camera = Camera::new(
        Vec3::new(0.0, 5.0, 14.0),  // Ajuste de la cámara para ver toda la estructura
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut yaw = 0.0;
    let mut pitch = 0.0;

    // Cargar las texturas desde la carpeta "textures"
    let texture_path = "textures/";

    let water_material = Material::new(Color::new(0, 105, 148), 0.5, 0.3, 0.1, 0.0, true, Some(&(texture_path.to_owned() + "water.png")));
    let sand_material = Material::new(Color::new(237, 201, 175), 0.7, 0.2, 0.05, 0.0, true, Some(&(texture_path.to_owned() + "earth.png")));
    let wood_material = Material::new(Color::new(139, 69, 19), 0.6, 0.2, 0.05, 0.0, true, Some(&(texture_path.to_owned() + "wood.png")));
    let leaves_material = Material::new(Color::new(34, 139, 34), 0.8, 0.3, 0.1, 0.0, true, Some(&(texture_path.to_owned() + "flowers.png")));
    let rock_material = Material::new(Color::new(100, 100, 100), 0.6, 0.3, 0.1, 0.0, true, Some(&(texture_path.to_owned() + "rock.png")));
    let grass_material = Material::new(Color::new(50, 200, 50), 0.8, 0.3, 0.1, 0.0, true, Some(&(texture_path.to_owned() + "grass.png")));

    // Tamaño de cada cubo
    let cube_size = 1.0;

    // Agua alrededor
    let water_cubes = vec![
        Cube::new(Vec3::new(-2.0, -1.5, -2.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(-1.0, -1.5, -2.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(0.0, -1.5, -2.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(1.0, -1.5, -2.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(2.0, -1.5, -2.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(-2.0, -1.5, -1.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(2.0, -1.5, -1.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(-2.0, -1.5, 0.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(2.0, -1.5, 0.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(-2.0, -1.5, 1.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(-1.0, -1.5, 1.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(0.0, -1.5, 1.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(1.0, -1.5, 1.0), cube_size, water_material.clone()),
        Cube::new(Vec3::new(2.0, -1.5, 1.0), cube_size, water_material.clone()),
    ];

    // Arena en la isla
    let sand_cubes = vec![
        Cube::new(Vec3::new(0.0, -1.0, 0.0), cube_size, sand_material.clone()),
        Cube::new(Vec3::new(-1.0, -1.0, 0.0), cube_size, sand_material.clone()),
        Cube::new(Vec3::new(1.0, -1.0, 0.0), cube_size, sand_material.clone()),
        Cube::new(Vec3::new(0.0, -1.0, 1.0), cube_size, sand_material.clone()),
        Cube::new(Vec3::new(0.0, -1.0, -1.0), cube_size, sand_material.clone()),
    ];

    // Tronco y hojas de palma
    let palm_tree = vec![
        Cube::new(Vec3::new(0.0, 0.0, 0.0), cube_size * 0.2, wood_material.clone()),
        Cube::new(Vec3::new(0.0, 0.5, 0.0), cube_size * 0.2, wood_material.clone()),
        Cube::new(Vec3::new(0.0, 1.0, 0.0), cube_size * 0.2, wood_material.clone()),
        Cube::new(Vec3::new(0.0, 1.5, 0.0), cube_size * 0.5, leaves_material.clone()),
        Cube::new(Vec3::new(-0.5, 1.5, 0.0), cube_size * 0.5, leaves_material.clone()),
        Cube::new(Vec3::new(0.5, 1.5, 0.0), cube_size * 0.5, leaves_material.clone()),
        Cube::new(Vec3::new(0.0, 1.5, -0.5), cube_size * 0.5, leaves_material.clone()),
        Cube::new(Vec3::new(0.0, 1.5, 0.5), cube_size * 0.5, leaves_material.clone()),
    ];

    // Rocas alrededor
    let rock_cubes = vec![
        Cube::new(Vec3::new(1.0, -1.0, 2.0), cube_size * 0.7, rock_material.clone()),
        Cube::new(Vec3::new(-1.5, -1.0, -1.5), cube_size * 0.5, rock_material.clone()),
        Cube::new(Vec3::new(1.5, -1.0, 0.5), cube_size * 0.4, rock_material.clone()),
    ];

    // Colina de pasto y tierra en el centro
    let hill_cubes = vec![
        Cube::new(Vec3::new(0.0, 0.0, 2.0), cube_size, sand_material.clone()),
        Cube::new(Vec3::new(0.0, 1.0, 2.0), cube_size, grass_material.clone()),
        Cube::new(Vec3::new(-1.0, 0.0, 2.0), cube_size, sand_material.clone()),
        Cube::new(Vec3::new(-1.0, 1.0, 2.0), cube_size, grass_material.clone()),
    ];

    // Agregar las aristas horizontales inferiores y superiores
    let frame_cubes = vec![
        // Aristas de las paredes y techo
        Cube::new(Vec3::new(-3.0, -1.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, 0.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, 1.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, 2.0, -3.0), cube_size, rock_material.clone()),

        Cube::new(Vec3::new(3.0, -1.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 0.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 1.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 2.0, -3.0), cube_size, rock_material.clone()),

        Cube::new(Vec3::new(-3.0, -1.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, 0.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, 1.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, 2.0, 2.0), cube_size, rock_material.clone()),

        Cube::new(Vec3::new(3.0, -1.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 0.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 1.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 2.0, 2.0), cube_size, rock_material.clone()),

        // Techo (aristas superiores)
        Cube::new(Vec3::new(-3.0, 3.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-2.0, 3.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-1.0, 3.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(0.0, 3.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(1.0, 3.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(2.0, 3.0, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 3.0, -3.0), cube_size, rock_material.clone()),

        Cube::new(Vec3::new(-3.0, 3.0, -2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 3.0, -2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, 3.0, -1.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 3.0, -1.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, 3.0, 0.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 3.0, 0.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, 3.0, 1.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 3.0, 1.0), cube_size, rock_material.clone()),

        Cube::new(Vec3::new(-3.0, 3.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-2.0, 3.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-1.0, 3.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(0.0, 3.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(1.0, 3.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(2.0, 3.0, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, 3.0, 2.0), cube_size, rock_material.clone()),

        // Aristas horizontales inferiores (parte de abajo)
        Cube::new(Vec3::new(-3.0, -1.5, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-2.0, -1.5, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-1.0, -1.5, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(0.0, -1.5, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(1.0, -1.5, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(2.0, -1.5, -3.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, -1.5, -3.0), cube_size, rock_material.clone()),

        Cube::new(Vec3::new(-3.0, -1.5, -2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, -1.5, -2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, -1.5, -1.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, -1.5, -1.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, -1.5, 0.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, -1.5, 0.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-3.0, -1.5, 1.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, -1.5, 1.0), cube_size, rock_material.clone()),

        Cube::new(Vec3::new(-3.0, -1.5, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-2.0, -1.5, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(-1.0, -1.5, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(0.0, -1.5, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(1.0, -1.5, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(2.0, -1.5, 2.0), cube_size, rock_material.clone()),
        Cube::new(Vec3::new(3.0, -1.5, 2.0), cube_size, rock_material.clone()),
    ];

    // Combina todos los objetos
    let mut objects: Vec<Box<dyn RayIntersect>> = Vec::new();
    objects.extend(water_cubes.into_iter().map(|c| Box::new(c) as Box<dyn RayIntersect>));
    objects.extend(sand_cubes.into_iter().map(|c| Box::new(c) as Box<dyn RayIntersect>));
    objects.extend(palm_tree.into_iter().map(|c| Box::new(c) as Box<dyn RayIntersect>));
    objects.extend(rock_cubes.into_iter().map(|c| Box::new(c) as Box<dyn RayIntersect>));
    objects.extend(hill_cubes.into_iter().map(|c| Box::new(c) as Box<dyn RayIntersect>));
    objects.extend(frame_cubes.into_iter().map(|c| Box::new(c) as Box<dyn RayIntersect>)); // Agregar el marco completo

    let mut time = 0.0; // Variable de tiempo para la animación

    event_loop.run(move |event, _, control_flow| {
        time += 0.016;  // Incrementa el tiempo por frame (~60 FPS)
        *control_flow = ControlFlow::Poll;
    
        match event {
            Event::RedrawRequested(_) => {
                render_scene(&camera, &objects, &mut framebuffer, &water_material, time);
    
                for (i, pixel) in framebuffer.pixels.iter().enumerate() {
                    let frame = pixels.get_frame();
                    let offset = i * 4;
                    frame[offset] = pixel.r;
                    frame[offset + 1] = pixel.g;
                    frame[offset + 2] = pixel.b;
                    frame[offset + 3] = 255;
                }
    
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => pixels.resize_surface(new_size.width, new_size.height),
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(winit::event::VirtualKeyCode::Up) = input.virtual_keycode {
                        camera.zoom(0.5);  // Acercar con la tecla de flecha hacia arriba
                    }
                    if let Some(winit::event::VirtualKeyCode::Down) = input.virtual_keycode {
                        camera.zoom(-0.5);  // Alejar con la tecla de flecha hacia abajo
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let zoom_amount = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, scroll) => scroll,  // Usar el scroll del mouse
                        _ => 0.0,
                    };
                    camera.zoom(zoom_amount * 0.5);  // Ajusta la velocidad del zoom
                }
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                    yaw += dx as f32 * 0.01;
                    pitch -= dy as f32 * 0.01;
                    pitch = pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.1, std::f32::consts::FRAC_PI_2 - 0.1);
    
                    let distance = 14.0;
                    let eye_x = distance * yaw.cos() * pitch.cos();
                    let eye_y = distance * pitch.sin();
                    let eye_z = distance * yaw.sin() * pitch.cos();
    
                    camera.eye = Vec3::new(eye_x, eye_y, eye_z);
                }
                _ => (),
            },
            _ => (),
        }
    
        window.request_redraw();
    });
    
}

fn render_scene(camera: &Camera, objects: &[Box<dyn RayIntersect>], framebuffer: &mut Framebuffer, water_material: &Material, time: f32) {
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let u = x as f32 / framebuffer.width as f32;
            let v = y as f32 / framebuffer.height as f32;

            let ray_direction = camera.get_ray_direction(u, v, framebuffer.aspect_ratio());

            let color = cast_ray(camera.eye, ray_direction, &objects, water_material, time);

            framebuffer.set_pixel(x, y, color);
        }
    }
}

fn fresnel(reflectivity: f32, view_dir: Vec3, normal: Vec3) -> f32 {
    let cos_theta = view_dir.dot(&normal).abs();
    let r0 = reflectivity;
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}
fn cast_ray(origin: Vec3, direction: Vec3, objects: &[Box<dyn RayIntersect>], water_material: &Material, time: f32) -> Color {
    let mut closest_intersection: Option<Intersect> = None;

    for object in objects {
        let intersection = object.ray_intersect(&origin, &direction);

        if intersection.is_intersecting {
            if closest_intersection.is_none() || intersection.distance < closest_intersection.as_ref().unwrap().distance {
                closest_intersection = Some(intersection);
            }
        }
    }

    if let Some(intersect) = closest_intersection {
        if intersect.material.has_texture {
            if intersect.material == *water_material {
                // Usar rem_euclid para que las coordenadas de textura sean cíclicas en el rango [0.0, 1.0]
                let animated_u = (intersect.u + time * 0.5).rem_euclid(1.0); // Mantener en [0.0, 1.0]
                let animated_v = (intersect.v + time * 0.5).rem_euclid(1.0);
                return intersect.material.get_color_from_texture(animated_u, animated_v);
            }
            return intersect.material.get_color_from_texture(intersect.u, intersect.v);
        }
        return intersect.material.diffuse;
    }

    Color::new(0, 129, 167)  // Color de fondo (cielo)
}
