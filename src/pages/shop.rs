use crate::data::account::Account;
use crate::pages::common::{side_panel, top_panel};
use crate::NMessage;
use eframe::{App, Frame};
use egui::{CentralPanel, Context};
use flume::Sender;
use std::rc::Rc;

pub struct Shop {
    show_panel: bool,
    tx: Sender<NMessage>,
    account: Rc<Account>,
}

impl Shop {
    pub fn new(tx: Sender<NMessage>, account: Rc<Account>) -> Self {
        Self {
            tx,
            account,
            show_panel: false,
        }
    }
}

impl App for Shop {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        side_panel(ctx, self.tx.clone(), self.show_panel);

        CentralPanel::default().show(ctx, |ui| {
            top_panel(ui, "Shop", &mut self.show_panel);
        });
    }
}
