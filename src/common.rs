use gpui::*;

pub static WIDTH: f64 = 400.0;
pub static HEIGHT: f64 = 600.0;

#[derive(Clone, Debug, IntoElement)]
pub struct ListItem {
    title: SharedString,
    subtitle: SharedString,
}

impl RenderOnce for ListItem {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x2a2a2a))
            .items_start()
            .p_2()
            .m_2()
            .rounded_md()
            .hover(|s| s.bg(rgb(0x3a3a3a)))
            .text_color(rgb(0xffffff))
            .text_xl()
            .child(self.title.clone())
            .child(div().flex().text_sm().child(self.subtitle.clone()))
    }
}

impl ListItem {
    pub fn new(title: String, subtitle: String) -> Self {
        ListItem {
            title: title.into(),
            subtitle: subtitle.into(),
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub count: usize,
    pub items: Vec<ListItem>,
}

// Thanks again to Matthias for the inspiration here
// https://github.com/MatthiasGrandl/Loungy
pub fn setup_window(app_width: f64, app_height: f64, cx: &mut AppContext) -> WindowOptions {
    let display_id_maybe = cx.displays().last().map(|d| d.id());
    let bounds_maybe = cx.displays().last().map(|d| d.bounds());
    let bounds = bounds_maybe.unwrap_or(Bounds {
        origin: Point::new(GlobalPixels::from(0.0), GlobalPixels::from(0.0)),
        size: Size {
            width: GlobalPixels::from(1920.0),
            height: GlobalPixels::from(1080.0),
        },
    });

    let mut options = WindowOptions::default();
    let center = bounds.center();

    options.focus = true;
    options.display_id = display_id_maybe;
    let width = GlobalPixels::from(app_width);
    let height = GlobalPixels::from(app_height);
    let x: GlobalPixels = center.x - width / 2.0;
    let y: GlobalPixels = center.y - height / 2.0;

    let bounds: Bounds<GlobalPixels> = Bounds::new(Point { x, y }, Size { width, height });
    options.bounds = WindowBounds::Fixed(bounds);
    options.titlebar = Some(TitlebarOptions::default());
    options.is_movable = true;
    options.kind = WindowKind::PopUp;
    options
}
