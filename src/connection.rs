use gtk::prelude::*;
use relm::{connect, Relm};
use relm_derive::Msg;
use serialport::prelude::*;
use std::io::Read;

#[derive(Msg)]
pub enum Msg {
    Connect,
    SendLine(String),
    ReciveLine(String),
}

pub struct Model {
    port: Option<Box<dyn SerialPort>>,
    stream: relm::EventStream<Msg>,
}

pub struct Widgets {
    port_combobox: gtk::ComboBoxText,
    connect_btn: gtk::Button,
    root: gtk::Box,
}

pub struct Widget {
    widgets: Widgets,
    model: Model,
}

impl relm::Update for Widget {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _param: Self::ModelParam) -> Self::Model {
        Model {
            port: None,
            stream: relm.stream().clone(),
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::SendLine(_line) => (),
            Msg::ReciveLine(_line) => (),
            Msg::Connect => {
                if self.model.port.is_none() {
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
                        .widgets
                        .port_combobox
                        .get_active_text()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "".to_string());

                    let stream = self.model.stream.clone();
                    let (_channel, sender) =
                        relm::Channel::new(move |line: String| stream.emit(Msg::ReciveLine(line)));

                    if let Ok(mut port) =
                        serialport::open_with_settings(&connection_string, &port_settings)
                    {
                        std::thread::spawn(move || {
                            // Read data from port in an endless loop
                            let mut buffer = vec![0; 512];
                            let mut line = String::new();
                            loop {
                                // Try to read a line
                                match port.read(&mut buffer) {
                                    Ok(n) if n > 0 => {
                                        println!("{}", n);
                                        for &c in &buffer {
                                            if c != 0x0a {
                                                if c.is_ascii_alphanumeric()
                                                    | c.is_ascii_punctuation()
                                                    | c.is_ascii_whitespace()
                                                {
                                                    line.push(c.into());
                                                }
                                            } else {
                                                sender.send(line.clone()).unwrap();
                                                line.clear();
                                            };
                                        }
                                        buffer = vec![0; 512];
                                    }

                                    Ok(_) => (),

                                    Err(err) => match err.kind() {
                                        std::io::ErrorKind::TimedOut => (),
                                        _ => {
                                            println!("{:?}", err);
                                            break;
                                        }
                                    },
                                };
                            }
                        });
                        self.model.port = None;
                        self.widgets.connect_btn.set_label("Disconnect");
                        self.widgets
                            .connect_btn
                            .get_style_context()
                            .add_class("destructive-action");
                        self.widgets
                            .connect_btn
                            .get_style_context()
                            .remove_class("suggested-action");
                    }
                } else {
                    self.model.port.take(); // Take connection out of the Option and drop it
                    self.widgets.connect_btn.set_label("Connect");
                    self.widgets
                        .connect_btn
                        .get_style_context()
                        .remove_class("destructive-action");
                    self.widgets
                        .connect_btn
                        .get_style_context()
                        .add_class("suggested-action");
                }
            }
        }
    }
}

impl relm::Widget for Widget {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.widgets.root.clone()
    }

    fn view(relm: &Relm<Self>, _model: Self::Model) -> Self {
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

        connect!(relm, connect_btn, connect_clicked(_), Msg::Connect);

        Self {
            widgets: Widgets {
                root: statusline,
                connect_btn,
                port_combobox,
            },
            model: Model {
                port: None,
                stream: relm.stream().clone(),
            },
        }
    }
}

/// Find avaible ports
fn get_ports() -> Vec<String> {
    serialport::available_ports()
        .unwrap()
        .iter()
        .map(|port| port.port_name.clone())
        .collect()
}
