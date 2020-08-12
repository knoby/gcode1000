use gtk::{
    BoxExt, ButtonExt, ContainerExt, EntryExt, ScrolledWindowExt, StyleContextExt, TextBufferExt,
    TextViewExt, WidgetExt,
};
use relm::Relm;
use relm_derive::Msg;

#[derive(Debug, Msg)]
pub enum Msg {
    LogLine(String),
    SendCommand(String),
    ClearLog,
    KeyInSendCmd(gdk::EventKey),
}

pub struct Model {
    stream: relm::EventStream<Msg>,
}

struct GtkWidgets {
    root: gtk::Box,
    textview: gtk::TextView,
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

    fn model(relm: &Relm<Self>, _param: Self::ModelParam) -> Self::Model {
        Model {
            stream: relm.stream().clone(),
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::KeyInSendCmd(key) => {
                if let Some(key_name) = key.get_keyval().name() {
                    if key_name == "Return" {
                        self.model.stream.emit(Msg::SendCommand(
                            self.widgets.send_cmd.get_text().to_string(),
                        ));
                    };
                };
            }
            Msg::LogLine(text) => {
                // Get current time
                let time = chrono::Local::now().format("%H:%M:%S%.3f");
                // Append Message
                let mut end_iter = self.widgets.textview.get_buffer().unwrap().get_end_iter();
                self.widgets
                    .textview
                    .get_buffer()
                    .unwrap()
                    .insert(&mut end_iter, &format!("{} -> {}\n", time, text));
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
            Msg::SendCommand(_text) => {
                self.widgets.send_cmd.set_text("");
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
        textview.set_property_monospace(true);

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

        let send_cmd_clone = send_cmd.clone();
        relm::connect!(
            relm,
            send_btn,
            connect_clicked(_),
            Msg::SendCommand(send_cmd_clone.get_text().to_string())
        );
        relm::connect!(relm, clear_btn, connect_clicked(_), Msg::ClearLog);
        relm::connect!(
            relm,
            send_cmd,
            connect_key_press_event(_, key),
            return (Msg::KeyInSendCmd(key.clone()), gtk::Inhibit(false))
        );

        Self {
            model,
            widgets: GtkWidgets {
                root: root_box,
                send_cmd,
                textview,
            },
        }
    }
}
