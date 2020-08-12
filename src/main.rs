use gtk::prelude::*;
use relm::{connect, Component, ContainerWidget, Relm, Update, Widget};
use relm_derive::Msg;

mod connection;
mod control;
mod log;

#[derive(Debug, Clone, Msg)]
enum Msg {
    Quit,
    EnqueueCommand(String),
    ClearCommandQueue,
    EvalResponse(String),
    SendCommand,
    Connect,
    Disconnect,
}

struct Win {
    model: Model,
    _manual_control: Component<control::Widget>,
    _connection_control: Component<connection::Widget>,
    _logging: Component<log::Widget>,
    _port: Option<Box<dyn serialport::SerialPort>>,
    window: gtk::Window,
}

struct Model {
    command_queue: std::collections::VecDeque<String>,
    connected: bool,
    waiting_for_ok: bool,
    relm: Relm<Win>,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _param: Self::ModelParam) -> Self::Model {
        Model {
            command_queue: std::collections::VecDeque::new(),
            relm: relm.clone(),
            waiting_for_ok: false,
            connected: false,
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::Connect => {
                self.model.connected = true;
                self.model.relm.stream().emit(Msg::ClearCommandQueue);
            }
            Msg::Disconnect => {
                self.model.connected = false;
            }
            Msg::EnqueueCommand(command) => {
                if self.model.connected {
                    self.model.command_queue.push_back(command);
                    self.model.relm.stream().emit(Msg::SendCommand);
                }
            }
            Msg::SendCommand => {
                // Check if we are currently waiting for a response
                if !self.model.waiting_for_ok & self.model.connected {
                    // Is something in the queue?
                    if !self.model.command_queue.is_empty() {
                        let command = self.model.command_queue.pop_front().unwrap();
                        self._connection_control
                            .emit(connection::Msg::SendLine(command.clone()));
                        self._logging.emit(log::Msg::LogLine(command));
                        self.model.waiting_for_ok = true;
                    }
                }
            }
            Msg::EvalResponse(response) => {
                // Are we waiting for a response?
                if self.model.waiting_for_ok {
                    let maybe_ok = response.split_at(2);
                    if maybe_ok.0 == "ok" {
                        self.model.waiting_for_ok = false;
                        // Check if it is answer to temperature or position
                        match maybe_ok.1.chars().nth(0) {
                            Some('T') => self
                                ._manual_control
                                .emit(control::Msg::SetTemperature(maybe_ok.1.to_string())),
                            Some('X') => self
                                ._manual_control
                                .emit(control::Msg::SetTemperature(maybe_ok.1.to_string())),
                            _ => (),
                        }
                        // Send new command
                        self.model.relm.stream().emit(Msg::SendCommand);
                    }
                }
            }
            Msg::ClearCommandQueue => {
                self.model.command_queue.clear();
                self.model.waiting_for_ok = false;
            }
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

        let connection_control = vbox.add_widget::<connection::Widget>(());

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
        // Add Line to log
        connect!(connection_control@connection::Msg::ReciveLine(ref text), logging, log::Msg::LogLine(text.clone()));
        // Add Line to Command Queue
        connect!(logging@log::Msg::SendCommand(ref text), relm, Msg::EnqueueCommand(text.clone()));
        // Add Command from control
        connect!(manual_control@control::Msg::SendCmd(ref text), relm, Msg::EnqueueCommand(text.clone()));
        // Clear Command Buffer
        connect!(connection_control@connection::Msg::Disconnect, relm, Msg::Disconnect);
        connect!(connection_control@connection::Msg::ConnectionActive, relm, Msg::Connect);
        // Connect Response Eval
        connect!(connection_control@connection::Msg::ReciveLine(ref text), relm, Msg::EvalResponse(text.clone()));

        // Return the Widget
        Win {
            window,
            _manual_control: manual_control,
            _connection_control: connection_control,
            _logging: logging,
            _port: None,
            model,
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

fn main() {
    Win::run(()).unwrap();
}
