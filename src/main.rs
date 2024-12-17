#![feature(step_trait)]
// #![feature(generic_const_exprs)]

mod intcode;
// mod p9;
// mod p10;
// mod p11;
// mod p12; // slow
// mod p13;
// mod p14;
// mod p15;
mod p16;
mod y2024;
mod util;

use y2024::p17 as Main;
// use p16 as Main;

fn main() {
    Main::p2(Main::IN);
    // Main::p1(Main::IN);
}
 
