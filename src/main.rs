extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use specs::{World, WorldExt, Join};

use std::path::Path;
use std::time::Duration;
use std::collections::HashMap;

pub mod texture_manager;
pub mod utils;
pub mod components;
pub mod game;

const IMAGE_WIDTH: u32 = 100;
const IMAGE_HEIGHT: u32 = 100;
const OUTPUT_WIDTH: u32 = 50;
const OUTPUT_HEIGHT: u32 = 50;
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

fn render(
    canvas: &mut WindowCanvas,
    texture_manager: &mut texture_manager::TextureManager<WindowContext>,
    _texture_creator: &TextureCreator<WindowContext>,
    _font: &sdl2::ttf::Font,
    ecs: &World,
) -> Result<(), String> {
    let color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(color);
    canvas.clear();

    let positions = ecs.read_storage::<components::Position>();
    let renderables = ecs.read_storage::<components::Renderable>();

    for (renderable, pos) in (&renderables, &positions).join() {
        let src = Rect::new(0, 0, renderable.i_w, renderable.i_h);
        let (x, y) = (pos.x as i32, pos.y as i32);
        let dest = Rect::new(
            x - ((renderable.o_w / 2) as i32),
            y - ((renderable.o_h / 2) as i32),
            renderable.o_w,
            renderable.o_h,
        );

        let center = Point::new(
            (renderable.o_w / 2) as i32,
            (renderable.o_h / 2) as i32,
        );

        let texture = texture_manager.load(&renderable.tex_name)?;

        canvas.copy_ex(
            &texture, 
            src, // source rect
            dest, // destination rect
            pos.rot, // angle of rotation 
            center, // center of image
            false, // flip horizontally
            false, // flip vertically
        )?;
    }

    canvas.present();
    Ok(())
}

struct State { ecs: World }

fn main() -> Result<(), String> {
    println!("Starting SDL2 test");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust SDL2 Test", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to initialize video subsystem");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to initialize canvas");

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = texture_manager::TextureManager::new(&texture_creator);

    // load image
    texture_manager.load("img/space_ship.png")?;

    // prepare the fonts
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut event_pump = sdl_context.event_pump()?;
    let mut key_manager: HashMap<String, bool> = HashMap::new();

    // creating the world/ecs
    let mut gs = State {
        ecs: World::new()
    };

    // registering components to the ecs
    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Renderable>();
    gs.ecs.register::<components::Player>();

    // loading the world/ecs
    game::load_world(&mut gs.ecs);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                },
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        None => {},
                        Some(key) => {
                            utils::key_down(&mut key_manager, key.to_string());
                        }
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        None => {},
                        Some(key) => {
                            utils::key_up(&mut key_manager, key.to_string());
                        }
                    }
                },
                _ => {}
            }
        }

        game::update(&mut gs.ecs, &mut key_manager);
        render(&mut canvas, &mut texture_manager, &texture_creator, &font, &gs.ecs)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
