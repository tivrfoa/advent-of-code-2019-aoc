#![feature(step_trait)]

mod intcode;
mod p9;
mod util;

use p9 as Main;

fn main() {
    Main::p1(Main::IN);
    // Main::p2(Main::IN);
}
