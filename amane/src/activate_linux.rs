use amane::{
    Color, Column, Horizontal, Layer, LayerWindow, Margin, Monitor, Text, Vertical, children,
};

pub fn activate_linux(monitor: &Monitor) -> LayerWindow {
    LayerWindow::new()
        .width(monitor.width as f32 * 0.2)
        .height(monitor.height as f32 * 0.1)
        .anchor_vertical(Vertical::Bottom)
        .anchor_horizontal(Horizontal::Right)
        .layer(Layer::Top)
        .margin(Margin {
            top: 0,
            right: 30,
            bottom: 30,
            left: 0,
        })
        .click_through()
        .child(Column::new(children![
            Text::new("Activate Linux")
                .size(20.)
                .color(Color::rgba(250, 250, 250, 100)),
            Text::new("Go to Settings to activate Linux.")
                .size(12.)
                .color(Color::rgba(250, 250, 250, 100)),
        ]))
}
