use eframe::App;
use egui::Visuals;

use crate::pages::index::Index;
use crate::pages::login::Login;
use crate::{NMessage, NPage};
use flume::{Receiver, Sender};

pub struct NGas {
    current_page: Box<dyn App>,
    rx: Receiver<NMessage>,
    tx: Sender<NMessage>,
}

impl NGas {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());

        let (tx, rx) = flume::unbounded();

        let index = Index::new(tx.clone());

        Self {
            rx,
            tx,
            current_page: Box::new(index),
        }
    }
}

impl App for NGas {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.current_page.update(ctx, frame);

        while let Ok(message) = self.rx.try_recv() {
            match message {
                NMessage::Redirect(page) => match page {
                    NPage::Index => {
                        let index = Index::new(self.tx.clone());
                        self.current_page = Box::new(index);
                    }
                    NPage::Login => {
                        let login = Login::new(self.tx.clone());
                        self.current_page = Box::new(login);
                    }
                },
            }
        }
    }
}
