use egui::{Response, Sense, Ui, Widget, WidgetInfo, WidgetType};

pub fn toggle_ui(ui: &mut Ui, on: &mut bool) -> Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }

    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

pub fn toggle(on: &mut bool) -> impl Widget + '_ {
    move |ui: &mut Ui| toggle_ui(ui, on)
}

pub fn hamburger_ui(ui: &mut Ui, on: &mut bool) -> Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(1.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }

    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact_selectable(&response, false);
        let rect = rect.expand(visuals.expansion);
        if !*on {
            ui.painter()
                .line_segment([rect.left_top(), rect.right_top()], visuals.fg_stroke);
            ui.painter()
                .line_segment([rect.left_center(), rect.right_center()], visuals.fg_stroke);
            ui.painter()
                .line_segment([rect.left_bottom(), rect.right_bottom()], visuals.fg_stroke);
        } else {
            ui.painter()
                .line_segment([rect.left_top(), rect.right_bottom()], visuals.fg_stroke);
            ui.painter()
                .line_segment([rect.left_bottom(), rect.right_top()], visuals.fg_stroke);
        }
    }

    response
}

pub fn hamburger(on: &mut bool) -> impl Widget + '_ {
    move |ui: &mut Ui| hamburger_ui(ui, on)
}
