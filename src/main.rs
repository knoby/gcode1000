use gtk::{BoxExt, ButtonExt, ContainerExt, WidgetExt};
use relm::{connect, Component, ContainerWidget, Relm, Update, Widget};
use relm_derive::Msg;

mod counter;

#[derive(Msg)]
enum Msg {
    Quit,
}

struct Win {
    counter_1: Component<counter::Counter>,
    counter_2: Component<counter::Counter>,
    window: gtk::Window,
}

impl Update for Win {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {}

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // Create the UI
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_size_request(400, 200);

        let gtk_box = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let counter_1 = gtk_box.add_widget::<counter::Counter>(());
        let counter_2 = gtk_box.add_widget::<counter::Counter>(());
        let reset_button = gtk::Button::with_label("Reset");
        gtk_box.add(&reset_button);

        for child in gtk_box.get_children() {
            gtk_box.set_child_packing(&child, true, true, 0, gtk::PackType::Start);
        }

        window.add(&gtk_box);

        window.show_all();

        // Connect the signals
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), gtk::Inhibit(false))
        );
        connect!(
            reset_button,
            connect_clicked(_),
            counter_2,
            counter::Msg::Reset
        );
        connect!(
            reset_button,
            connect_clicked(_),
            counter_1,
            counter::Msg::Reset
        );

        // Return the Widget
        Win {
            window,
            counter_1,
            counter_2,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
