use std::time::Duration;

use amane::{
    Align::{Center, Start},
    Apps, Color, Column, DesktopApp, Key, Keyboard, Layer, LayerWindow, Rectangle, ScrollArea,
    Service,
    Size::Parent,
    Text, TextInput, Widget, children,
};

pub fn app_launcher() -> LayerWindow {
    let launcher = AppLauncher::read();

    let apps = &AppLauncher::read().apps;

    let mut rows: Vec<Box<dyn Widget>> = Vec::new();

    let mut idx = 0;

    for app in apps {
        let query = &AppLauncher::read().query;

        if !app.name().to_lowercase().contains(query) {
            continue;
        }


        let launched = app.clone();

        let app_name = if app.name().len() < 15 {
            app.name()
        } else {
            &format!("{}..", &app.name()[..15])
        };

        let color = if idx == launcher.selected {
            Color::rgb(221, 221, 221)
        } else {
            Color::rgb(152, 152, 152)
        };

        let row = Rectangle::new()
            .width(300.)
            .height(13.)
            .cursor(amane::Cursor::Pointer)
            .on_click(move |_| {
                toggle_launcher(&["".to_string()]);
                launched.launch();
            })
            .child(
                Text::new(format!("[ {} ]", app_name))
                    .size(13.)
                    .color(color),
            );

        idx += 1;

        rows.push(Box::new(row));
    }

    let app_launcher_widget = if AppLauncher::read().query != "" {
        Column::new(children![
            Rectangle::new()
                .width(Parent)
                .height(30.)
                .border(1., Color::rgb(221, 221, 221))
                .padding(5.)
                .child(
                    TextInput::new("app_name")
                        .placeholder("App Name")
                        .color(Color::rgb(221, 221, 221))
                        .focused()
                        .on_change(|txt| {
                            let mut launcher = AppLauncher::write();
                            launcher.query = txt;
                            launcher.selected = 0;
                        })
                        .on_submit(launch_selected)
                ),
            ScrollArea::new("apps", Column::new(rows).gap(10.)),
        ])
        .gap(10.)
    } else {
        Column::new(children![
            Rectangle::new().width(1.).height(1.).child(
                TextInput::new("app_name")
                    .on_change(|txt| AppLauncher::write().query = txt)
                    .color(Color::rgb(51, 51, 51))
                    .focused()
            ),
            Text::new("-- WELCOME --").color(Color::rgb(221, 221, 221)),
            Text::new("TYPE ANYTHING").color(Color::rgb(221, 221, 221)),
        ])
    };

    LayerWindow::new()
        .width(200.)
        .height(300.)
        .layer(Layer::Overlay)
        .visible(launcher.open)
        .keyboard(Keyboard::Exclusive)
        .on_key(key_pressed)
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill("#333333")
                .padding(10.)
                .radius(5.)
                .border(1., Color::rgb(221, 221, 221))
                .align_child(Center, Start)
                .child(app_launcher_widget),
        )
}

struct AppLauncher {
    open: bool,
    query: String,
    selected: u32,
    apps: Vec<DesktopApp>,
}

impl Service for AppLauncher {
    fn new() -> Self {
        Self {
            open: false,
            query: String::new(),
            selected: 0,
            apps: Vec::new(),
        }
    }

    fn interval() -> Duration {
        Duration::from_secs(3)
    }

    fn update(&mut self) -> bool {
        let apps = Apps::read().list().to_vec();

        let changed = apps != self.apps;

        self.apps = apps;

        changed
    }
}

pub fn toggle_launcher(_: &[String]) -> String {
    let mut launcher = AppLauncher::write();

    if !launcher.open {
        launcher.query = "".to_string();
        TextInput::set_text("app_name", "");
    }

    launcher.open = !launcher.open;

    if launcher.open {
        String::from("opened")
    } else {
        String::from("closed")
    }
}

fn key_pressed(key: Key) {
    let mut launcher = AppLauncher::write();

    if key == Key::Down {
        launcher.selected += 1;
    } else if key == Key::Up {
        launcher.selected = launcher.selected.saturating_sub(1);
    }
}

fn launch_selected(text: String) {
    let apps = &Apps::read().list().to_vec();
    let selected = AppLauncher::read().selected;
    let mut launcher= AppLauncher::write();
    let mut filtered_apps = Vec::new();

    for app in apps {
        if !app.name().to_lowercase().contains(&text) {
            continue;
        }

        filtered_apps.push(app);
    }

    let idx = selected.min((filtered_apps.len() - 1) as u32);

    launcher.open = false;
    filtered_apps[idx as usize].launch();
}
