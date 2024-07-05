// use fltk::{
//     enums::{Align, Color, Font, FrameType},
//     prelude::*,
//     *,
// };

// const BLUE: Color = Color::from_hex(0x42A5F5);
// const SEL_BLUE: Color = Color::from_hex(0x2196F3);
// const GRAY: Color = Color::from_hex(0x757575);
// const WIDTH: i32 = 600;
// const HEIGHT: i32 = 400;

// fn main() {
//     let app = app::App::default();
//     let mut win = window::Window::default()
//         .with_size(WIDTH, HEIGHT)
//         .with_label("Flutter-like!");
//     let mut bar =
//         frame::Frame::new(0, 0, WIDTH, 60, "  FLTK App!").with_align(Align::Left | Align::Inside);
//     let mut text = frame::Frame::default()
//         .with_size(100, 40)
//         .center_of(&win)
//         .with_label("You have pushed the button this many times:");
//     let mut count = frame::Frame::default()
//         .size_of(&text)
//         .below_of(&text, 0)
//         .with_label("0");
//     let mut but = button::Button::new(WIDTH - 100, HEIGHT - 100, 60, 60, "@+6plus");
//     win.end();
//     win.make_resizable(true);
//     win.show();

//     // Theming
//     app::background(255, 255, 255);
//     app::set_visible_focus(false);
//     app.load_system_fonts();

//     bar.set_frame(FrameType::FlatBox);
//     bar.set_label_size(22);
//     bar.set_label_color(Color::White);
//     bar.set_color(BLUE);
//     bar.draw(|b| {
//         draw::set_draw_rgb_color(211, 211, 211);
//         draw::draw_rectf(0, b.height(), b.width(), 3);
//     });

//     text.set_label_size(18);
//     text.set_label_font(Font::Screen);

//     count.set_label_size(36);
//     count.set_label_color(GRAY);

//     but.set_color(BLUE);
//     but.set_selection_color(SEL_BLUE);
//     but.set_label_color(Color::White);
//     but.set_frame(FrameType::GleamUpFrame);
//     // End theming

//     but.set_callback(move |_| {
//         let label = (count.label().parse::<i32>().unwrap() + 1).to_string();
//         count.set_label(&label);
//     });

//     app.run().unwrap();
// }

// use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};
// use fltk_theme::{color_themes, ColorTheme, SchemeType, ThemeType, WidgetScheme, WidgetTheme};
// mod myuifile;

// #[macro_use]
// extern crate tr;

// #[derive(Clone, Copy)]
// enum Message {
//     Increment,
//     Decrement,
// }

// fn main() {
//     let app = app::App::default();

//     // let theme = ColorTheme::new(color_themes::DARK_THEME);
//     // theme.apply();
//     // let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
//     // widget_scheme.apply();
//     let widget_theme = WidgetTheme::new(ThemeType::Metro);
//     widget_theme.apply();
//     app::own_colormap();
//     app::get_system_colors();

//     let mut ui = myuifile::UserInterface::make_window();

//     let mut wind = Window::default().with_size(400, 400).with_label("Counter");
//     wind.make_resizable(true);
//     let mut flex = Flex::default()
//         .with_size(300, 300)
//         .center_of_parent()
//         .column();
//     let mut but_inc = Button::default().with_label("+");
//     let mut frame = Frame::default().with_label("0");
//     let mut but_dec = Button::default().with_label("-");
//     flex.end();
//     wind.end();
//     wind.show();

//     let (s, r) = app::channel::<Message>();

//     but_inc.emit(s, Message::Increment);
//     but_dec.emit(s, Message::Decrement);

//     // app.run().unwrap();

//     while app.wait() {
//         let label: i32 = frame.label().parse().unwrap();
//         if let Some(msg) = r.recv() {
//             match msg {
//                 Message::Increment => frame.set_label(&(label + 1).to_string()),
//                 Message::Decrement => frame.set_label(&(label - 1).to_string()),
//             }
//         }
//     }
// }

// use std::thread;

// use fltk::{
//     app, button::Button, dialog::FileDialog, enums::Align, frame::Frame, group::Flex, group::Pack, image::SvgImage, prelude::*, window::Window,
// };

// fn main() {
//     // Create an FLTK application
//     let app = app::App::default();

//     // Create a window to contain the dialog
//     let mut wind = Window::new(100, 100, 400, 300, "Custom Dialog");

//     // Use a flex box for the main layout
//     let mut flex = Flex::new(0, 0, 400, 300, "").column();

//     // Main instruction
//     let mut main_instr = Frame::new(0, 0, 400, 40, "Main Instruction Text");
//     main_instr.set_label_size(24);
//     main_instr.set_align(Align::Center | Align::Inside);

//     // Body text
//     let mut body_text = Frame::new(0, 0, 400, 60, "This is the body text that provides additional details.");
//     body_text.set_align(Align::Wrap);

//     // SVG Icon
//     let icon_data = include_str!("icon.svg");
//     let mut icon_frame = Frame::new(0, 0, 400, 100, "");
//     if let Ok(mut svg_img) = SvgImage::from_data(icon_data) {
//         svg_img.scale(64, 64, true, true);
//         icon_frame.set_image(Some(svg_img));
//     }

//     // Buttons at the bottom
//     let mut button_pack = Pack::new(0, 0, 400, 40, "");
//     button_pack.set_spacing(10);

//     let mut ok_button = Button::new(0, 0, 80, 40, "OK");
//     let mut cancel_button = Button::new(0, 0, 80, 40, "Cancel");
//     button_pack.end();

//     flex.end();
//     wind.end();
//     wind.show();

//     // Set button callbacks
//     ok_button.set_callback(move |_| {
//         println!("OK button pressed");
//         app.quit();
//     });

//     cancel_button.set_callback(move |_| {
//         println!("Cancel button pressed");
//         app.quit();
//     });

//     // Run the FLTK application
//     app.run().unwrap();

// }

mod fltk_dialog;

use std::{sync::mpsc, thread, time::Duration};

use fltk::{
    app, button::Button, dialog::FileDialog, enums::Align, frame::Frame, group::Flex, group::Pack,
    image::SvgImage, prelude::*, window::Window,
};
use fltk_theme::{ThemeType, WidgetTheme};

fn main() {
    // we need to create the fltk_dialog::run method, it should
    fltk_dialog_run(real_main);
}

#[derive(Debug, Clone)]
struct DialogRequest {
    pub id: u32,
    pub title: String,
    pub message: String,
    pub buttons: Vec<String>,
}

fn create_messagebox() {
    let mut wind = Window::new(100, 100, 400, 300, "Custom Dialog");

    // Use a flex box for the main layout
    let mut flex = Flex::new(0, 0, 400, 300, "").column();

    // Main instruction
    let mut main_instr = Frame::new(0, 0, 400, 40, "Main Instruction Text");
    main_instr.set_label_size(24);
    main_instr.set_align(Align::Center | Align::Inside);

    // Body text
    let mut body_text = Frame::new(
        0,
        0,
        400,
        60,
        "This is the body text that provides additional details.",
    );
    body_text.set_align(Align::Wrap);

    // SVG Icon
    let icon_data = include_str!("icon.svg");
    let mut icon_frame = Frame::new(0, 0, 400, 100, "");
    if let Ok(mut svg_img) = SvgImage::from_data(icon_data) {
        svg_img.scale(64, 64, true, true);
        icon_frame.set_image(Some(svg_img));
    }

    // Buttons at the bottom
    let mut button_pack = Pack::new(0, 0, 400, 40, "");
    button_pack.set_spacing(10);

    let mut ok_button = Button::new(0, 0, 80, 40, "OK");
    let mut cancel_button = Button::new(0, 0, 80, 40, "Cancel");
    button_pack.end();

    flex.end();
    wind.end();
    wind.show();
}

fn fltk_dialog_run(main: fn(dlg: DialogContext) -> ()) {
    #[cfg(windows)]
    {
        app::App::default();
        let widget_theme = WidgetTheme::new(ThemeType::Metro);
        widget_theme.apply();
    }

    #[cfg(target_os = "linux")]
    app::App::default().with_scheme(app::Scheme::Gtk);

    let (send_message, receive_message) = mpsc::channel::<DialogRequest>();
    let (send_result, receive_result) = mpsc::channel::<u32>();
    let (send_progress, receive_process) = mpsc::channel::<(u32, u32)>();

    let t = thread::spawn(move || {
        let context = DialogContext::new(send_message, receive_result);
        main(context);
    });


    loop {
        if let Err(e) = app::wait_for(0.1) {
            println!("Error: {:?}", e);
            break;
        }

        if t.is_finished() {
            app::quit();
            break;
        }

        if let Ok(message) = receive_message.try_recv() {
            create_messagebox();
        }
    }
}

struct DialogContext {
    id: u32,
    sender: mpsc::Sender<DialogRequest>,
    receiver: mpsc::Receiver<u32>,
}

impl DialogContext {
    pub fn new(sender: mpsc::Sender<DialogRequest>, receiver: mpsc::Receiver<u32>) -> Self {
        Self { sender, receiver, id: 0 }
    }
    pub fn messagebox_info_ok(&mut self, title: &str, message: &str) {
        self.id = self.id + 1;
        self.sender
            .send(DialogRequest {
                id: self.id,
                title: title.to_string(),
                message: message.to_string(),
                buttons: vec!["OK".to_string()],
            })
            .unwrap();
    }
}

fn real_main(mut dlg: DialogContext) {
    println!("test...");
    thread::sleep(Duration::from_secs(2));
    dlg.messagebox_info_ok("Title", "Message");
    thread::sleep(Duration::from_secs(5));
    dlg.messagebox_info_ok("Title2", "Message2");
    thread::sleep(Duration::from_secs(5));
}
