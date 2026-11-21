use std::time::Duration;

use amane::{
    Align, Audio, Battery, Button, Center, Color, Cursor::Pointer, End, Full,
    Justify::SpaceBetween, Layer, LayerWindow, Margin, Padding, Parent, Rectangle, Row, Scroll,
    Service, Start, Text, Vertical, Widget, Workspaces, Zone, children,
};

pub fn bar() -> LayerWindow {
    let margin_top = 12;
    let margin_side = 20;

    let battery_string = battery_string();

    let audio = Audio::read();

    let audio_string = if audio.muted() {
        String::from("[ VOL: MUT ]")
    } else {
        format!("[ VOL: {} ]", audio.volume())
    };

    let audio_color = audio_color();

    let time = &Clock::read().time;
    let date = &Clock::read().date;

    let workspaces = Workspaces::read();

    let mut workspace_buttons: Vec<Box<dyn Widget>> = Vec::new();

    if workspaces.list().len() > 0 {
        for i in 0..=workspaces.list().len() - 1 {
            let workspace = &workspaces.list()[i];

            let ws_id = workspace.id();
            let ws_focus = workspace.focused();

            let workspace_text = if ws_focus { "o" } else { "." };

            workspace_buttons.push(Box::new(
                Rectangle::new()
                    .width(10.)
                    .height(25.)
                    .cursor(Pointer)
                    .child(
                        Text::new(workspace_text)
                            .color(Color::rgb(221, 221, 221))
                            .size(20.),
                    )
                    .on_click(move |_| Workspaces::focus(ws_id)),
            ));

            if i != workspaces.list().len() - 1 {
                workspace_buttons.push(Box::new(
                    Rectangle::new()
                        .width(32.)
                        .height(25.)
                        .child(Text::new("---").color(Color::rgb(221, 221, 221)).size(20.)),
                ))
            }
        }
    }

    let ws_width = (50 * workspaces.list().len()) as f32;

    LayerWindow::new()
        .width(Full)
        .height(32.)
        .margin(Margin {
            top: margin_top,
            right: margin_side,
            bottom: 0,
            left: margin_side,
        })
        .anchor_vertical(Vertical::Top)
        .layer(Layer::Top)
        .space(Zone::Reserve)
        .child(
            Rectangle::new().width(Parent).height(Parent).child(
                Row::new(children![
                    pill(Row::new(workspace_buttons).gap(5.), Start, ws_width),
                    pill(
                        Text::new(format!("// {} // {} //", time, date))
                            .color(Color::rgb(221, 221, 221)),
                        Center,
                        280.
                    ),
                    pill(
                        Rectangle::new()
                            .width(100.) // This is set to less than required, so Amane automagically chooses the right value
                            .height(Parent)
                            .align_child(Center, Center)
                            .child(
                                Row::new(children![
                                    Rectangle::new()
                                        .width(110.)
                                        .height(Parent)
                                        .on_click(clicked_audio)
                                        .on_scroll(scrolled_audio)
                                        .cursor(Pointer)
                                        .align_child(Center, Center)
                                        .child(Text::new(audio_string).color(audio_color)),
                                    Text::new(battery_string).color(Color::rgb(221, 221, 221))
                                ])
                                .gap(5.)
                                .align(Center)
                            ),
                        End,
                        260.
                    )
                ])
                .justify(SpaceBetween),
            ),
        )
}

fn battery_string() -> String {
    if Battery::read().present() {
        let battery = Battery::read().percent();
        let is_charging = Battery::read().charging();

        let charge_text = if is_charging { " (CHR)" } else { "" };

        format!("[ BAT: {battery}{charge_text} ]")
    } else {
        "[ BAT: NONE ]".to_string()
    }
}

fn section(content: impl Widget + 'static, align: impl Into<Align>) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .align_child(align, Center)
        .child(content)
}

fn pill(content: impl Widget + 'static, align: impl Into<Align>, width: f32) -> Rectangle {
    let padding_v = 5.;
    let padding_h = 12.;

    section(
        Rectangle::new()
            .width(width)
            .height(Parent)
            .fill(Color::rgb(51, 51, 51))
            .radius(5.)
            .padding(Padding {
                top: padding_v,
                right: padding_h,
                bottom: padding_v,
                left: padding_h,
            })
            .align_child(Center, Center)
            .child(content),
        align,
    )
}

fn clicked_audio(_: Button) {
    Audio::toggle_mute();
}

fn scrolled_audio(scroll: Scroll) {
    let volume = i32::from(Audio::read().volume());

    let step: i32 = (scroll.y * 5.).round() as i32;

    let changed = (volume + step).clamp(0, 100);

    Audio::set_volume(changed as u8);
}

fn audio_color() -> Color {
    if Audio::read().muted() {
        Color::rgb(255, 153, 153)
    } else {
        Color::rgb(221, 221, 221)
    }
}

struct Clock {
    time: String,
    date: String,
}

impl Service for Clock {
    fn new() -> Self {
        Self {
            time: amane::output("date +%^H:%^M"),
            date: amane::output("date +'%^b %d, %^a'"),
        }
    }

    fn interval() -> Duration {
        Duration::from_secs(1)
    }

    fn update(&mut self) -> bool {
        let time = amane::output("date +%H:%M");
        let date = amane::output("date +'%^b %d, %^a'");

        let changed = time != self.time || date != self.date;

        self.time = time;
        self.date = date;

        changed
    }
}
