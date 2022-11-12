// Simple display/window device

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::render::TextureAccess;
use sdl2::pixels::PixelFormatEnum;
use std::time::Duration;

use crate::vm::{VM, Value};






/*
let mut event_pump = sdl_context.event_pump().unwrap();

let mut event_pump = sdl_context.event_pump().unwrap();
let mut i = 0;
'running: loop {
    i = (i + 1) % 255;
    canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    canvas.clear();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                break 'running
            },
            _ => {}
        }
    }

    // The rest of the game loop goes here...

    canvas.present();

    //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}
*/







struct Window<'a>
{
    width: u32,
    height: u32,



    // window id
    //window_id


    canvas: sdl2::render::Canvas<sdl2::video::Window>,

    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,

    texture: Option<Texture<'a>>,
}

// TODO: eventually we want to allow multiple windows
static mut WINDOW: Option<Window> = None;

pub fn window_create(vm: &mut VM)
{
    let width = 800;
    let height = 600;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();

    let window = Window {
        width,
        height,
        canvas,
        texture_creator,
        texture: None,
    };

    unsafe {
        WINDOW = Some(window)
    }

    unsafe {
        let window = WINDOW.as_mut().unwrap();

        window.texture = Some( window.texture_creator.create_texture(
            PixelFormatEnum::RGB24,
            TextureAccess::Streaming,
            width,
            height
        ).unwrap());
    }
}

pub fn window_show(vm: &mut VM)
{
    //println!("show the window");

    unsafe {
        let mut window = WINDOW.as_mut().unwrap();
        window.canvas.window_mut().show();

        let (width, height) = window.canvas.window().size();
        println!("width={}, height={}", width, height);
    }
}

pub fn window_copy_pixels(vm: &mut VM)
{
    // Get the address to copy pixel data from
    let src_addr = vm.pop();
    let data_ptr = vm.get_heap_ptr(src_addr.as_usize());

    unsafe {
        let mut window = WINDOW.as_mut().unwrap();

        // Update the texture
        let pitch = 3 * window.width as usize;
        let data_len = (3 * window.width * window.height) as usize;
        let pixel_slice = std::slice::from_raw_parts(data_ptr, data_len);
        window.texture.as_mut().unwrap().update(None, pixel_slice, pitch).unwrap();

        // Copy the texture into the canvas
        window.canvas.copy(
            &window.texture.as_ref().unwrap(),
            None,
            None
        ).unwrap();

        // Update the screen with any rendering performed since the previous call
        window.canvas.present();
    }
}
