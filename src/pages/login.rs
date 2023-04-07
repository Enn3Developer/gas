use crate::{NMessage, NPage};
use eframe::{App, Frame};
use egui::{CentralPanel, Context};
use flume::Sender;

pub struct Login {
    tx: Sender<NMessage>,
}

impl Login {
    pub fn new(tx: Sender<NMessage>) -> Self {
        Self { tx }
    }
}

impl App for Login {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Login");
            if ui.button("Home").clicked() {
                self.tx
                    .send(NMessage::Redirect(NPage::Index))
                    .unwrap_or_default();
            }
        });
    }
}
