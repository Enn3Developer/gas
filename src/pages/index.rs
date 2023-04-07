use crate::widgets::hamburger;
use crate::{NMessage, NPage};
use eframe::App;
use egui::{Align, CentralPanel, Layout, ScrollArea, SidePanel, Visuals, WidgetText};
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.show_panel {
            SidePanel::left("left_panel").show(ctx, |ui| {
                if ui
                    .selectable_label(false, WidgetText::from("Login").heading())
                    .clicked()
                {
                    self.tx
                        .send(NMessage::Redirect(NPage::Login))
                        .unwrap_or_default();
                }
            });
        }

        CentralPanel::default().show(ctx, |ui| {
            // Top panel
            ui.horizontal_top(|ui| {
                ui.add(hamburger(&mut self.show_panel));
                ui.add_space(10.0);
                egui::warn_if_debug_build(ui);
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    ui.heading("GAS");
                });
            });
            ui.separator();

            // Main page
            ScrollArea::vertical().show(ui, |ui| {
                ui.label("Test");
            });
        });
    }
}
