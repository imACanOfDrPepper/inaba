use std::time::Duration;

use amane::{
    Align, Audio, Battery, Button, Center, Color, End, Full, Layer, LayerWindow, Margin, Monitor,
    Padding, Parent, Rectangle, Row, Scroll, Service, Shadow, Start, Text, Vertical, Widget,
    Workspace, Workspaces, Zone, children,
};

pub fn bar(monitor: &Monitor) -> LayerWindow {
    let rem = monitor.width.min(monitor.height) as f32;
    let margin_top = (rem * 0.01) as i32;
    let margin_side = (rem * 0.02) as i32;

    let padding_v = rem * 0.005;
    let padding_h = rem * 0.02;

    let battery_string = battery_string();

    let audio = Audio::read();

    let audio_string = if audio.muted() {
        String::from("   0%")
    } else {
        format!("  {}%", audio.volume())
    };

    let audio_color = audio_color();

    let time = &Clock::read().time;
    let date = &Clock::read().date;

    let workspaces = Workspaces::read();

    let mut workspace_buttons: Vec<Box<dyn Widget>> = Vec::new();

    for workspace in workspaces.list() {
        workspace_buttons.push(Box::new(workspace_button(workspace)));
    }

    LayerWindow::new()
        .width(Full)
        .height(rem * 0.04)
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
                .radius(12.)
                .padding(Padding {
                    top: padding_v,
                    right: padding_h,
                    bottom: padding_v,
                    left: padding_h,
                })
                .fill(Color::rgb(51, 51, 51))
                .shadow(Shadow::inner("#000000").opacity(0.15).blur(8.0))
                .child(Row::new(children![
                    section(Row::new(workspace_buttons).gap(5.), Start),
                    section(
                        Text::new(format!("{} · {}", time, date))
                            .size(rem * 0.02)
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
                                .child(Text::new(audio_string).size(rem * 0.02).color(audio_color)),
                            Text::new(battery_string)
                                .size(rem * 0.02)
                                .color(Color::rgb(221, 221, 221))
                        ])
                        .gap(12.),
                        End
                    )
                ])),
        )
}

fn battery_string() -> String {
    if Battery::read().present() {
        let battery = Battery::read().percent();
        let is_charging = Battery::read().charging();

        let battery_icon = if is_charging {
            if battery < 5 {
                "󰢟"
            } else if battery < 10 {
                "󰢜"
            } else if battery < 20 {
                "󰂆"
            } else if battery < 30 {
                "󰂇"
            } else if battery < 40 {
                "󰂈"
            } else if battery < 50 {
                "󰢝"
            } else if battery < 60 {
                "󰂉"
            } else if battery < 70 {
                "󰢞"
            } else if battery < 80 {
                "󰂊"
            } else if battery < 90 {
                "󰂋"
            } else {
                "󰂅"
            }
        } else {
            if battery < 5 {
                "󱃍"
            } else if battery < 10 {
                "󰁺"
            } else if battery < 20 {
                "󰁻"
            } else if battery < 30 {
                "󰁼"
            } else if battery < 40 {
                "󰁽"
            } else if battery < 50 {
                "󰁾"
            } else if battery < 60 {
                "󰁿"
            } else if battery < 70 {
                "󰂀"
            } else if battery < 80 {
                "󰂁"
            } else if battery < 90 {
                "󰂂"
            } else {
                "󰁹"
            }
        };

        format!("{battery_icon} {battery}%")
    } else {
        "".to_string()
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
            time: amane::output("date +%H:%M"),
            date: amane::output("date +'%b %d, %a'"),
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

fn workspace_button(workspace: &Workspace) -> Rectangle {
    let fill = if workspace.focused() {
        "#dddddd"
    } else if workspace.urgent() {
        "#ff9999"
    } else {
        "#b2b2b2"
    };

    let id = workspace.id();

    let width = if workspace.focused() {
        30.
    } else {
        20.
    };

    Rectangle::new()
        .width(width)
        .height(20.)
        .fill(fill)
        .radius(15.)
        .align_child(Center, Center)
        .on_click(move |_| Workspaces::focus(id))
        .child(Text::new(workspace.index().to_string()).size(12.).color(Color::rgb(51, 51, 51)))
}
