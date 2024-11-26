#![feature(step_trait)]

mod intcode;
mod p9;
mod p10;
mod p11;
mod util;

use p11 as Main;

fn main() {
    Main::p1(Main::IN);
    Main::p2(Main::IN);

    // run_previous();
}

fn run_previous() {
    p9::p1(p9::IN);
    p9::p2(p9::IN);
    p10::p1(p10::IN);
    p10::p2(p10::IN);
}
