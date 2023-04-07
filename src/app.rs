use eframe::App;
use egui::Visuals;
use std::rc::Rc;

use crate::data::account::{Account, AccountManager};
use crate::pages::index::Index;
use crate::pages::login::Login;
use crate::pages::shop::Shop;
use crate::{NMessage, NPage};
use flume::{Receiver, Sender};

pub struct NGas {
    current_page: Box<dyn App>,
    account_manager: Rc<AccountManager>,
    account: Option<Rc<Account>>,
    rx: Receiver<NMessage>,
    tx: Sender<NMessage>,
}

impl NGas {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());

        let (tx, rx) = flume::unbounded();

        let index = Index::new(tx.clone());

        let mut account_manager = AccountManager::new();
        account_manager.add_account(Account::new("admin", "StroongPasswd123!"));
        account_manager.add_account(Account::new("user12", "password"));

        Self {
            rx,
            tx,
            account_manager: Rc::new(account_manager),
            current_page: Box::new(index),
            account: None,
        }
    }

    fn redirect_page(&mut self, page: NPage) {
        match page {
            NPage::Index => {
                let index = Index::new(self.tx.clone());
                self.current_page = Box::new(index);
            }
            NPage::Login => {
                let login = Login::new(self.tx.clone(), self.account_manager.clone());
                self.current_page = Box::new(login);
            }
            NPage::Shop => {
                let shop = Shop::new(self.tx.clone(), self.account.clone().unwrap().clone());
                self.current_page = Box::new(shop);
            }
        }
    }
}

impl App for NGas {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.current_page.update(ctx, frame);

        while let Ok(message) = self.rx.try_recv() {
            match message {
                NMessage::Redirect(page) => self.redirect_page(page),
                NMessage::Login(account) => self.account = Some(Rc::new(account)),
            }
        }
    }
}
