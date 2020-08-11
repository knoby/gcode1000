use gtk::prelude::*;
use relm::{connect, Relm};
use relm_derive::Msg;
use serialport::prelude::*;
use std::io::Read;

#[derive(Msg)]
pub enum Msg {
    Connect,
    SendLine(String),
    Disconnect,
    ReciveLine(String),
}

pub struct Model {
    connection_thread: Option<std::thread::JoinHandle<()>>,
    stream: relm::EventStream<Msg>,
    thread_command: Option<std::sync::mpsc::Sender<ThreadCmd>>,
}

pub struct Widgets {
    port_combobox: gtk::ComboBoxText,
    connect_btn: gtk::Button,
    disconnect_btn: gtk::Button,
    root: gtk::Box,
}

enum ThreadCmd {
    Disconnect,
    SendLine(String),
}

enum ThreadStatus {
    ConnectionError,
    RecivedLine(String),
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
            connection_thread: None,
            stream: relm.stream().clone(),
            thread_command: None,
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::SendLine(line) => {
                if let Some(ref thread_command) = self.model.thread_command {
                    thread_command
                        .send(ThreadCmd::SendLine(format!("{}\n", line)))
                        .ok();
                }
            }
            Msg::ReciveLine(_line) => (),
            Msg::Disconnect => {
                // Send Stop signal to thread
                if let Some(commander) = self.model.thread_command.take() {
                    commander.send(ThreadCmd::Disconnect).ok();
                }
                // Wait for thread to finish
                if let Some(join_handle) = self.model.connection_thread.take() {
                    join_handle.join().ok(); // We don't care about result
                }
                self.widgets.connect_btn.set_sensitive(true);
                self.widgets.disconnect_btn.set_sensitive(false);
            }
            Msg::Connect => {
                if self.model.connection_thread.is_some() {
                    self.update(Msg::Disconnect);
                } else {
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

                    if let Ok((mpsc_tx, thread_handle)) = create_connection_thread(
                        connection_string,
                        port_settings,
                        self.model.stream.clone(),
                    ) {
                        self.widgets.connect_btn.set_sensitive(false);
                        self.widgets.disconnect_btn.set_sensitive(true);
                        self.model.thread_command = Some(mpsc_tx);
                        self.model.connection_thread = Some(thread_handle);
                    }
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
        let disconnect_btn = gtk::Button::with_label(&"Disconnect");
        disconnect_btn
            .get_style_context()
            .add_class("destructive-action");
        statusline.pack_start(&disconnect_btn, false, false, 0);
        disconnect_btn.set_sensitive(false);

        connect!(relm, connect_btn, connect_clicked(_), Msg::Connect);
        connect!(relm, disconnect_btn, connect_clicked(_), Msg::Disconnect);

        Self {
            widgets: Widgets {
                root: statusline,
                connect_btn,
                disconnect_btn,
                port_combobox,
            },
            model: Model {
                connection_thread: None,
                stream: relm.stream().clone(),
                thread_command: None,
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

/// Creates the thread with all channels that handles the connection to the printer
fn create_connection_thread(
    connection_string: String,
    port_settings: SerialPortSettings,
    stream: relm::EventStream<Msg>,
) -> Result<
    (
        std::sync::mpsc::Sender<ThreadCmd>,
        std::thread::JoinHandle<()>,
    ),
    (),
> {
    // Create Channel from and to thread
    let (_channel, sender) = relm::Channel::new(move |msg: ThreadStatus| {
        match msg {
            ThreadStatus::ConnectionError => stream.emit(Msg::Disconnect),
            ThreadStatus::RecivedLine(line) => stream.emit(Msg::ReciveLine(line)),
        };
    });

    let (mpsc_tx, mpsc_rx) = std::sync::mpsc::channel::<ThreadCmd>();

    if let Ok(mut port) = serialport::open_with_settings(&connection_string, &port_settings) {
        let thread_handle = std::thread::spawn(move || {
            // Read data from port in an endless loop
            let mut buffer = vec![0; 512];
            let mut line = String::new();
            loop {
                // Read Command
                match mpsc_rx.try_recv() {
                    Ok(cmd) => match cmd {
                        ThreadCmd::Disconnect => break,
                        ThreadCmd::SendLine(line) => {
                            if port.write_all(line.as_ref()).is_err() {
                                break;
                            }
                        }
                    },
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
                    Err(std::sync::mpsc::TryRecvError::Empty) => (),
                }
                // Try to read a line
                match port.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        for &c in &buffer {
                            if c != 0x0a {
                                if c.is_ascii_alphanumeric()
                                    | c.is_ascii_punctuation()
                                    | c.is_ascii_whitespace()
                                {
                                    line.push(c.into());
                                }
                            } else {
                                sender
                                    .send(ThreadStatus::RecivedLine(line.clone()))
                                    .unwrap();
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
            sender.send(ThreadStatus::ConnectionError).ok();
        });
        Ok((mpsc_tx, thread_handle))
    } else {
        Err(())
    }
}
