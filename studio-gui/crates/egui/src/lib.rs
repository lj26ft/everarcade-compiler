#[derive(Default)]
pub struct Context;

#[derive(Default)]
pub struct Ui;

impl Ui {
    pub fn heading(&mut self, _text: impl ToString) {}
    pub fn label(&mut self, _text: impl ToString) {}
    pub fn separator(&mut self) {}
    pub fn button(&mut self, _text: impl ToString) -> Response {
        Response { clicked: false }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Response {
    clicked: bool,
}
impl Response {
    pub fn clicked(&self) -> bool {
        self.clicked
    }
}

pub struct CentralPanel;
impl Default for CentralPanel {
    fn default() -> Self {
        Self
    }
}
impl CentralPanel {
    pub fn show(self, _ctx: &Context, add_contents: impl FnOnce(&mut Ui)) {
        let mut ui = Ui::default();
        add_contents(&mut ui);
    }
}

pub struct SidePanel;
impl SidePanel {
    pub fn left(_id: impl ToString) -> Self {
        Self
    }
    pub fn right(_id: impl ToString) -> Self {
        Self
    }
    pub fn show(self, _ctx: &Context, add_contents: impl FnOnce(&mut Ui)) {
        let mut ui = Ui::default();
        add_contents(&mut ui);
    }
}

pub struct TopBottomPanel;
impl TopBottomPanel {
    pub fn top(_id: impl ToString) -> Self {
        Self
    }
    pub fn bottom(_id: impl ToString) -> Self {
        Self
    }
    pub fn show(self, _ctx: &Context, add_contents: impl FnOnce(&mut Ui)) {
        let mut ui = Ui::default();
        add_contents(&mut ui);
    }
}

pub mod style {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Visuals;
}
