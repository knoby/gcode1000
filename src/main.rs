use gtk::prelude::*;
use relm::{connect, Component, ContainerWidget, Relm, Update, Widget};
use relm_derive::Msg;

mod control;

#[derive(Msg)]
enum Msg {
    Quit,
}

struct Win {
    manual_control: Component<control::Widget>,
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

    fn view(relm: &Relm<Self>, _model: Self::Model) -> Self {
        // Create the UI

        // The main Window
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_size_request(800, 600);
        window.maximize();

        // Add a header bar to the window
        let header_bar = gtk::HeaderBarBuilder::default()
            .show_close_button(true)
            .title("GCode 1000")
            .build();
        window.set_titlebar(Some(&header_bar));

        // Create a notebook to have some nice tabs on the left side
        let notebook = gtk::NotebookBuilder::default()
            .tab_pos(gtk::PositionType::Left)
            .build();

        // Add the manual control page
        let manual_control = notebook.add_widget::<control::Widget>(());
        notebook.set_tab_label(
            &notebook.get_nth_page(Some(0)).unwrap(), // Safe to unwrap because we added the 0st element just bevore
            Some(&create_tab_widget("Move")),
        );

        // Add Print Page
        let printing = gtk::Label::new(Some("Printing"));
        notebook.add(&printing);
        notebook.set_tab_label(
            &notebook.get_nth_page(Some(1)).unwrap(), // Safe to unwrap because we added the 1st element just bevore
            Some(&create_tab_widget("Print")),
        );

        // Add Settings Page
        let settings = gtk::Label::new(Some("Settings"));
        notebook.add(&settings);
        notebook.set_tab_label(
            &notebook.get_nth_page(Some(2)).unwrap(), // Safe to unwrap because we added the 2st element just bevore
            Some(&create_tab_widget("Settings")),
        );

        window.add(&notebook);

        window.show_all();

        // Connect the signals
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), gtk::Inhibit(false))
        );

        // Return the Widget
        Win {
            window,
            manual_control,
        }
    }
}

fn create_tab_widget(label: &str) -> gtk::Box {
    let tab_widget = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let mut image_path = "resources/png/".to_string();
    image_path.push_str(match label {
        "Settings" => "settings.png",
        "Move" => "move.png",
        "Print" => "printer.png",
        _ => "",
    });
    tab_widget.pack_start(&gtk::Image::from_file(image_path), false, false, 0);
    tab_widget.pack_start(&gtk::Label::new(Some(label)), false, false, 0);
    tab_widget.show_all();

    tab_widget
}

fn main() {
    Win::run(()).unwrap();
}
