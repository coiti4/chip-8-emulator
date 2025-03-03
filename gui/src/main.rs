use std::fs::File;
use std::io::Read;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

use rfd::FileDialog;
use std::path::PathBuf;

use core::*;

const SCALE: u32 = 16;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

const TICKS_PER_FRAME: usize = 10;

fn keymap(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

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
            canvas.fill_rect(Rect::new(x * SCALE as i32, y * SCALE as i32, SCALE, SCALE)).unwrap();
        }
    }

    // update canvas
    canvas.present();
}

fn main() {
    // try to obtain the path of the ROM file
    let rom_path = std::env::args().nth(1).map(PathBuf::from).or_else(|| {
        // if the path is not provided, open a file dialog
        FileDialog::new()
            .add_filter("CHIP-8 ROM", &["ch8", "rom", "bin"])
            .set_directory("../roms") // Carpeta inicial (opcional)
            .pick_file()
    });

    let rom_path = match rom_path {
        Some(path) => path,
        None => {
            eprintln!("No ROM file selected, exiting...");
            std::process::exit(1);
        }
    };

    let mut chip8 = Emu::new();
    let mut rom = File::open(&rom_path).expect("Error opening ROM file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("Error reading ROM file");
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
    let mut paused = false; 

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..}=> break 'gameloop,
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    // Open file dialog to select a new ROM when Enter is pressed
                    let new_rom_path = FileDialog::new()
                        .add_filter("CHIP-8 ROM", &["ch8", "rom", "bin"])
                        .set_directory("../roms")
                        .pick_file();

                    let new_rom_path = match new_rom_path {
                        Some(path) => path,
                        None => continue,
                    };

                    rom = File::open(&new_rom_path).expect("Error opening ROM file");
                    buffer = Vec::new();
                    rom.read_to_end(&mut buffer).expect("Error reading ROM file");
                    chip8.reset();
                    chip8.load_rom(&buffer);
                    paused = false;
                },
                // pause/unpause the emulator with P or space
                Event::KeyDown { keycode: Some(Keycode::P), .. } | Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    paused = !paused;
                },
                Event::KeyDown { keycode: Some(key), .. } if !paused => {
                    if let Some(k) = keymap(key) {
                        chip8.keypress(k, true);
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } if !paused => {
                    if let Some(k) = keymap(key) {
                        chip8.keypress(k, false);
                    }
                },
                _ => {}
            }
        }
        
        if !paused {
            for _ in 0..TICKS_PER_FRAME {
                chip8.tick();
            }
            chip8.tick_timers();
        }
        draw_screen(&chip8, &mut canvas);
    }
}
