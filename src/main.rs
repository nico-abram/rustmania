#[macro_use]
extern crate nom;
extern crate ggez;
extern crate chrono;

mod gameplay_screen;
mod player_config;
mod notedata;
mod fraction;
mod timingdata;

use ggez::conf;
use std::fs::File;

fn main() {
    let c = conf::Conf::from_toml_file(&mut File::open("src/config.toml").unwrap()).unwrap();
    let context = &mut ggez::Context::load_from_conf("rustmania", "ixsetf", c).unwrap();

    let mut p1_layout = player_config::NoteLayout::new([72, 136, 200, 264], ggez::graphics::Image::new(context, "/arrow.png").unwrap(),
                                                        ggez::graphics::Image::new(context, "/receptor.png").unwrap(), 250);

    let mut p2_layout = player_config::NoteLayout::new([472, 536, 600, 664], ggez::graphics::Image::new(context, "/arrow.png").unwrap(),
                                                       ggez::graphics::Image::new(context, "/receptor.png").unwrap(), 250);

    if let Err(e) = p1_layout.set_scroll_speed(0.7){
        println!("Couldn't set scroll speed: {}", e);
    }

    if let Err(e) = p2_layout.set_scroll_speed(1.4){
        println!("Couldn't set scroll speed: {}", e);
    }

    let notedata = notedata::NoteData::from_sm();

    let notes = timingdata::TimingData::from_notedata(notedata, &p1_layout.arrow_sprite);

    let mut game_screen = gameplay_screen::GameplayScreen::new(&p1_layout, &notes, &p2_layout, &notes, 600);

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        context.filesystem.mount(&path, true);
    }

    game_screen.start();
    if let Err(e) = ggez::event::run(context, &mut game_screen) {
        println!("Error: {}", e);
    } else {
        println!("Exit successful.");
    }
}
