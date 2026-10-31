use amane::{
    Align::Center, Button, Color, Column, Cursor::Pointer, Horizontal::Left, Layer::Bottom,
    LayerWindow, Margin, Media, Rectangle, Row, Service, Size::Parent, Text, Vertical::Top, Widget,
    children,
};

pub fn widgets() -> LayerWindow {
    let margin = 30;

    let media = Media::read();

    let symbol = if media.playing() { "||" } else { ">" };

    let media_title = if media.title().len() < 20 {
        media.title()
    } else {
        &format!("{}..", &media.title()[..20])
    };

    let media_artist = if media.artist().len() < 15 {
        media.artist()
    } else {
        &format!("{}..", &media.artist()[..15])
    };

    let label = if media.title() != "" {
        format!("{} - {}", media_title, media_artist)
    } else {
        "NOTHING PLAYING".to_string()
    };

    LayerWindow::new()
        .width(420.)
        .height(69.)
        .layer(Bottom)
        .anchor_vertical(Top)
        .anchor_horizontal(Left)
        .margin(Margin {
            top: margin,
            right: 0,
            bottom: 0,
            left: margin,
        })
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(Color::rgb(51, 51, 51))
                .radius(5.)
                .padding(10.)
                .opacity(0.5)
                .align_child(Center, Center)
                .child(
                    Column::new(children![
                        section(Text::new(label).color(Color::rgb(221, 221, 221))),
                        section(
                            Row::new(children![
                                button("<<", |_| Media::previous()),
                                button(symbol, |_| Media::play_pause()),
                                button(">>", |_| Media::next()),
                            ])
                            .gap(10.)
                        )
                    ])
                    .gap(5.),
                ),
        )
}

fn section(widget: impl Widget + 'static) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .align_child(Center, Center)
        .child(widget)
}

fn button(label: &str, clicked: fn(Button)) -> Rectangle {
    Rectangle::new()
        .width(40.)
        .height(Parent)
        .fill(Color::rgb(0, 0, 0))
        .align_child(Center, Center)
        .radius(5.)
        .on_click(clicked)
        .cursor(Pointer)
        .child(Text::new(label).color(Color::rgb(221, 221, 221)))
}
