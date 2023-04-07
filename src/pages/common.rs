use crate::widgets::hamburger;
use crate::{NMessage, NPage};
use egui::{Align, Context, Layout, SidePanel, Ui, WidgetText};
use flume::Sender;

pub fn top_panel(ui: &mut Ui, title: &str, show_panel: &mut bool) {
    ui.horizontal_top(|ui| {
        ui.add(hamburger(show_panel));
        egui::warn_if_debug_build(ui);
        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
            ui.add_space(10.0);
            ui.heading("GAS");
            ui.add_space(ui.available_width() / 2.0 - (title.len() / 2 + 1) as f32);
            ui.heading(title);
        });
    });
    ui.separator();
}

pub fn side_panel(ctx: &Context, tx: Sender<NMessage>, show_panel: bool) {
    if show_panel {
        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.add_space(20.0);
            if ui
                .selectable_label(false, WidgetText::from("Home").heading())
                .clicked()
            {
                tx.send(NMessage::Redirect(NPage::Index))
                    .unwrap_or_default();
            }

            if ui
                .selectable_label(false, WidgetText::from("Login").heading())
                .clicked()
            {
                tx.send(NMessage::Redirect(NPage::Login))
                    .unwrap_or_default();
            }
        });
    }
}
