use crate::common::{HEIGHT, ListItem, State, WIDTH, setup_window};
use gpui::*;

#[derive(Clone)]
pub struct StateModel {
    pub inner: Entity<State>,
}

impl StateModel {
    pub fn init(app: &mut App) {
        let model = app.new(|_cx| State {
            count: 0,
            items: vec![],
        });
        let this = Self { inner: model };
        app.set_global(this.clone());
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut App), app: &mut App) {
        if !app.has_global::<Self>() {
            return;
        }
        app.update_global::<Self, _>(|mut this, cx| {
            f(&mut this, cx);
        });
    }

    pub fn push(&self, item: ListItem, cx: &mut App) {
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

impl List {
    fn render_entry(&mut self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.global::<StateModel>().inner.clone();
        let items = state.read(cx).items.clone();
        let item = items.get(ix).unwrap().clone();
        div().child(item).into_any_element()
    }
}

impl Render for List {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().flex().child(
            list(
                self.state.clone(),
                cx.processor(|this, ix, _window, cx| this.render_entry(ix, cx)),
            )
            .w_full()
            .h_full(),
        )
    }
}

impl List {
    pub fn new(app: &mut App) -> Entity<Self> {
        app.new(|cx| {
            let state = cx.global::<StateModel>().inner.clone();
            cx.subscribe(
                &state,
                |this: &mut List, model, _event: &AddListItemEvent, cx| {
                    let items = model.read(cx).items.clone();
                    this.state = ListState::new(items.len(), ListAlignment::Bottom, px(20.));
                    cx.notify();
                },
            )
            .detach();

            List {
                state: ListState::new(0, ListAlignment::Bottom, px(20.)),
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct AddListItemEvent {
    pub list_item: ListItem,
}

impl EventEmitter<AddListItemEvent> for State {}

pub struct Header {
    count: usize,
}

impl Render for Header {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("header")
            .flex()
            .w_full()
            .py_3()
            .justify_center()
            .items_center()
            .border_b_1()
            .border_color(rgb(0x3a3a3a))
            .text_color(rgb(0xffffff))
            .text_xl()
            .child(format!("Total Items: {}", self.count))
    }
}

impl Header {
    pub fn new(app: &mut App) -> Entity<Self> {
        app.new(|cx| {
            let state = cx.global::<StateModel>().inner.clone();
            cx.subscribe(
                &state,
                |this: &mut Header, model, _event: &AddListItemEvent, cx| {
                    this.count = model.read(cx).items.len();
                    cx.notify();
                },
            )
            .detach();

            Header { count: 0 }
        })
    }
}

pub struct Input;

impl Render for Input {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_click));

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
    pub fn new(app: &mut App) -> Entity<Self> {
        app.new(|_cx| Input)
    }

    fn on_click(&mut self, _event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        StateModel::update(
            |this, cx| {
                let count = this.inner.read(cx).count;
                let item = ListItem::new(count, format!("Item {count}"), "Description".to_string());
                this.push(item, cx);
            },
            cx,
        );
    }
}

pub struct Workspace {
    pub header_view: Entity<Header>,
    pub list_view: Entity<List>,
    pub input_view: Entity<Input>,
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.header_view.clone())
            .child(self.list_view.clone())
            .child(self.input_view.clone())
    }
}

impl Workspace {
    pub fn new(app: &mut App) -> Entity<Self> {
        StateModel::init(app);
        let header_view = Header::new(app);
        let list_view = List::new(app);
        let input_view = Input::new(app);
        app.new(|_cx| Workspace {
            header_view,
            list_view,
            input_view,
        })
    }
}

pub fn run_app(app: Application) {
    app.run(|cx: &mut App| {
        let window_options = setup_window(WIDTH, HEIGHT, cx);
        cx.open_window(window_options, |_window, app| Workspace::new(app))
            .unwrap();
    });
}
