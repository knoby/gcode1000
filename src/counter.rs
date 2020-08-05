use gtk::{BoxExt, ButtonExt, LabelExt};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

pub struct Model {
    counter: u32,
}

#[derive(Msg)]
pub enum Msg {
    Inc,
    Dec,
    Reset,
}

struct Widgets {
    label: gtk::Label,
    root: gtk::Box,
}

pub struct Counter {
    model: Model,
    widgets: Widgets,
}

impl Update for Counter {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_relm: &Relm<Self>, _param: Self::ModelParam) -> Self::Model {
        Model { counter: 0 }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::Dec => {
                self.model.counter -= 1;
                self.widgets.label.set_text(&self.model.counter.to_string());
            }
            Msg::Inc => {
                self.model.counter += 1;
                self.widgets.label.set_text(&self.model.counter.to_string());
            }
            Msg::Reset => {
                self.model.counter = 0;
                self.widgets.label.set_text(&self.model.counter.to_string());
            }
        }
    }
}

impl Widget for Counter {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.widgets.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // Build the UI
        let gtk_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let inc_button = gtk::Button::with_label("Inc");
        let dec_button = gtk::Button::with_label("Dec");
        let counter_label = gtk::Label::new(Some("0"));

        gtk_box.pack_start(&inc_button, true, true, 0);
        gtk_box.pack_start(&dec_button, true, true, 0);
        gtk_box.pack_start(&counter_label, true, true, 0);

        // Connect events
        connect!(relm, inc_button, connect_clicked(_), Msg::Inc);
        connect!(relm, dec_button, connect_clicked(_), Msg::Dec);

        Self {
            model,
            widgets: Widgets {
                root: gtk_box,
                label: counter_label,
            },
        }
    }
}
