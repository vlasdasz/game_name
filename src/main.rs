#![feature(default_free_fn)]
#![feature(result_option_inspect)]
#![feature(explicit_generic_args_with_impl_trait)]

mod controls_view;
mod game_level;
use test_engine::{paths::home, rtools::Boxed, Screen};

use crate::controls_view::ControlsView;

#[tokio::main]
async fn main() {
    let mut screen = Screen::new(&home().join("game_name/test_engine"), (1000, 680).into());
    screen.ui.set_view(ControlsView::boxed());
    screen.ui.add_debug_view();
    screen.start_main_loop();
}
