#![feature(step_trait)]

mod intcode;
mod p7;
mod util;

use p7 as Main;

fn main() {
    Main::p1(Main::IN);
    Main::p2(Main::IN);
}
