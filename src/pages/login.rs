use crate::pages::common::{side_panel, top_panel};
use crate::{NMessage, NPage};
use eframe::{App, Frame};
use egui::{CentralPanel, Context};
use flume::Sender;

pub struct Login {
    tx: Sender<NMessage>,
    show_panel: bool,
    user: String,
    pass: String,
}

impl Login {
    pub fn new(tx: Sender<NMessage>) -> Self {
        Self {
            tx,
            show_panel: false,
            user: String::new(),
            pass: String::new(),
        }
    }
}

impl App for Login {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        side_panel(ctx, self.tx.clone(), self.show_panel);

        CentralPanel::default().show(ctx, |ui| {
            top_panel(ui, "Login", &mut self.show_panel);
            ui.add_space(10.0);

            ui.label("Username:");
            ui.text_edit_singleline(&mut self.user);
            ui.add_space(10.0);
            ui.label("Password:");
            ui.text_edit_singleline(&mut self.pass);
            ui.add_space(10.0);
            if ui.button("Login").clicked() {
                // TODO: login
            }
        });
    }
}
