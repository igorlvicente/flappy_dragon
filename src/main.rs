use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, context: &mut BTerm) {
        context.cls();
        context.print_centered(5, "Welcome to Flappy Dragon");
        context.print_centered(8, "(P) Play Game");
        context.print_centered(9, "(Q) Quit Game");

        if let Some(key) = context.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => context.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, _context: &mut BTerm) {
        // TODO: Implement this
        self.mode = GameMode::End;
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn dead(&mut self, context: &mut BTerm) {
        context.cls();
        context.print_centered(5, "You are dead");
        context.print_centered(8, "(P) Play Again");
        context.print_centered(9, "(Q) Quit Game");
        if let Some(key) = context.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => context.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(context),
            GameMode::Playing => self.play(context),
            GameMode::End => self.dead(context),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
