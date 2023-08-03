#![windows_subsystem = "windows"]

use custom_widgets::widgets::TabConfig;
use custom_widgets::widgets::WindowHandleButton;
use druid::FileDialogOptions;
use druid::FileSpec;
use druid::UnitPoint;
use druid::widget::Align;
use druid::widget::Axis;
use druid::widget::Button;
use druid::widget::ZStack;
use druid::widget::{prelude::*, Flex};
use druid::{ AppLauncher, LocalizedString,
    WindowDesc};
use druid::WidgetExt;
use kurbo::Vec2;

mod custom_widgets;

pub use crate::custom_widgets::widgets::{Grid, AppData, InfGrid, 
    GridScale, NumberedTabs, DynamicTabData, WindowActions::{Close, Resize, Minimize}, };

pub use crate::custom_widgets::tabs::{Tabs, TabsEdge, };

pub fn main() {
    let window = WindowDesc::new(build_root_widget())
    .title(LocalizedString::new("Fancy Colors"))
    .show_titlebar(false);

    let initial_state = AppData {
        square_side: 50.0,
        count: "50".to_string(),
        advanced: DynamicTabData::new(2),
        tab_config: TabConfig {
            axis: Axis::Horizontal,
            edge: TabsEdge::Leading,
            transition: Default::default(),
        },
    };

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(initial_state)
        .expect("launch failed");
}

fn build_root_widget() -> impl Widget<AppData> {
    // let grid = InfGrid::new();
   
    // let tab = TabHandle::new();
        

    let dyn_tabs = ZStack::new(Tabs::for_policy(NumberedTabs)
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .lens(AppData::advanced))
        .with_child(WindowHandleButton::new(Close), 
        Vec2::new(0.0, 0.0),
        Vec2::new(40.0, 30.0),
        UnitPoint::new(1.0, 0.0),
        Vec2::new(0.0, 0.0))
        .with_child(WindowHandleButton::new(Resize),
        Vec2::new(0.00, 0.0),
        Vec2::new(40.0, 29.0),
        UnitPoint::new(1.0, 0.0),
        Vec2::new(-40.0, 0.0))
        .with_child(WindowHandleButton::new(Minimize),
        Vec2::new(0.0, 0.0),
        Vec2::new(40.0, 29.0),
        UnitPoint::new(1.0, 0.0),
        Vec2::new(-80.0, 0.0));

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

    let add_tab_button = Button::new("Add a tab")
        .on_click(|_c, d: &mut AppData, _e| d.advanced.add_tab());

    // let tab_row = Flex::row().add_flex_child(i for i in TabList)

    let layout = Flex::column()
        .with_flex_child(dyn_tabs, 15.0)
        // .with_flex_child(grid, 300.0)
        // .with_child(slider)
        // .with_child(text)
        .with_child(save)
        .with_child(open)
        .with_child(add_tab_button);
    Align::centered(layout)
}