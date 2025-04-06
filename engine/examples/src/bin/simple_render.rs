use gamerplex_renderer::{
    init,
    primitives::{create_cube, create_color_material},
    integration::{RenderRegistry, RenderUpdateSystem},
    components::{Camera, Light, MeshRenderer, Material},
    types::LightType,
    pipeline::{default_pbr_shader},
};
use gamerplex_renderer::RenderResource;

use gamerplex_math::{Vector3, Quaternion, Transform};

fn main() {
    // Initialize the renderer
    let context = init();
    let mut resource = RenderResource::new(context);
    
    // Load the default shader
    resource.load_shader("default", default_pbr_shader());
    
    // Handle window setup (using winit or similar in a real application)
    
    // Create a render system
    let mut render_system = RenderUpdateSystem::new(resource);
    
    // Create a registry
    let mut registry = RenderRegistry::new();
    
    // Create a camera
    let camera = Camera {
        position: Vector3::new(0.0, 2.0, 5.0),
        rotation: Quaternion::from_euler(
            -20.0_f32.to_radians(),
            0.0,
            0.0
        ),
        is_active: true,
        ..Default::default()
    };
    
    // Create a light
    let light = Light {
        light_type: LightType::Directional,
        position: Vector3::new(5.0, 5.0, 5.0),
        direction: Vector3::new(-1.0, -1.0, -1.0).normalize(),
        color: [1.0, 1.0, 1.0],
        intensity: 1.0,
        ..Default::default()
    };
    
    // Create a cube mesh and material
    // In a real application, this would be done with actual rendering resources
    let cube_transform = Transform::new(
        Vector3::new(0.0, 0.0, 0.0),
        Quaternion::identity(),
        Vector3::ones()
    );
    
    let red_material = create_color_material([1.0, 0.0, 0.0, 1.0]);
    
    // Register components
    registry.register_camera(1, camera);
    registry.register_light(2, light);
    registry.register_transform(3, cube_transform);
    
    // Main loop
    loop {
        // Update physics (if any)
        
        // Render the scene
        render_system.render();
        
        // Handle events and timing
        // In a real application, this would be managed by a game loop
        
        // For demonstration purposes, break after one frame
        break;
    }
}