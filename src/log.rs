use gtk::{
    BoxExt, ButtonExt, ContainerExt, EntryExt, ScrolledWindowExt, StyleContextExt, TextBufferExt,
    TextViewExt, WidgetExt,
};
use relm::Relm;
use relm_derive::Msg;

#[derive(Debug, Msg)]
pub enum Msg {
    LogLine(String),
    ClearLog,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Model {}

struct GtkWidgets {
    root: gtk::Box,
    textview: gtk::TextView,
    clear_btn: gtk::Button,
    send_btn: gtk::Button,
    send_cmd: gtk::Entry,
}

pub struct Widget {
    model: Model,
    widgets: GtkWidgets,
}

impl relm::Update for Widget {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {
        Model::default()
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::LogLine(text) => {
                // Get current time
                let time = std::time::SystemTime::now();
                let mut end_iter = self.widgets.textview.get_buffer().unwrap().get_end_iter();
                self.widgets
                    .textview
                    .get_buffer()
                    .unwrap()
                    .insert(&mut end_iter, &format!("> {}\n", text));
                self.widgets
                    .textview
                    .scroll_to_iter(&mut end_iter, 0.0, false, 0.0, 0.0);
            }
            Msg::ClearLog => {
                let mut start = self.widgets.textview.get_buffer().unwrap().get_start_iter();
                let mut end = self.widgets.textview.get_buffer().unwrap().get_end_iter();
                self.widgets
                    .textview
                    .get_buffer()
                    .unwrap()
                    .delete(&mut start, &mut end)
            }
        }
    }
}

impl relm::Widget for Widget {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.widgets.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // Create all UI Elements
        let root_box = gtk::Box::new(gtk::Orientation::Vertical, 3);

        let textview = gtk::TextView::new();
        textview.set_cursor_visible(false);
        textview.set_editable(false);

        let scrollview = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scrollview.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        scrollview.add(&textview);

        root_box.pack_start(&scrollview, true, true, 3);

        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 3);

        let send_cmd = gtk::Entry::new();
        hbox.pack_start(&send_cmd, true, true, 0);

        let send_btn = gtk::Button::with_label("Send");
        send_btn.get_style_context().add_class("suggested-action");
        hbox.pack_start(&send_btn, false, false, 3);

        let clear_btn = gtk::Button::with_label("Clear");
        clear_btn
            .get_style_context()
            .add_class("destructive-action");
        hbox.pack_start(&clear_btn, false, false, 3);

        root_box.pack_start(&hbox, false, false, 3);

        relm::connect!(
            relm,
            send_btn,
            connect_clicked(_),
            Msg::LogLine("Hallo Welt!".to_string())
        );
        relm::connect!(relm, clear_btn, connect_clicked(_), Msg::ClearLog);

        Self {
            model,
            widgets: GtkWidgets {
                root: root_box,
                send_btn,
                send_cmd,
                clear_btn,
                textview,
            },
        }
    }
}
