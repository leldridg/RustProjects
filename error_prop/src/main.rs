use sdl2::*;
use sdl2::video::Window;

fn main() -> Result<(), String> {
    let sdl_context: Sdl = sdl2::init()?;

    let video_subsystem: VideoSubsystem = sdl_context.video()?;

    let window: Window = video_subsystem.window("Rust!", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to build window!");

    println!("If you can read this then everything worked.");
    Ok(())
}
