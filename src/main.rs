extern crate amethyst;
extern crate genmesh;
extern crate cgmath;

mod geometry;

use amethyst::Application;
use amethyst::State;
use amethyst::Trans;
use amethyst::Event;
use amethyst::VirtualKeyCode;
use amethyst::WindowEvent;
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::Gate;
use amethyst::ecs::components::Mesh;
use amethyst::ecs::components::Texture;
use amethyst::ecs::resources::Camera;
use amethyst::ecs::resources::Projection;
use amethyst::ecs::resources::ScreenDimensions;
use amethyst::ecs::World;
use amethyst::gfx_device::DisplayConfig;
use amethyst::project::Config;
use amethyst::renderer::AmbientLight;
use amethyst::renderer::Layer;
use amethyst::renderer::Pipeline;
use amethyst::renderer::PointLight;
use amethyst::renderer::VertexPosNormal;
use amethyst::renderer::pass::Clear;
use amethyst::renderer::pass::DrawShaded;
use geometry::*;

struct Game;

impl State for Game
{
    fn on_start(&mut self, world: &mut World, assets: &mut AssetManager, pipeline: &mut Pipeline)
    {
        let clear_layer = Layer::new("main", vec![
            Clear::new([0.4, 0.5, 0.7, 1.0]),
            DrawShaded::new("main", "main")
        ]);
        pipeline.layers = vec![clear_layer];

        {
            let dimensions = world.read_resource::<ScreenDimensions>().pass();

            let mut camera = world.write_resource::<Camera>().pass();
            camera.proj = Projection::Perspective {
                fov: 60.0,
                aspect_ratio: dimensions.aspect_ratio,
                near: 0.1,
                far: 100.0,
            };
            camera.eye = [5.0, 0.0, 0.0];
            camera.target = [0.0, 0.0, 0.0];

            let mut ambient_light = world.write_resource::<AmbientLight>().pass();
            ambient_light.power = 0.2;
        }

        let sphere_verts = gen_sphere(32, 32);
        assets.register_asset::<Mesh>();
        assets.register_asset::<Texture>();
        assets.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("sphere", sphere_verts);
        assets.load_asset_from_data::<Texture, [f32; 4]>("red", [1.0, 0.0, 0.0, 1.0]);
        assets.load_asset_from_data::<Texture, [f32; 4]>("white", [1.0, 1.0, 1.0, 1.0]);

        let sphere = assets.create_renderable("sphere", "red", "white", "white", 1.0).unwrap();
        world.create_now().with(sphere).build();

        let light = PointLight {
            center: [2.0, 2.0, 2.0],
            radius: 5.0,
            intensity: 3.0,
            ..Default::default()
        };
        world.create_now().with(light).build();
    }

    fn handle_events(&mut self,
                     events: &[WindowEvent],
                     _: &mut World,
                     _: &mut AssetManager,
                     _: &mut Pipeline)
        -> Trans
    {
        for event in events {
            match **event {
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return Trans::Quit,
                Event::Closed => return Trans::Quit,
                _ => (),
            }
        }
        Trans::None
    }
}

fn main()
{
    let config = DisplayConfig::load("resources/config/display.yml");
    let mut game = Application::build(Game, config).done();
    game.run();
}