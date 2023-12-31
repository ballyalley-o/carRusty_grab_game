use rusty_engine::prelude::*;
use rand::prelude::*;


// initial state of Game state
struct GameState {
    high_score: u32,
    score: u32,
    cone_index: i32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            cone_index: 0,
            spawn_timer: Timer::from_seconds(2.0, true)
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.window_settings(WindowDescriptor {
        title: "CarRUSTY Grab Game".to_string(),
        ..Default::default()
    });

    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    let player = game.add_sprite("player", SpritePreset::RacingCarRed);
    player.translation = Vec2::new(0.00, 0.00);
    player.rotation = std::f32::consts::FRAC_PI_2;
    player.collision = true;

    let score = game.add_text("score", "SCORE: 0");
    score.translation = Vec2::new(520.00, 320.00);

    let high_score = game.add_text("high_score", "HIGH SCORE: 0");
    high_score.translation = Vec2::new(-520.00, 320.00);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // exits if q is pressed
    if engine.keyboard_state.just_pressed(KeyCode::Q) {
        engine.should_exit = true;
    }
    // keep text in place in the edge of the screen
    let offset =( (engine.time_since_startup_f64 * 3.0).cos() * 5.0) as f32;
    let score = engine.texts.get_mut("score").unwrap();
    score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
    score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;

    let high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 110.0;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;

    // handle collisions
for event in engine.collision_events.drain(..) {
    if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
        // remove the enemy car
        for label in [event.pair.0, event.pair.1] {
            if label != "player" {
                engine.sprites.remove(&label);
            }
        }
        game_state.score += 1;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = format!("SCORE: {}", game_state.score);
        println!("SCORE: {}", game_state.score);

        if game_state.score > game_state.high_score {
            game_state.high_score = game_state.score;
            let high_score = engine.texts.get_mut("high_score").unwrap();
            high_score.value = format!("HIGH SCORE: {}", game_state.high_score);
        }
        engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.3);
    }
}

// handle movements
let player = engine.sprites.get_mut("player").unwrap();
const MOVEMENT_SPEED: f32 = 100.00;

    if engine.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;

    }

    if engine.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;

    }

    if engine.keyboard_state.pressed_any(&[KeyCode::Right, KeyCode::D]) {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;

    }

    if engine.keyboard_state.pressed_any(&[KeyCode::Left, KeyCode::A]) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;

    }
    // handle mouse events
if engine.mouse_state.just_pressed(MouseButton::Left) {
    if let Some(mouse_location) = engine.mouse_state.location() {
        let label = format!("cone_{}", game_state.cone_index);
        game_state.cone_index += 1;
        let cone = engine.add_sprite(label.clone(), SpritePreset::RacingConeStraight);
            cone.translation = mouse_location;
            cone.collision = true;
    }
}

// handle enemy spawning
if game_state.spawn_timer.tick(engine.delta).just_finished() {
    let label = format!("cone_{}", game_state.cone_index);
        game_state.cone_index += 1;
        let cone = engine.add_sprite(label.clone(), SpritePreset::RacingConeStraight);
            cone.translation.x = thread_rng().gen_range(-550.0..550.0);
            cone.translation.y = thread_rng().gen_range(-325.0..325.0);
            cone.collision = true;
}


// handle reset score
if engine.keyboard_state.just_pressed(KeyCode::R) {
    game_state.score = 0;
    let score = engine.texts.get_mut("score").unwrap();
    score.value = "SCORE: 0".to_string();
}

}

