use crate::common::{setup_window, ListItem, State, HEIGHT, WIDTH};
use gpui::*;

#[derive(Clone)]
pub struct StateModel {
    pub inner: Model<State>,
}

impl StateModel {
    pub fn init(cx: &mut WindowContext) {
        let model = cx.new_model(|_cx| State {
            count: 0,
            items: vec![],
        });
        let this = Self { inner: model };
        cx.set_global(this.clone());
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut WindowContext), cx: &mut WindowContext) {
        if !cx.has_global::<Self>() {
            return;
        }
        cx.update_global::<Self, _>(|mut this, cx| {
            f(&mut this, cx);
        });
    }

    pub fn push(&self, item: ListItem, cx: &mut WindowContext) {
        self.inner.update(cx, |model, cx| {
            model.items.push(item.clone());
            model.count += 1;
            cx.emit(AddListItemEvent { list_item: item });
        });
    }
}

impl Global for StateModel {}

pub struct List {
    state: ListState,
}

impl Render for List {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(list(self.state.clone()).w_full().h_full())
    }
}

impl List {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let state = cx.global::<StateModel>().inner.clone();
            cx.subscribe(&state, |this: &mut List, model, _event, cx| {
                let items = model.read(cx).items.clone();
                this.state = ListState::new(
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

            List {
                state: ListState::new(0, ListAlignment::Bottom, Pixels(20.), move |_, _| {
                    div().into_any_element()
                }),
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct AddListItemEvent {
    pub list_item: ListItem,
}

impl EventEmitter<AddListItemEvent> for State {}

pub struct Input;

impl Render for Input {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                StateModel::update(
                    |this, cx| {
                        let count = this.inner.read(cx).count;
                        let item =
                            ListItem::new(format!("Item {count}"), "Description".to_string());
                        this.push(item, cx);
                    },
                    cx,
                );
            });

        div()
            .flex()
            .w_full()
            .py_2()
            .justify_center()
            .items_center()
            .child(button)
    }
}

impl Input {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Input)
    }
}

pub struct Workspace {
    pub list_view: View<List>,
    pub input_view: View<Input>,
}

impl Render for Workspace {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.list_view.clone())
            .child(self.input_view.clone())
    }
}

impl Workspace {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        StateModel::init(cx);
        let list_view = List::new(cx);
        let input_view = Input::new(cx);
        cx.new_view(|_cx| Workspace {
            list_view,
            input_view,
        })
    }
}

pub fn run_app(app: App) {
    app.run(|cx: &mut AppContext| {
        let window_options = setup_window(WIDTH, HEIGHT, cx);
        cx.open_window(window_options, |cx| Workspace::new(cx));
    });
}
