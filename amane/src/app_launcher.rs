use std::time::Duration;

use amane::{
    Apps, Color, Column, Image, Keyboard, Layer, LayerWindow, Rectangle, Row, ScrollArea, Service,
    Size::Parent, Text, TextInput, Widget, children,
};

pub fn app_launcher() -> LayerWindow {
    let launcher = AppLauncher::read();

    let apps = Apps::read();

    let mut rows: Vec<Box<dyn Widget>> = Vec::new();

    for app in apps.list() {
        let query = &AppSearch::read().query;

        if !app.name().to_lowercase().contains(query) {
            continue;
        }

        let launched = app.clone();

        let icon = Rectangle::new().width(24.).height(24.);

        let icon = match app.icon_path() {
            Some(path) => icon.fill(Image::contain(path)),
            None => icon,
        };

        let row = Rectangle::new()
            .width(300.)
            .height(32.)
            .cursor(amane::Cursor::Pointer)
            .on_click(move |_| {
                toggle(&["".to_string()]);
                launched.launch();
            })
            .child(Row::new(children![
                icon,
                Text::new(app.name())
                    .size(16.)
                    .color(Color::rgb(221, 221, 221)),
            ]));

        rows.push(Box::new(row));
    }

    LayerWindow::new()
        .width(300.)
        .height(500.)
        .layer(Layer::Overlay)
        .visible(launcher.open)
        .keyboard(Keyboard::Exclusive)
        .on_key(|_| {
            AppLauncher::write().open = false;
        })
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill("#333333")
                .child(Column::new(children![
                    Rectangle::new()
                        .width(300.0)
                        .height(40.0)
                        .fill("#222222")
                        .padding(8.0)
                        .child(
                            TextInput::new("app_name")
                                .placeholder("App Name")
                                .color(Color::rgb(221, 221, 221))
                                .focused()
                                .on_change(|txt| {
                                    AppSearch::write().query = txt;
                                })
                                .on_submit(|_| {
                                    AppLauncher::write().open = false;
                                })
                        ),
                    ScrollArea::new("apps", Column::new(rows)),
                ])),
        )
}

struct AppSearch {
    query: String,
}

impl Service for AppSearch {
    fn new() -> Self {
        Self {
            query: String::new(),
        }
    }

    fn interval() -> Duration {
        Duration::from_secs(3600)
    }
}

struct AppLauncher {
    open: bool,
}

impl Service for AppLauncher {
    fn new() -> Self {
        Self { open: false }
    }

    fn interval() -> Duration {
        Duration::from_secs(1)
    }
}

pub fn toggle(_: &[String]) -> String {
    let mut launcher = AppLauncher::write();

    launcher.open = !launcher.open;

    if launcher.open {
        String::from("opened")
    } else {
        String::from("closed")
    }
}
