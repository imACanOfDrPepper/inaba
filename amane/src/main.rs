use amane::{
    Align, App, Battery, Center, Color, Column, End, Full, Horizontal, Layer, LayerWindow, Margin, Monitor, Padding, Parent, Rectangle, Row, Service, Shadow, Start, Text, Vertical, Zone, children
};

fn main() {
    App::new()
        .window_per_monitor(bar)
        .window_per_monitor(activate_linux)
        .run();
}

fn activate_linux(monitor: &Monitor) -> LayerWindow {
    let rem = monitor.width.min(monitor.height) as f32;
    let margin = (rem * 0.03) as i32;

    LayerWindow::new()
        .width(monitor.width as f32 * 0.2)
        .height(monitor.height as f32 * 0.1)
        .anchor_vertical(Vertical::Bottom)
        .anchor_horizontal(Horizontal::Right)
        .margin(Margin {
            top: 0,
            right: margin,
            bottom: margin,
            left: 0,
        })
        .click_through()
        .child(Column::new(children![
            Text::new("Activate Linux")
                .size(rem * 0.03)
                .color(Color::rgba(250, 250, 250, 100)),
            Text::new("Go to Settings to activate Linux.")
                .size(rem * 0.02)
                .color(Color::rgba(250, 250, 250, 100)),
        ]))
}

fn bar(monitor: &Monitor) -> LayerWindow {
    let rem = monitor.width.min(monitor.height) as f32;
    let margin_top = (rem * 0.01) as i32;
    let margin_side = (rem * 0.02) as i32;

    let padding_v = rem * 0.005;
    let padding_h = rem * 0.02;

    let battery_string = battery_string();

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
                .fill(Color::rgb(119, 119, 119))
                .shadow(Shadow::inner("#000000").opacity(0.15).blur(8.0))
                .child(Row::new(children![
                    section(Text::new(format!("Workspaces"))
                        .size(rem * 0.02)
                        .color(Color::rgb(221, 221, 221)), Start),
                    section(Text::new(format!("Time"))
                        .size(rem * 0.02)
                        .color(Color::rgb(221, 221, 221)), Center),
                    section(Text::new(battery_string)
                        .size(rem * 0.02)
                        .color(Color::rgb(221, 221, 221)), End)
                ])),
        )
}

fn battery_string() -> String {
    if Battery::read().present() {
        let battery = Battery::read().percent();
        let is_charging = Battery::read().charging();

        let battery_icon = if is_charging {
            if battery < 5 {"󰢟"}
            else if battery < 10 {"󰢜"}
            else if battery < 20 {"󰂆"}
            else if battery < 30 {"󰂇"}
            else if battery < 40 {"󰂈"}
            else if battery < 50 {"󰢝"}
            else if battery < 60 {"󰂉"}
            else if battery < 70 {"󰢞"}
            else if battery < 80 {"󰂊"}
            else if battery < 90 {"󰂋"}
            else {"󰂅"}
        } else {
            if battery < 5 {"󱃍"}
            else if battery < 10 {"󰁺"}
            else if battery < 20 {"󰁻"}
            else if battery < 30 {"󰁼"}
            else if battery < 40 {"󰁽"}
            else if battery < 50 {"󰁾"}
            else if battery < 60 {"󰁿"}
            else if battery < 70 {"󰂀"}
            else if battery < 80 {"󰂁"}
            else if battery < 90 {"󰂂"}
            else {"󰁹"}
        };

        format!("{battery_icon}  {battery}%")
    } else {
        "No battery found".to_string()
    }
}

fn section(content: Text, align: impl Into<Align>) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .align_child(align, Center)
        .child(content)
}
