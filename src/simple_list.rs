use crate::common::{setup_window, ListItem, State, HEIGHT, WIDTH};
use gpui::*;

pub struct Main {
    list_state: ListState,
    state_model: Model<State>,
}

impl Render for Main {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let state_model_clone = self.state_model.clone();
        let button = div()
            .flex()
            .p_2()
            .bg(rgb(0x2a2a2a))
            .rounded_md()
            .hover(|s| s.bg(rgb(0x3a3a3a)))
            .text_color(rgb(0xffffff))
            .text_xl()
            .cursor(CursorStyle::PointingHand)
            .child("Add Item")
            .on_mouse_down(MouseButton::Left, move |_mde, cx| {
                cx.update_model(&state_model_clone, |model, cx| {
                    let new_item =
                        ListItem::new(format!("Item {}", model.count), "Subtitle".to_string());
                    model.items.push(new_item);
                    model.count += 1;
                    cx.notify();
                });
            });

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(list(self.list_state.clone()).w_full().h_full())
            .child(
                div()
                    .flex()
                    .w_full()
                    .py_2()
                    .justify_center()
                    .items_center()
                    .child(button),
            )
    }
}

impl Main {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let state_model = cx.new_model(|_cx| State {
                count: 0,
                items: vec![],
            });

            cx.observe(&state_model, |this: &mut Main, model, cx| {
                let items = model.read(cx).items.clone();
                this.list_state = ListState::new(
                    items.len(),
                    ListAlignment::Bottom,
                    Pixels(20.),
                    move |idx, _cx| {
                        let item = items.get(idx).unwrap().clone();
                        div().child(item).into_any_element()
                    },
                );
            })
            .detach();

            Self {
                list_state: ListState::new(0, ListAlignment::Bottom, Pixels(20.), move |_, _| {
                    div().into_any_element()
                }),
                state_model,
            }
        })
    }
}

pub fn run_app(app: App) {
    app.run(|cx: &mut AppContext| {
        let window_options = setup_window(WIDTH, HEIGHT, cx);
        cx.open_window(window_options, |cx| Main::new(cx));
    });
}
