use amane::{
    Align, Column, Horizontal, Layer, LayerWindow, Margin, Notification, Notifications, Rectangle,
    Service, Size::Parent, Text, Urgency, Vertical, Widget, children,
};

use crate::palette::{BLACK, RED, WHITE};

pub fn notifs() -> LayerWindow {
    let margin = 40;

    let notifs = Notifications::read();

    let mut cards: Vec<Box<dyn Widget>> = Vec::new();

    for notif in notifs.list() {
        cards.push(Box::new(card(notif)));
    }

    if notifs.list().len() > 0 {
        LayerWindow::new()
            .width(600.)
            .height(600.)
            .anchor_vertical(Vertical::Top)
            .anchor_horizontal(Horizontal::Right)
            .layer(Layer::Overlay)
            .margin(Margin {
                top: margin,
                right: margin,
                bottom: 0,
                left: 0,
            })
            .child(Column::new(cards).gap(8.))
    } else {
        LayerWindow::new()
            .width(600.)
            .height(600.)
            .anchor_vertical(Vertical::Top)
            .anchor_horizontal(Horizontal::Right)
            .click_through()
            .child(Column::new(cards).gap(8.))
    }
}

fn card(notif: &Notification) -> Rectangle {
    let fill = match notif.urgency() {
        Urgency::Critical => RED,
        _ => BLACK,
    };

    let id = notif.id();

    Rectangle::new()
        .width(Parent)
        .height(80.)
        .radius(5.)
        .fill(fill)
        .padding(12.)
        .border(1., WHITE)
        .on_click(move |_| Notifications::click(id))
        .align_child(Align::Start, Align::Center)
        .child(Column::new(children![
            section(Text::new(notif.summary()).elide().color(WHITE).size(20.)),
            section(Text::new(notif.body()).elide().color(WHITE)),
        ]))
}

fn section(widget: impl Widget + 'static) -> Rectangle {
    Rectangle::new().width(Parent).height(Parent).child(widget)
}

pub fn clear_notifs(_: &[String]) -> String {
    Notifications::clear();

    String::from("cleared")
}
