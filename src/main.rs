use gtk::{BoxExt, ButtonExt, ContainerExt, LabelExt, WidgetExt};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

// the model
struct Model {
    counter: u32,
}

#[derive(Msg)]
enum MyMsg {
    Inc,
    Dec,
    Quit,
}

struct Win {
    model: Model,
    window: gtk::Window,
    lable: gtk::Label,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = MyMsg;

    fn model(_relm: &Relm<Self>, _param: Self::ModelParam) -> Self::Model {
        Model { counter: 0 }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            MyMsg::Dec => {
                self.model.counter -= 1;
                self.lable.set_text(&self.model.counter.to_string());
            }
            MyMsg::Inc => {
                self.model.counter += 1;
                self.lable.set_text(&self.model.counter.to_string());
            }
            MyMsg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // Build the UI
        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        let gtk_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let inc_button = gtk::Button::with_label("Inc");
        let dec_button = gtk::Button::with_label("Dec");
        let counter_label = gtk::Label::new(Some("0"));

        gtk_box.pack_end(&inc_button, true, true, 0);
        gtk_box.pack_end(&dec_button, true, true, 0);
        gtk_box.pack_end(&counter_label, true, true, 0);

        window.add(&gtk_box);

        window.show_all();

        // Connect events
        connect!(relm, inc_button, connect_clicked(_), MyMsg::Inc);
        connect!(relm, dec_button, connect_clicked(_), MyMsg::Dec);
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(MyMsg::Quit), gtk::Inhibit(false))
        );

        Self {
            model,
            window,
            lable: counter_label,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
