use std::time::Duration;

use amane::{
    Align, Audio, Battery, Button, Center, Color, End, Full, Layer, LayerWindow, Margin, Padding,
    Parent, Rectangle, Row, Scroll, Service, Start, Text, Vertical, Widget, Workspace, Workspaces,
    Zone, children,
};

pub fn bar() -> LayerWindow {
    let margin_top = 10;
    let margin_side = 15;

    let padding_v = 3.;
    let padding_h = 10.;

    let battery_string = battery_string();

    let audio = Audio::read();

    let audio_string = if audio.muted() {
        String::from("[VOL: MUT]")
    } else {
        format!("[VOL: {}]", audio.volume())
    };

    let audio_color = audio_color();

    let time = &Clock::read().time;
    let date = &Clock::read().date;

    let workspaces = Workspaces::read();

    let mut workspace_text = String::new();

    for workspace in workspaces.list() {
        workspace_text.push_str(if workspace.id() == 1 {
            if workspace.focused() {
                "o"
            } else {
                "."
            }
        } else {
            if workspace.focused() {
                "---o"
            } else {
                "---."
            }
        });
    }

    LayerWindow::new()
        .width(Full)
        .height(25.)
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
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .radius(5.)
                .padding(Padding {
                    top: padding_v,
                    right: padding_h,
                    bottom: padding_v,
                    left: padding_h,
                })
                .fill(Color::rgb(51, 51, 51))
                .child(Row::new(children![
                    section(
                        Text::new(workspace_text).color(Color::rgb(221, 221, 221)),
                        Start
                    ),
                    section(
                        Text::new(format!("// {} // {} //", time, date))
                            .size(12.)
                            .color(Color::rgb(221, 221, 221)),
                        Center
                    ),
                    section(
                        Row::new(children![
                            Rectangle::new()
                                .width(Parent)
                                .height(Parent)
                                .on_click(clicked_audio)
                                .on_scroll(scrolled_audio)
                                .align_child(End, Center)
                                .child(Text::new(audio_string).size(12.).color(audio_color)),
                            Text::new(battery_string)
                                .size(12.)
                                .color(Color::rgb(221, 221, 221))
                        ])
                        .gap(12.)
                        .align(Center),
                        End
                    )
                ])),
        )
}

fn battery_string() -> String {
    if Battery::read().present() {
        let battery = Battery::read().percent();
        let is_charging = Battery::read().charging();

        let charge_text = if is_charging { " (CHR)" } else { "" };

        format!("[BAT: {battery}{charge_text}]")
    } else {
        "[BAT: NONE]".to_string()
    }
}

fn section(content: impl Widget + 'static, align: impl Into<Align>) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .align_child(align, Center)
        .child(content)
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

        let changed = time != self.time;

        self.time = time;

        changed
    }
}
