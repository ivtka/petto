use eframe::egui::{CentralPanel, CtxRef, ScrollArea, Vec2};
use eframe::epi::{App, Frame, Storage};
use eframe::{NativeOptions, run_native};

mod petto;

use crate::petto::Petto;

impl App for Petto {
    fn setup(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>, _storage: Option<&dyn Storage>) {
        self.configure_fonts(ctx);
    }
    
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        ctx.request_repaint();

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.render_customer_card(ui);
            });
        });
    }

    fn name(&self) -> &str {
        "Petto"
    }
}

fn main() {
    let app = Petto::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(Box::new(app), win_options);
}
