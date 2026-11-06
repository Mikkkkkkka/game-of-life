mod configuration;
mod game_of_life;

// use std::env;

fn main() {
    println!("-- Game of life! --");

    // let args: Vec<String> = env::args().collect();
    // let config = configuration::Config::new(args);

    let mut frame = game_of_life::GameFrame::empty(10, 10);
    println!("{}", frame);
    println!("---");
    frame.set_cell(game_of_life::Cell::Alive, 5, 4);
    frame.set_cell(game_of_life::Cell::Alive, 5, 5);
    frame.set_cell(game_of_life::Cell::Alive, 5, 6);
    println!("original:\n{}", frame);
    for i in 0..3 {
        println!("---");
        frame = frame.next();
        println!("step {i}:\n{}", frame);
    }
}
