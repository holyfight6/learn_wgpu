use futures::executor::block_on;
use learn_wgpu::state::State;

fn main() {
    block_on(State::run());
}
