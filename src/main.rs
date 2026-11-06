mod configuration;
mod game_of_life;

// use std::env;

fn main() {
    println!("-- Game of life! --");

    // let args: Vec<String> = env::args().collect();
    // let config = configuration::Config::new(args);

    {
        use game_of_life::{Cell, GameFrame};
        let mut frame = GameFrame::empty(64, 36);
        frame.set_cell_at(4, 7, Cell::Alive);
        frame.set_cell_at(5, 7, Cell::Alive);
        frame.set_cell_at(6, 7, Cell::Alive);
        frame.set_cell_at(6, 6, Cell::Alive);
        frame.set_cell_at(5, 5, Cell::Alive);
        println!("original:\n{}", frame);
        for i in 1..6 {
            println!("---");
            frame = frame.next();
            println!("step {i}:\n{frame}");
        }
    }
}
