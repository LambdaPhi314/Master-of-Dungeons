use druid::{Widget, Lens, Data, EventCtx, Env, LifeCycleCtx, 
    LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, PaintCtx, 
    Color, RenderContext, Event, Rect};
use druid::widget::Controller;
use im::Vector;
use kurbo::{Point, Vec2, Line, Size};

#[derive(Clone, Data)]
pub struct Grid {
    offset: Vec2,
    last_grab_point: Point,
    grid_scale: GridScale,
}

impl Grid {
    pub fn new() -> Self {
        Grid {offset: Vec2::default(), last_grab_point: Point::default(), grid_scale: GridScale::default()}
    }
}

#[derive(Clone, Data)]
pub struct AppData {
    pub square_side: f64,
    pub count: String,
    pub tab_data: TabList,
}

#[derive(Clone, Data)]
pub struct GridScale {
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
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut AppData, _env: &Env) {
        match event {
            Event::MouseDown(mouse) => {
                ctx.set_active(true);
                self.last_grab_point = mouse.pos;
                ctx.request_paint();
            },
            Event::MouseUp(_mouse) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
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
                self.grid_scale.old = self.grid_scale.new;
                if mouse.wheel_delta.y > 0.0 {
                    self.grid_scale.new = min(max(0.2,self.grid_scale.new - 0.1),2.0);//rauszoome
                    self.offset.x = mouse.pos.x + (self.grid_scale.new / self.grid_scale.old) * (self.offset.x - mouse.pos.x); //Neuer Vektor Maus-Grid
                    self.offset.y = mouse.pos.y + (self.grid_scale.new / self.grid_scale.old) * (self.offset.y - mouse.pos.y);
                } else {
                    self.grid_scale.new = min(max(0.2,self.grid_scale.new + 0.1),2.0);//reinzoomen
                    self.offset.x = mouse.pos.x + (self.grid_scale.new / self.grid_scale.old) * (self.offset.x - mouse.pos.x); //Neuer Vektor Maus-Grid
                    self.offset.y = mouse.pos.y + (self.grid_scale.new / self.grid_scale.old) * (self.offset.y - mouse.pos.y);
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
                    i as f64*data.square_side*self.grid_scale.new + self.offset.x,
                    0.0 + self.offset.y
                ),
                Point::new(
                    i as f64*data.square_side*self.grid_scale.new + self.offset.x,
                    count as f64*data.square_side*self.grid_scale.new + self.offset.y
                )
            ), &stroke_color, 1.0);  
        }
        for i in 0..count+1 { //horizontal
            ctx.stroke(Line::new(
                Point::new(
                    0.0 + self.offset.x, 
                    i as f64*data.square_side*self.grid_scale.new + self.offset.y
                ),
                Point::new(
                    count as f64*data.square_side*self.grid_scale.new + self.offset.x, 
                    i as f64*data.square_side*self.grid_scale.new + self.offset.y
                )
            ), &stroke_color, 1.0);  
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct InfGrid {
    offset: Vec2,
    last_grab_point: Point,
    grid_scale: GridScale,
}

impl InfGrid {
    pub fn new() -> Self {
        InfGrid {offset: Vec2::default(), last_grab_point: Point::default(), grid_scale: GridScale::default()}
    }
}

impl Widget<AppData> for InfGrid {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut AppData, _env: &Env) {
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
                dbg!(mouse);
                println!("{}", self.grid_scale.old);
                self.grid_scale.old = self.grid_scale.new;

                if mouse.wheel_delta.y > 0.0 {
                    self.grid_scale.new = min(max(0.2,self.grid_scale.new - 0.1),2.0);//rauszoomen
                    self.offset.x = mouse.pos.x + (self.grid_scale.new / self.grid_scale.old) * (self.offset.x - mouse.pos.x); //Neuer Vektor Maus-Grid
                    self.offset.y = mouse.pos.y + (self.grid_scale.new / self.grid_scale.old) * (self.offset.y - mouse.pos.y);
                } else {
                    self.grid_scale.new = min(max(0.2,self.grid_scale.new + 0.1),2.0);//reinzoomen
                    self.offset.x = mouse.pos.x + (self.grid_scale.new / self.grid_scale.old) * (self.offset.x - mouse.pos.x); //Neuer Vektor Maus-Grid
                    self.offset.y = mouse.pos.y + (self.grid_scale.new / self.grid_scale.old) * (self.offset.y - mouse.pos.y);
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

        let size = ctx.size();
        #[allow(unused_assignments)]
        let mut stroke_color = Color::rgb8(255, 255, 255);

        for i in 0..(size.width/(data.square_side*self.grid_scale.new)+2.0) as i32{ //vertikal
            if i == (self.offset.x/(data.square_side*self.grid_scale.new)) as i32 {
                stroke_color = Color::rgb8(255,0,0);
            } else {
                stroke_color = Color::rgb8(255,255,255);
            }
            ctx.stroke(Line::new(
                Point::new(
                    i as f64*data.square_side*self.grid_scale.new + self.offset.x % (data.square_side*self.grid_scale.new),
                    min(0.0 + self.offset.y % (data.square_side*self.grid_scale.new),0.0)
                ),
                Point::new(
                    i as f64*data.square_side*self.grid_scale.new + self.offset.x % (data.square_side*self.grid_scale.new),
                    size.height
                )
            ), &stroke_color, 1.0);  
        }
        for i in 0..(size.height/(data.square_side*self.grid_scale.new)+2.0) as i32{ //horizontal
            if i == (self.offset.y/(data.square_side*self.grid_scale.new)) as i32 {
                stroke_color = Color::rgb8(255,0,0);
            }
            else {
                stroke_color = Color::rgb8(255, 255, 255)
            }
            ctx.stroke(Line::new(
                Point::new(
                    min(0.0 + self.offset.x % (data.square_side*self.grid_scale.new),0.0), 
                    i as f64*data.square_side*self.grid_scale.new + self.offset.y % (data.square_side*self.grid_scale.new)
                ),
                Point::new(
                    size.width, 
                    i as f64*data.square_side*self.grid_scale.new + self.offset.y % (data.square_side*self.grid_scale.new)
                )
            ), &stroke_color, 1.0);  
        }
    }
}

#[derive(Clone, Data)]
pub struct TabHandle {
    tab_list: TabList,
}

impl TabHandle {
    pub fn new() -> Self {
        TabHandle { tab_list: TabList::new() }
    }
}

#[derive(Clone, Data)]
pub struct TabList {
    tabs: Vector<TabContent>,
}

impl TabList {
    pub fn new() -> Self{
        let mut vec = Vector::new();
        vec.push_back(TabContent{layout: Grid::new(), tab_id: 0});
        TabList { tabs: vec }
    } 
}

#[derive(Clone, Data)]
struct TabContent {
    layout: Grid,
    tab_id: u8,
}

impl Widget<AppData> for TabHandle {
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &AppData, _env: &Env) {
        let size = ctx.size();
        ctx.fill(Rect::new(0.0, 0.0, size.width, 30.0), &Color::rgb8(155, 155, 155))
    }
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppData, _env: &Env) {
        
    }
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {
        
    }
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppData, _env: &Env) {
        
    }
    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &AppData, _env: &Env) -> Size {
        bc.max()
    }
}

pub struct WindowController {
}

impl<T, W: Widget<T>> Controller<T, W> for WindowController {
    fn event(&mut self, _child: &mut W, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        if let Event::MouseMove(_) = event {
            ctx.window().handle_titlebar(true)
       }
    }
}

