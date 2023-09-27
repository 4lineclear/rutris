//!
#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    missing_docs,
    rustdoc::all,
    future_incompatible
)]
#![warn(missing_debug_implementations)]
#![allow(clippy::enum_glob_use)]
fn main() {
    easy_sgr::println!(
        "\
    This is an overarching crate that contains files relevant to {[blue]}rutris{[]}.\n\n\
    \t- For the {[green]}LIB{[]} implementation see {[blue]}rutris-lib{[]}\n\
    \t- For the {[green]}TUI{[]} implementation see {[blue]}rutris-tui{[]}\n\
    \t- For the {[green]}GUI{[]} implementation see {[blue]}rutris-gui{[]}\n\
    "
    );
}
