use assets::Assets;
use game::Game;
mod game;
mod assets;
mod rl_misc;
mod context;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .resizable()
        .build();

    let assets = Assets::load(&mut rl, &thread).unwrap(); // FIXME

    let mut game = Game::new(rl, &thread, assets);

    while game.running() {
        game.update();
    }
}