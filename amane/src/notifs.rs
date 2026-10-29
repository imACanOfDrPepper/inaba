use amane::{
    Align, Color, Column, Horizontal, Layer, LayerWindow, Margin, Notification, Notifications, Rectangle, Service, Size::Parent, Text, Urgency, Vertical, Widget, children
};

pub fn notifs() -> LayerWindow {
    let margin = 30;

    let notifs = Notifications::read();

    let mut cards: Vec<Box<dyn Widget>> = Vec::new();

    for notif in notifs.list() {
        cards.push(Box::new(card(notif)));
    }

    LayerWindow::new()
        .width(400.)
        .height(400.)
        .anchor_vertical(Vertical::Top)
        .anchor_horizontal(Horizontal::Right)
        .layer(Layer::Overlay)
        .margin(Margin {
            top: margin,
            right: margin,
            bottom: 0,
            left: 0,
        })
        .visible(!notifs.list().is_empty())
        .child(Column::new(cards).gap(8.))
}

fn card(notif: &Notification) -> Rectangle {
    let fill = match notif.urgency() {
        Urgency::Critical => "#aa4444",
        _ => "#333333",
    };

    let id = notif.id();

    Rectangle::new()
        .width(Parent)
        .height(60.)
        .radius(12.)
        .fill(fill)
        .padding(10.)
        .on_click(move |_| Notifications::click(id))
        .align_child(Align::Start, Align::Center)
        .child(Column::new(children![
            section(
                Text::new(notif.summary())
                    .elide()
                    .color(Color::rgb(221, 221, 221))
            ),
            section(
                Text::new(notif.body())
                    .elide()
                    .color(Color::rgb(221, 221, 221))
                    .size(12.)
            ),
        ]))
}

fn section(widget: impl Widget + 'static) -> Rectangle {
    Rectangle::new().width(Parent).height(Parent).child(widget)
}

pub fn clear_notifs(_: &[String]) -> String {
    Notifications::clear();

    String::from("cleared")
}
