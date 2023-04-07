use crate::pages::common::{side_panel, top_panel};
use crate::NMessage;
use eframe::{App, Frame};
use egui::{CentralPanel, Context, ScrollArea};
use flume::Sender;

pub struct Index {
    show_panel: bool,
    tx: Sender<NMessage>,
}

impl Index {
    pub fn new(tx: Sender<NMessage>) -> Self {
        Self {
            show_panel: false,
            tx,
        }
    }
}

impl App for Index {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        side_panel(ctx, self.tx.clone(), self.show_panel);

        CentralPanel::default().show(ctx, |ui| {
            top_panel(ui, "Home", &mut self.show_panel);

            // Main page
            ScrollArea::vertical().show(ui, |ui| {
                ui.label("Test");
            });
        });
    }
}
