mod controls_view;
mod game_level;

use test_engine::{tools::Boxed, Screen};

use crate::controls_view::ControlsView;

fn main() {
    Screen::new((1000, 680).into())
        .add_view(ControlsView::boxed())
        .add_debug_view()
        .start_main_loop();
}
