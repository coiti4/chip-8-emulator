use std::env;

use std::fs::File;
use std::io::Read;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use core::*;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

const TICKS_PER_FRAME: usize = 10;

fn draw_screen(chip8: &Emu, canvas: &mut Canvas<Window>) {
    // Clear canvas as black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen = chip8.get_screen();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (idx, pixel) in screen.iter().enumerate() {
        if *pixel {
            // position of pixel in screen
            let x = (idx % SCREEN_WIDTH) as i32;
            let y = (idx / SCREEN_WIDTH) as i32;

            // draw pixel at position (x, y) with scale
            canvas.fill_rect(Rect::new(x as i32 * SCALE as i32, y as i32 * SCALE as i32, SCALE, SCALE)).unwrap();
        }
    }

    // update canvas
    canvas.present();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <rom_file>");
        return;
    }

    let mut chip8 = Emu::new();

    let mut rom = File::open(&args[1]).expect("Failed to open ROM file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    chip8.load_rom(&buffer);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'gameloop,
                _ => {}
            }
        }
        
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        draw_screen(&chip8, &mut canvas);
    }
}
