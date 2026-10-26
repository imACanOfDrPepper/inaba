use amane::{
    Color, Column, Horizontal, Layer, LayerWindow, Margin, Monitor, Notification, Notifications, Rectangle, Service, Size::Parent, Text, Urgency, Vertical, Widget
};

pub fn notifs(monitor: &Monitor) -> LayerWindow {
    let rem = monitor.width.min(monitor.height) as f32;
    let margin = (rem * 0.02) as i32;

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

    let notif_body = if !notif.body().is_empty() {
        format!(": {}", notif.body())
    } else {
        "".to_string()
    };

    let label = format!("{}{}", notif.summary(), notif_body);

    Rectangle::new()
        .width(Parent)
        .height(60.)
        .radius(12.)
        .fill(fill)
        .padding(12.)
        .on_click(move |_| Notifications::click(id))
        .child(Text::new(label).elide().color(Color::rgb(221, 221, 221)))
}
