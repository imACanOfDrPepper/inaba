use amane::{
    Align::Center,
    Button, Column, Cpu,
    Cursor::Pointer,
    Horizontal::{Left, Right},
    Layer::Bottom,
    LayerWindow, Margin, Media, Memory, Rectangle, Row, Service,
    Size::Parent,
    Text,
    Vertical::Top,
    Widget, children,
};

use crate::palette::{BLACK, BLACKER, WHITE};

pub fn media() -> LayerWindow {
    let margin = 40;

    let media = Media::read();

    let symbol = if media.playing() { "||" } else { ">" };

    let media_title = if media.title().chars().count() < 20 {
        media.title()
    } else {
        &format!("{}..", &media.title().chars().take(20).collect::<String>())
    };

    let media_artist = if media.artist().chars().count() < 15 {
        media.artist()
    } else {
        &format!("{}..", &media.artist().chars().take(15).collect::<String>())
    };

    let label = if media.title() != "" {
        format!("{} - {}", media_title, media_artist)
    } else {
        "NOTHING PLAYING".to_string()
    };

    LayerWindow::new()
        .width(650.)
        .height(90.)
        .layer(Bottom)
        .anchor_vertical(Top)
        .anchor_horizontal(Left)
        .namespace("inaba-desktop-widget")
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
                .fill(BLACK)
                .border(2., BLACKER)
                .radius(5.)
                .padding(12.)
                .opacity(0.5)
                .align_child(Center, Center)
                .child(
                    Column::new(children![
                        section(Text::new(label).color(WHITE).size(20.)),
                        section(
                            Row::new(children![
                                button("<<", |_| Media::previous()),
                                button(symbol, |_| Media::play_pause()),
                                button(">>", |_| Media::next()),
                            ])
                            .gap(12.)
                        )
                    ])
                    .gap(5.),
                ),
        )
}

pub fn system() -> LayerWindow {
    let margin = 40;

    let cpu = Cpu::read();
    let mem = Memory::read();

    LayerWindow::new()
        .width(200.)
        .height(90.)
        .layer(Bottom)
        .anchor_vertical(Top)
        .anchor_horizontal(Right)
        .namespace("inaba-desktop-widget")
        .margin(Margin {
            top: margin,
            right: margin,
            bottom: 0,
            left: 0,
        })
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(BLACK)
                .border(2., BLACKER)
                .radius(5.)
                .padding(12.)
                .opacity(0.5)
                .align_child(Center, Center)
                .child(section(Column::new(children![
                    Text::new(format!("[ CPU: {} ]", cpu.percent()))
                        .color(WHITE)
                        .size(20.),
                    Text::new(format!("[ MEM: {} ]", mem.percent()))
                        .color(WHITE)
                        .size(20.),
                ]))),
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
        .width(60.)
        .height(Parent)
        .fill(BLACKER)
        .align_child(Center, Center)
        .radius(5.)
        .on_click(clicked)
        .cursor(Pointer)
        .child(Text::new(label).color(WHITE).size(20.))
}
