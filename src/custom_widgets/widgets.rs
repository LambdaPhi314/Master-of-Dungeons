use druid::{Widget, Lens, Data, EventCtx, Env, LifeCycleCtx, 
    LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, PaintCtx, 
    Color, RenderContext, Event, Rect, widget::{Label, Axis}, WindowState};
use im::Vector;
use kurbo::{Point, Vec2, Line, Size};
use crate::custom_widgets::tabs::{TabInfo, TabsEdge, TabsPolicy, TabsTransition, };
// 
// pub const WINDOW_CLOSE_IMAGE: Image = Image::new(ImageBuf::from_data(include_bytes!("./assets/PicWithAlpha.png")).unwrap());
// const WINDOW_CLOSE_IMAGE: Image = 
// const WINDOW_CLOSE_IMAGE_POS: Rect = Rect::new(x0, y0, x1, y1);


#[derive(Debug, Clone, Data, PartialEq)]
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

#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub square_side: f64,
    pub count: String,
    pub advanced: DynamicTabData,
    pub tab_config: TabConfig,
}

#[derive(Data, Clone, Lens)]
pub struct TabConfig {
    pub axis: Axis,
    pub edge: TabsEdge,
    pub transition: TabsTransition,
}

#[derive(Debug, Clone, Data, PartialEq)]
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
        for i in 0..(size.height/(data.square_side*self.grid_scale.new)+1.0) as i32{ //horizontal
            if i == (self.offset.y/(data.square_side*self.grid_scale.new)) as i32 && i != 0 {
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
pub struct NumberedTabs;

impl TabsPolicy for NumberedTabs {
    type Key = usize;
    type Build = ();
    type Input = DynamicTabData;
    type LabelWidget = Label<DynamicTabData>;
    type BodyWidget = Label<DynamicTabData>;

    fn tabs_changed(&self, old_data: &DynamicTabData, data: &DynamicTabData) -> bool {
        old_data.tabs_key() != data.tabs_key()
    }

    fn tabs(&self, data: &DynamicTabData) -> Vec<Self::Key> {
        data.tab_labels.iter().copied().collect()
    }

    fn tab_info(&self, key: Self::Key, _data: &DynamicTabData) -> TabInfo<DynamicTabData> {
        TabInfo::new(format!("Tab {key:?}"), true)
    }

    fn tab_body(&self, key: Self::Key, _data: &DynamicTabData) -> Label<DynamicTabData> {
        Label::new(format!("Dynamic tab body {key:?}"))
    }

    fn close_tab(&self, key: Self::Key, data: &mut DynamicTabData) {
        if let Some(idx) = data.tab_labels.index_of(&key) {
            data.remove_tab(idx)
        }
    }

    fn tab_label(
        &self,
        _key: Self::Key,
        info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        Self::default_make_label(info)
    }
}

#[derive(Data, Clone, Lens)]
pub struct DynamicTabData {
    highest_tab: usize,
    removed_tabs: usize,
    tab_labels: Vector<usize>,
}

impl DynamicTabData {
    pub fn new(highest_tab: usize) -> Self {
        DynamicTabData {
            highest_tab,
            removed_tabs: 0,
            tab_labels: (1..=highest_tab).collect(),
        }
    }

    pub fn add_tab(&mut self) {
        self.highest_tab += 1;
        self.tab_labels.push_back(self.highest_tab);
    }

    pub fn remove_tab(&mut self, idx: usize) {
        if idx >= self.tab_labels.len() {
            tracing::warn!("Attempt to remove non existent tab at index {}", idx)
        } else {
            self.removed_tabs += 1;
            self.tab_labels.remove(idx);
        }
    }

    // This provides a key that will monotonically increase as interactions occur.
    pub fn tabs_key(&self) -> (usize, usize) {
        (self.highest_tab, self.removed_tabs)
    }
}

pub enum WindowActions {
    Minimize,
    Resize,
    Close
}

pub struct WindowHandleButton {
    action: WindowActions,
}

impl WindowHandleButton {
    pub fn new( action: WindowActions ) -> Self {
        WindowHandleButton { action: action }
    }
}

impl Widget<AppData> for WindowHandleButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut AppData, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                let mut win = ctx.window().clone();
                match self.action {
                    WindowActions::Minimize => {
                        win.set_window_state(WindowState::Minimized)
                    }
                    WindowActions::Resize => {
                        if win.get_window_state() == druid::WindowState::Restored {
                            win.set_window_state(druid::WindowState::Maximized)
                        } else {
                            win.set_window_state(druid::WindowState::Restored);
                        }
                    }
                    WindowActions::Close => {
                        win.close()
                    }
                }
            }
        _=> {}
        }
    }
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {
        
    }
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppData, _env: &Env) {
        
    }
    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &AppData, _env: &Env) -> Size {
        bc.max()
    }
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &AppData, _env: &Env) {
        let size = ctx.size();
        match self.action {
            WindowActions::Minimize => {
                ctx.fill(Rect::new(0.0, 0.0, size.width, size.height), &Color::rgb8(51, 153, 255))
            }
            WindowActions::Resize => {
                ctx.fill(Rect::new(0.0, 0.0, size.width, size.height), &Color::rgb8(0, 255, 0))
            }
            WindowActions::Close => {
                ctx.fill(Rect::new(0.0, 0.0, size.width, size.height), &Color::rgb8(255, 0, 0))
            }
        }
        
    }
}