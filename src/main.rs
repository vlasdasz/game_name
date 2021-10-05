mod controls_view;

use test_engine::{tools::Boxed, Screen};

use crate::controls_view::ControlsView;

fn main() {
    Screen::new((1000, 800).into())
        .add_debug_view()
        .add_view(ControlsView::boxed())
        .start_main_loop();
}
