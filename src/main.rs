#![feature(step_trait)]

mod intcode;
mod p8;
mod util;

use p8 as Main;

fn main() {
    Main::p1(Main::IN);
    Main::p2(Main::IN);
}
