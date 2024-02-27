mod common;
mod global_list;
mod simple_list;

use gpui::*;

fn main() {
    let app = App::new();
    simple_list::run_app(app);
    // global_list::run_app(app);
}
