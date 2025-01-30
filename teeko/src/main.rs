use sdl2::event::Event;

mod view;
use view::board_view::Renderer;

mod model;
use model::game::{GameState, PieceDropCommand};

fn test_task_a() {


    let mut v: Vec<u32> = Vec::new();
    v.push(18);
    v.push(20);

    let mut size: usize = v.len();
    let mut capacity: usize = v.capacity();

    println!("Our vector has {} elements", size);
    println!("Our vector has space reserved for {} elements", capacity);

    for i in 0..size {
        println!("v[{}] = {}", i, v[i]);
    }

    v.pop();

    size = v.len();
    capacity= v.capacity();

    println!("Our vector has {} elements", size);
    println!("Our vector has space reserved for {} elements", capacity);

    for i in 0..size {
        println!("v[{}] = {}", i, v[i]);
    }

}

fn test_task_b(mut game: &mut GameState) {
    let command1: PieceDropCommand = PieceDropCommand{row: 0, col: 1, player: model::game::BoardPiece::Red};
    let command2: PieceDropCommand = PieceDropCommand{row: 0, col: 1, player: model::game::BoardPiece::Black};

    command1.perform(game);
    println!("Second drop valid: {}", command2.is_valid(game));
    command1.undo(game);
    println!("Second drop valid: {}", command2.is_valid(game));
} 

fn main() -> Result<(), String> {

    // test_task_a();

    let screen_width: u32 = 800;
    let screen_height: u32 = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Rust!", screen_width, screen_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();
    let texture_loader = canvas.texture_creator();

    let board_view: Renderer = Renderer::new(screen_width, screen_height, &texture_loader);

    let mut game_state = GameState::new();
    
    // test_task_b(&mut game_state);

    // game loop

    let mut running: bool = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                },
                Event::MouseButtonDown { x, y, .. } => {
                    let col: usize =  (x * 5 / board_view.screen_area.w).try_into().unwrap();
                    let row: usize =  (y * 5 / board_view.screen_area.h).try_into().unwrap();

                    game_state.handle_click(row, col);
                },
                Event::KeyDown { keycode, .. } => {
                    if keycode.unwrap() == sdl2::keyboard::Keycode::U {
                        game_state.undo_action();
                    }
                    else if keycode.unwrap() == sdl2::keyboard::Keycode::R {
                        game_state.redo_action();
                    }
                }
                _ => {}
            }
        }
        
        board_view.render(&mut canvas, &game_state.board);
        canvas.present(); // update display
    }

    Ok(())
}
