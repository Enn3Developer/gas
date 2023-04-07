use crate::data::account::{Account, AccountManager};
use crate::pages::common::{side_panel, top_panel};
use crate::{NMessage, NPage};
use eframe::{App, Frame};
use egui::{CentralPanel, Color32, Context, TextEdit};
use flume::Sender;
use std::rc::Rc;

pub struct Login {
    tx: Sender<NMessage>,
    show_panel: bool,
    account_manager: Rc<AccountManager>,
    user: String,
    pass: String,
    invalid: bool,
}

impl Login {
    pub fn new(tx: Sender<NMessage>, account_manager: Rc<AccountManager>) -> Self {
        Self {
            tx,
            account_manager,
            show_panel: false,
            user: String::new(),
            pass: String::new(),
            invalid: false,
        }
    }
}

impl App for Login {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        side_panel(ctx, self.tx.clone(), self.show_panel);

        CentralPanel::default().show(ctx, |ui| {
            top_panel(ui, "Login", &mut self.show_panel);

            ui.label("Username:");
            ui.text_edit_singleline(&mut self.user);
            ui.add_space(10.0);
            ui.label("Password:");
            ui.add(TextEdit::singleline(&mut self.pass).password(true));
            ui.add_space(10.0);
            if ui.button("Login").clicked() {
                let account = Account::new(&self.user, &self.pass);
                if self.account_manager.check(&account) {
                    self.invalid = false;
                    self.tx.send(NMessage::Login(account)).unwrap_or_default();
                    self.tx
                        .send(NMessage::Redirect(NPage::Shop))
                        .unwrap_or_default();
                } else {
                    self.invalid = true;
                }
            }

            if self.invalid {
                ui.scope(|ui| {
                    ui.visuals_mut().override_text_color = Some(Color32::DARK_RED);
                    ui.label("Invalid username or passwrd");
                });
            }
        });
    }
}
