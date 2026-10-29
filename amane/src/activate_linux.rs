use amane::{
    Color, Column, Horizontal, Layer, LayerWindow, Margin, Text, Vertical, children,
};

pub fn activate_linux() -> LayerWindow {
    LayerWindow::new()
        .width(200.)
        .height(70.)
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
