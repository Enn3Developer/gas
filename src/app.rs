use crate::widgets::{hamburger, toggle};
use egui::panel::Side;
use egui::{
    Align, CentralPanel, CollapsingHeader, Id, Layout, ScrollArea, SidePanel, Visuals, WidgetText,
};

pub struct TemplateApp {
    show_panel: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self { show_panel: false }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.show_panel {
            SidePanel::left("left_panel").show(ctx, |ui| {
                ui.selectable_label(false, WidgetText::from("Login").heading());
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
