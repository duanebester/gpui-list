use gpui::Application;
use gpui_list::simple_list;

fn main() {
    let app = Application::new();
    simple_list::run_app(app);
}
