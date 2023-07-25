// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of a custom drawing widget.
//! We draw an image, some text, a shape, and a curve.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use druid::FileDialogOptions;
use druid::FileSpec;
use druid::WidgetExt;
use druid::widget::Align;
use druid::widget::Button;
use druid::widget::TextBox;
use druid::widget::{prelude::*, Flex, Slider};
use druid::{ AppLauncher, Color, LocalizedString, Point,
    WindowDesc, Lens
};

use druid::piet::kurbo::Line;
use kurbo::Vec2;

#[derive(Clone, Data, Lens)]
struct Grid {
    offset: Vec2,
    last_grab_point: Point,
}

impl Grid {
    pub fn new() -> Self {
        Grid {offset: Vec2::default(), last_grab_point: Point::default()}
    }
}

#[derive(Clone, Lens, Data)]
struct AppData {
    square_side: f64,
    count: String,
    grid_scale: GridScale,
}

#[derive(Clone, Data)]
struct GridScale {
    old: f64,
    new: f64,
}

impl GridScale {
    pub fn default() -> Self {
        GridScale { old: 1.0, new: 1.0 }
    }
}

fn min(a:f64, b:f64) -> f64 {
    if a > b {
        return b
    }
    a
}
fn max(a:f64, b:f64) -> f64 {
    if a > b {
        return a
    }
    b
}

impl Widget<AppData> for Grid {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::MouseDown(mouse) => {
                ctx.set_active(true);
                self.last_grab_point = mouse.pos;
                // dbg!(mouse);
                ctx.request_paint();
            },
            Event::MouseUp(_mouse) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
                //dbg!(mouse);
            },
            Event::MouseMove(mouse) => {
                if ctx.is_active() {
                    self.offset.x += mouse.pos.x - self.last_grab_point.x;
                    self.offset.y += mouse.pos.y - self.last_grab_point.y; 
                    self.last_grab_point = mouse.pos;
                    ctx.request_paint();
                }
            },
            Event::Wheel(mouse) => {
                println!("{}", 12312.2%23.1);
                data.grid_scale.old = data.grid_scale.new;


                if mouse.wheel_delta.y > 0.0 {
                    data.grid_scale.new = min(max(0.2,data.grid_scale.new - 0.1),2.0);//rauszoome
                    self.offset.x = mouse.pos.x + (data.grid_scale.new / data.grid_scale.old) * (self.offset.x - mouse.pos.x); //Neuer Vektor Maus-Grid
                    self.offset.y = mouse.pos.y + (data.grid_scale.new / data.grid_scale.old) * (self.offset.y - mouse.pos.y);
                } else {
                    data.grid_scale.new = min(max(0.2,data.grid_scale.new + 0.1),2.0);//reinzoomen
                    self.offset.x = mouse.pos.x + (data.grid_scale.new / data.grid_scale.old) * (self.offset.x - mouse.pos.x); //Neuer Vektor Maus-Grid
                    self.offset.y = mouse.pos.y + (data.grid_scale.new / data.grid_scale.old) * (self.offset.y - mouse.pos.y);
                }
                ctx.request_paint()
            },
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppData, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {
        if _old_data.square_side != _data.square_side {
            _ctx.request_paint()
        }
    }

    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &AppData, _env: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {

        //let size = ctx.size();
        let stroke_color = Color::rgb8(255, 255, 255);
        let mut count = 1;
        if data.count != "".to_string() {
            count = data.count.parse().unwrap();
            if count <= 1 {
                count = 1
            }
        }

        for i in 0..count+1 { //vertikal
            ctx.stroke(Line::new(
                Point::new(
                    i as f64*data.square_side*data.grid_scale.new + self.offset.x,
                    0.0 + self.offset.y
                ),
                Point::new(
                    i as f64*data.square_side*data.grid_scale.new + self.offset.x,
                    count as f64*data.square_side*data.grid_scale.new + self.offset.y
                )
            ), &stroke_color, 1.0);  
        }
        for i in 0..count+1 { //horizontal
            ctx.stroke(Line::new(
                Point::new(
                    0.0 + self.offset.x, 
                    i as f64*data.square_side*data.grid_scale.new + self.offset.y
                ),
                Point::new(
                    count as f64*data.square_side*data.grid_scale.new + self.offset.x, 
                    i as f64*data.square_side*data.grid_scale.new + self.offset.y
                )
            ), &stroke_color, 1.0);  
        }
    }
}

#[derive(Clone, Data, Lens)]
struct InfGrid {
    offset: Vec2,
    last_grab_point: Point,
}

impl InfGrid {
    pub fn new() -> Self {
        InfGrid {offset: Vec2::default(), last_grab_point: Point::default()}
    }
}

impl Widget<AppData> for InfGrid {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::MouseDown(mouse) => {
                ctx.set_active(true);
                self.last_grab_point = mouse.pos;
                // dbg!(mouse);
                ctx.request_paint();
            },
            Event::MouseUp(_mouse) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
                //dbg!(mouse);
            },
            Event::MouseMove(mouse) => {
                if ctx.is_active() {
                    self.offset.x += (mouse.pos.x - self.last_grab_point.x);
                    self.offset.y += (mouse.pos.y - self.last_grab_point.y); 
                    self.last_grab_point = mouse.pos;
                    ctx.request_paint();
                }
            },
            Event::Wheel(mouse) => {
                dbg!(mouse);
                println!("{}", data.grid_scale.old);
                data.grid_scale.old = data.grid_scale.new;

                if mouse.wheel_delta.y > 0.0 {
                    data.grid_scale.new = min(max(0.2,data.grid_scale.new - 0.1),2.0);//rauszoomen
                    self.offset.x = mouse.pos.x + (data.grid_scale.new / data.grid_scale.old) * (self.offset.x - mouse.pos.x); //Neuer Vektor Maus-Grid
                    self.offset.y = mouse.pos.y + (data.grid_scale.new / data.grid_scale.old) * (self.offset.y - mouse.pos.y);
                } else {
                    data.grid_scale.new = min(max(0.2,data.grid_scale.new + 0.1),2.0);//reinzoomen
                    self.offset.x = mouse.pos.x + (data.grid_scale.new / data.grid_scale.old) * (self.offset.x - mouse.pos.x); //Neuer Vektor Maus-Grid
                    self.offset.y = mouse.pos.y + (data.grid_scale.new / data.grid_scale.old) * (self.offset.y - mouse.pos.y);
                }

                ctx.request_paint()
            },
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, data: &AppData, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {
        if _old_data.square_side != _data.square_side {
            _ctx.request_paint()
        }
    }

    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &AppData, _env: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {

        let size = ctx.size();
        let stroke_color = Color::rgb8(255, 255, 255);
        let mut count = 1;

        if data.count != "".to_string() {
            count = data.count.parse().unwrap();
            if count <= 1 {
                count = 1
            }
        }

        for i in 0..(size.height/(data.square_side*data.grid_scale.new)+2.0) as i32{ //vertikal
            ctx.stroke(Line::new(
                Point::new(
                    (i as f64*data.square_side*data.grid_scale.new + self.offset.x % data.square_side*data.grid_scale.new),
                    (0.0 + self.offset.y % data.square_side*data.grid_scale.new)
                ),
                Point::new(
                    (i as f64*data.square_side*data.grid_scale.new + self.offset.x % data.square_side*data.grid_scale.new),
                    (count as f64*data.square_side*data.grid_scale.new + self.offset.y % data.square_side*data.grid_scale.new)
                )
            ), &stroke_color, 1.0);  
        }
        for i in 0..(size.width/(data.square_side*data.grid_scale.new)+2.0) as i32{ //horizontal
            ctx.stroke(Line::new(
                Point::new(
                    (0.0 + self.offset.x % data.square_side*data.grid_scale.new), 
                    (i as f64*data.square_side*data.grid_scale.new + self.offset.y % data.square_side*data.grid_scale.new)
                ),
                Point::new(
                    (count as f64*data.square_side*data.grid_scale.new + self.offset.x % data.square_side*data.grid_scale.new), 
                    (i as f64*data.square_side*data.grid_scale.new + self.offset.y % data.square_side*data.grid_scale.new)
                )
            ), &stroke_color, 1.0);  
        }
    }
}

pub fn main() {
    let window = WindowDesc::new(build_root_widget())
    .title(LocalizedString::new("Fancy Colors"));

    let initial_state = AppData {
        square_side: 50.0,
        count: "50".to_string(),
        grid_scale: GridScale::default(),
    };

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(initial_state)
        .expect("launch failed");
}

fn build_root_widget() -> impl Widget<AppData> {
    let grid = InfGrid::new();
    // let button = Button::new("Make config window")
    //     .on_click(|ctx: &mut EventCtx, data: &mut AppData, env: &Env| {
    //         ctx.new_sub_window(
    //             WindowConfig::default()
    //                 .show_titlebar(false)
    //                 .window_size(Size::new(100., 100.)),
    //             Slider::new(),
    //             data.clone(),
    //             env.clone(),
    //         );
    //     })
    //     .center();
    let slider = Slider::new()
        .with_range(1.0, 100.0)
        .lens(AppData::square_side);

    let text = TextBox::new()
        .lens(AppData::count);

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
        .with_flex_child(grid, 300.0)
        .with_child(slider)
        .with_child(text)
        .with_child(save)
        .with_child(open);
    Align::centered(layout)
}