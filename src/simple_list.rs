use crate::common::{HEIGHT, ListItem, State, WIDTH, setup_window};
use gpui::*;

pub struct Main {
    list_state: ListState,
    state_model: Entity<State>,
}

impl Main {
    fn render_entry(&mut self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
        let items = self.state_model.read(cx).items.clone();
        let item = items.get(ix).unwrap().clone();
        div().child(item).into_any_element()
    }
}

impl Render for Main {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            .on_mouse_down(MouseButton::Left, move |_mde, _window, cx| {
                state_model_clone.update(cx, |model, cx| {
                    let new_item = ListItem::new(
                        model.count,
                        format!("Item {}", model.count),
                        "Subtitle".to_string(),
                    );
                    model.items.push(new_item);
                    model.count += 1;
                    cx.emit(StateChangedEvent);
                });
            });

        div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                list(
                    self.list_state.clone(),
                    cx.processor(|this, ix, _window, cx| this.render_entry(ix, cx)),
                )
                .w_full()
                .h_full(),
            )
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
    pub fn new(app: &mut App) -> Entity<Self> {
        app.new(|cx| {
            let state_model = cx.new(|_cx| State {
                count: 0,
                items: vec![],
            });

            cx.subscribe(
                &state_model,
                |this: &mut Main, model, _event: &StateChangedEvent, cx| {
                    let items = model.read(cx).items.clone();
                    this.list_state = ListState::new(items.len(), ListAlignment::Bottom, px(20.));
                    cx.notify();
                },
            )
            .detach();

            Self {
                list_state: ListState::new(0, ListAlignment::Bottom, px(20.)),
                state_model,
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct StateChangedEvent;

impl EventEmitter<StateChangedEvent> for State {}

pub fn run_app(app: Application) {
    app.run(|cx: &mut App| {
        let window_options = setup_window(WIDTH, HEIGHT, cx);
        cx.open_window(window_options, |_window, app| Main::new(app))
            .unwrap();
    });
}
