use test_engine::{gl_wrapper::Screen, tools::New, TestScreen};

fn main() {
    TestScreen::new()
        .set_size((1000, 800).into())
        .start_main_loop();
}
