use amane::{
    Center, Color, Column, Cursor::Pointer, Image, LayerWindow, Lock, Monitor, Rectangle, Row,
    Service, Size::Parent, Text, TextInput, children,
};

use crate::palette::{BLACK, GRAY, RED, WHITE};

pub fn lock_screen(_: &Monitor) -> LayerWindow {
    let lock = Lock::read();

    let status = if lock.checking() {
        "[ CHECKING ] "
    } else if lock.failed() {
        "[ WRONG PASSWORD :( ]"
    } else {
        "[ LOCKED ]"
    };

    let text_color = if lock.failed() { RED } else { BLACK };

    LayerWindow::new().width(1.).height(1.).child(
        Rectangle::new()
            .width(Parent)
            .height(Parent)
            .fill(
                Image::cover("/home/drpepper/Pictures/Wallpapers/lagtrain.jpg")
                    .thumbnail(64, 36)
                    .blurred(2),
            )
            .align_child(Center, Center)
            .child(
                Column::new(children![
                    Text::new(status).size(24.).color(text_color),
                    Rectangle::new()
                        .width(500.)
                        .height(40.)
                        .fill(GRAY)
                        .radius(5.)
                        .padding(8.)
                        .border(1., BLACK)
                        .child(
                            TextInput::new("password")
                                .placeholder("Password")
                                .password()
                                .focused()
                                .color(BLACK)
                                .on_submit(|password| Lock::unlock(&password)),
                        ),
                    Rectangle::new().width(500.).height(40.).child(
                        Row::new(children![
                            button("[ SUSPEND ]", "systemctl suspend", WHITE),
                            button("[ REBOOT ]", "reboot", RED),
                            button("[ SHUTDOWN ]", "poweroff", RED)
                        ])
                        .gap(5.)
                    ),
                ])
                .gap(12.),
            ),
    )
}

pub fn lock(_: &[String]) -> String {
    Lock::start();

    String::from("locked")
}

fn button(string: &'static str, command: &'static str, color: Color) -> Rectangle {
    Rectangle::new()
        .width(Parent)
        .height(Parent)
        .cursor(Pointer)
        .radius(5.)
        .align_child(Center, Center)
        .fill(BLACK)
        .on_click(|_| amane::spawn(command))
        .child(Text::new(string).color(color))
}
