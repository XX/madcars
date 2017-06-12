extern crate amethyst;

use amethyst::Application;
use amethyst::State;
use amethyst::Trans;
use amethyst::Event;
use amethyst::VirtualKeyCode;
use amethyst::WindowEvent;
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::World;
use amethyst::project::Config;
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::Pipeline;
use amethyst::renderer::Layer;
use amethyst::renderer::pass::Clear;

struct Game;

impl State for Game
{
    fn on_start(&mut self, _: &mut World, _: &mut AssetManager, pipeline: &mut Pipeline)
    {
        let clear_layer = Layer::new("main", vec![Clear::new([0.0, 0.0, 0.0, 1.0])]);
        pipeline.layers = vec![clear_layer];
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