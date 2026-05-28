use everarcade_sdk::{game::CounterGame, input::PlayerInput, runtime::DeterministicRuntime};
fn main() {
    let mut rt = DeterministicRuntime::new(CounterGame);
    let _ = rt.tick(vec![PlayerInput::new(0, "p1", "inc")]).unwrap();
    println!("deterministic-example-ok");
}
