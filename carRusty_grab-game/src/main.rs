use rusty_engine::prelude::*;


// initial state of Game state
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, false)
        }
    }
}

fn main() {
    let mut game = Game::new();

    let player = game.add_sprite("player", SpritePreset::RacingCarRed);
    player.translation = Vec2::new(0.00, 0.00);
    player.rotation = std::f32::consts::FRAC_PI_2;
    player.scale = 0.5;
    player.layer = 1.0;

    let temporary = game.add_sprite("temporary", SpritePreset::RacingCarBlue);
    temporary.translation = Vec2::new(30.00, 0.00);
    temporary.layer = 1.1;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
//    game_state.current_score += 1;
//    println!("CURRENT SCORE: {}", game_state.current_score);
}