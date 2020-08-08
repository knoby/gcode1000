use gtk::prelude::*;
use relm::{connect, Component, ContainerWidget, Relm, Update, Widget};
use relm_derive::Msg;
use serialport::prelude::*;

mod control;
mod log;

#[derive(Debug, Clone, Msg)]
enum Msg {
    Quit,
    GetPorts,
    Connect,
}

struct ConnectionWidgets {
    port_combobox: gtk::ComboBoxText,
    connect_btn: gtk::Button,
}

struct Win {
    manual_control: Component<control::Widget>,
    logging: Component<log::Widget>,
    port: Option<Box<dyn serialport::SerialPort>>,
    window: gtk::Window,
    connection_widgets: ConnectionWidgets,
}

impl Update for Win {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {}

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::GetPorts => {
                // Remove all Ports and get new
                self.connection_widgets.port_combobox.remove_all();
                for port in get_ports() {
                    self.connection_widgets.port_combobox.append_text(&port);
                }
            }
            Msg::Connect => {
                if self.port.is_none() {
                    // Open a connection to the port specified in the connection tab
                    let port_settings = serialport::SerialPortSettings {
                        baud_rate: 250_000,
                        stop_bits: StopBits::One,
                        data_bits: DataBits::Eight,
                        flow_control: FlowControl::None,
                        parity: Parity::None,
                        timeout: std::time::Duration::from_millis(10),
                    };
                    let connection_string = self
                        .connection_widgets
                        .port_combobox
                        .get_active_text()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "".to_string());
                    if let Ok(port) =
                        serialport::open_with_settings(&connection_string, &port_settings)
                    {
                        self.port = Some(port);
                        self.connection_widgets.connect_btn.set_label("Disconnect");
                        self.connection_widgets
                            .connect_btn
                            .get_style_context()
                            .add_class("destructive-action");
                        self.connection_widgets
                            .connect_btn
                            .get_style_context()
                            .remove_class("suggested-action");
                    }
                } else {
                    self.port.take(); // Take connection out of the Option and drop it
                    self.connection_widgets.connect_btn.set_label("Connect");
                    self.connection_widgets
                        .connect_btn
                        .get_style_context()
                        .remove_class("destructive-action");
                    self.connection_widgets
                        .connect_btn
                        .get_style_context()
                        .add_class("suggested-action");
                }
            }
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

        // Create vertical box to store Statusline and main window
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);

        // Create the status line
        let statusline = gtk::Box::new(gtk::Orientation::Horizontal, 2);

        // Add a simple combobox to choose the port
        let port_combobox = gtk::ComboBoxText::new();
        for port in get_ports().iter() {
            port_combobox.append_text(port);
        }
        port_combobox.set_active(Some(0));

        statusline.pack_start(&gtk::Label::new(Some("Port:")), false, false, 0);
        statusline.pack_start(&port_combobox, false, false, 0);

        let connect_btn = gtk::Button::with_label(&"Connect");
        connect_btn
            .get_style_context()
            .add_class("suggested-action");
        statusline.pack_start(&connect_btn, false, false, 0);

        vbox.pack_start(&statusline, false, false, 0);

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

        // Add Log Page
        let logging = notebook.add_widget::<log::Widget>(());
        notebook.set_tab_label(
            &notebook.get_nth_page(Some(2)).unwrap(), // Safe to unwrap because we added the 1st element just bevore
            Some(&create_tab_widget("Log")),
        );

        // Add Settings Page
        let settings = gtk::Label::new(Some("Settings"));
        notebook.add(&settings);
        notebook.set_tab_label(
            &notebook.get_nth_page(Some(3)).unwrap(), // Safe to unwrap because we added the 2st element just bevore
            Some(&create_tab_widget("Settings")),
        );

        vbox.pack_end(&notebook, true, true, 0);

        window.add(&vbox);

        window.show_all();

        // Connect the signals
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), gtk::Inhibit(false))
        );
        connect!(relm, connect_btn, connect_clicked(_), Msg::Connect);

        // Return the Widget
        Win {
            window,
            manual_control,
            logging,
            connection_widgets: ConnectionWidgets {
                port_combobox,
                connect_btn,
            },
            port: None,
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
        "Log" => "menu.png",
        _ => "",
    });
    tab_widget.pack_start(&gtk::Image::from_file(image_path), false, false, 0);
    tab_widget.pack_start(&gtk::Label::new(Some(label)), false, false, 0);
    tab_widget.show_all();

    tab_widget
}

/// Find avaible ports
fn get_ports() -> Vec<String> {
    serialport::available_ports()
        .unwrap()
        .iter()
        .map(|port| port.port_name.clone())
        .collect()
}

fn main() {
    Win::run(()).unwrap();
}
