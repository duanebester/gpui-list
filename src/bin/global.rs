use gpui::Application;
use gpui_list::global_list;

fn main() {
    let app = Application::new();
    global_list::run_app(app);
}
