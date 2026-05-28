use everarcade_sdk::{game::CounterGame, input::PlayerInput, runtime::DeterministicRuntime};

pub fn run_template_tick() -> String {
    let mut rt = DeterministicRuntime::new(CounterGame);
    rt.tick(vec![PlayerInput::new(0, "player-a", "inc")]).expect("deterministic tick")
}
