pub use egui;

#[derive(Default)]
pub struct NativeOptions;

pub struct Frame;

pub struct CreationContext<'a> {
    pub egui_ctx: &'a egui::Context,
}

pub trait App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame);
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type AppCreator = dyn FnOnce(&CreationContext<'_>) -> Box<dyn App>;

pub fn run_native(
    _title: &str,
    _native_options: NativeOptions,
    app_creator: Box<AppCreator>,
) -> Result<(), Error> {
    let ctx = egui::Context::default();
    let cc = CreationContext { egui_ctx: &ctx };
    let mut app = app_creator(&cc);
    let mut frame = Frame;
    app.update(&ctx, &mut frame);
    Ok(())
}
