use futures::executor::block_on;
use wgpu_toutorial::state::State;

fn main() {
    block_on(State::run());
}
