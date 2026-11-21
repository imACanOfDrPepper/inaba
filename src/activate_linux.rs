use amane::{Color, Column, Horizontal, Layer, LayerWindow, Margin, Text, Vertical, children};

pub fn activate_linux() -> LayerWindow {
    LayerWindow::new()
        .width(300.)
        .height(100.)
        .anchor_vertical(Vertical::Bottom)
        .anchor_horizontal(Horizontal::Right)
        .layer(Layer::Top)
        .margin(Margin {
            top: 0,
            right: 50,
            bottom: 50,
            left: 0,
        })
        .click_through()
        .child(Column::new(children![
            Text::new("Activate Linux")
                .size(30.)
                .color(Color::rgba(250, 250, 250, 100))
                .font("Inter"),
            Text::new("Go to Settings to activate Linux.")
                .size(18.)
                .color(Color::rgba(250, 250, 250, 100))
                .font("Inter"),
        ]))
}
