#![windows_subsystem = "windows"]

use druid::FileDialogOptions;
use druid::FileSpec;
use druid::widget::Align;
use druid::widget::Button;
use druid::widget::{prelude::*, Flex};
use druid::{ AppLauncher, LocalizedString,
    WindowDesc};


mod custom_widgets;

pub use crate::custom_widgets::widgets::{Grid, AppData, InfGrid, 
    GridScale, TabHandle, TabList, };

pub fn main() {
    let window = WindowDesc::new(build_root_widget())
    .title(LocalizedString::new("Fancy Colors"))
    .show_titlebar(false);

    let initial_state = AppData {
        square_side: 50.0,
        count: "50".to_string(),
        tab_data: TabList::new(),
    };

    AppLauncher::with_window(window)
        // .log_to_console()
        .launch(initial_state)
        .expect("launch failed");
}

fn build_root_widget() -> impl Widget<AppData> {
    let grid = InfGrid::new();
   
    let tab = TabHandle::new();//.controller(WindowController{});

    // let slider = Slider::new()
    //     .with_range(1.0, 100.0)
    //     // .lens(AppData::square_side);

    // let text = TextBox::new()
    //     // .lens(AppData::count);

    let other = FileSpec::new("Image files", &["webp", "jpg", "png", "bmp"]);
    // The options can also be generated at runtime,
    // so to show that off we create a String for the default save name.
    let default_save_name = String::from("MyFile.txt");
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![other])
        .default_type(other)
        .default_name(default_save_name)
        .name_label("Target")
        .title("Choose a target for this lovely file")
        .button_text("Export");
    let open_dialog_options = save_dialog_options
        .clone()
        .default_name("MySavedFile.txt")
        .name_label("Source")
        .title("Where did you put that file?")
        .button_text("Import");
    let save = Button::new("Save").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()))
    });
    let open = Button::new("Open").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
    });

    let layout = Flex::column()
        .with_flex_child(tab, 15.0)
        .with_flex_child(grid, 300.0)
        // .with_child(slider)
        // .with_child(text)
        .with_child(save)
        .with_child(open);
    Align::centered(layout)
}