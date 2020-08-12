use gtk::prelude::*;
use relm::Relm;
use relm_derive::Msg;

#[derive(Debug, Msg)]
pub enum Msg {
    SendCmd(String),
    //MoveX(f32),
    //MoveY(f32),
    //MoveZ(f32),
    //MoveE1(f32),
    //MoveE2(f32),
    GetTemperature,
    SetTemperature(String),
    SetPosition(String),
    GetPosition,
}

pub struct Model {
    relm: relm::Relm<Widget>,
}

struct GtkWidgets {
    root: gtk::Box,
    label_x_pos: gtk::Label,
    label_y_pos: gtk::Label,
    label_z_pos: gtk::Label,
    label_e1_temp: gtk::Label,
    label_bed_temp: gtk::Label,
}

pub struct Widget {
    model: Model,
    widgets: GtkWidgets,
}

impl relm::Update for Widget {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        relm::interval(relm.stream(), 1000, || Msg::GetTemperature);
        relm::interval(relm.stream(), 1000, || Msg::GetPosition);
    }

    fn model(relm: &Relm<Self>, _param: Self::ModelParam) -> Self::Model {
        Model { relm: relm.clone() }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::GetPosition => self
                .model
                .relm
                .stream()
                .emit(Msg::SendCmd("M114".to_string())),
            Msg::GetTemperature => self
                .model
                .relm
                .stream()
                .emit(Msg::SendCmd("M105".to_string())),
            Msg::SendCmd(_cmd) => (),
            Msg::SetPosition(response) => (),
            Msg::SetTemperature(response) => (),
        }
    }
}

impl relm::Widget for Widget {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.widgets.root.clone()
    }

    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self {
        // The root widget
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

        // Create all UI Elements for manual control
        let btn_x_neg = gtk::Button::with_label("X-");
        let btn_x_pos = gtk::Button::with_label("X+");
        let btn_y_neg = gtk::Button::with_label("Y-");
        let btn_y_pos = gtk::Button::with_label("Y+");
        let btn_z_neg = gtk::Button::with_label("Z-");
        let btn_z_pos = gtk::Button::with_label("Z+");
        let btn_e1_neg = gtk::Button::with_label("E1-");
        let btn_e1_pos = gtk::Button::with_label("E1+");
        let btn_e2_neg = gtk::Button::with_label("E2-");
        let btn_e2_pos = gtk::Button::with_label("E2+");
        let btn_x_home = gtk::Button::with_label("Home X");
        let btn_y_home = gtk::Button::with_label("Home Y");
        let btn_z_home = gtk::Button::with_label("Home Z");

        let grid = gtk::Grid::new();
        grid.set_column_spacing(3);
        grid.set_row_spacing(3);
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);
        for _ in 0..9 {
            grid.insert_row(0);
        }
        for _ in 0..9 {
            grid.insert_column(0);
        }

        grid.attach(&btn_x_neg, 1, 1, 1, 2);
        grid.attach(&btn_x_pos, 3, 1, 1, 2);
        grid.attach(&btn_y_neg, 2, 2, 1, 2);
        grid.attach(&btn_y_pos, 2, 0, 1, 2);
        grid.attach(&btn_z_neg, 4, 2, 1, 2);
        grid.attach(&btn_z_pos, 4, 0, 1, 2);
        grid.attach(&gtk::Label::new(None), 5, 0, 1, 1); // I want an empty col
        grid.attach(&btn_e1_neg, 6, 2, 1, 2);
        grid.attach(&btn_e1_pos, 6, 0, 1, 2);
        grid.attach(&btn_e2_neg, 7, 2, 1, 2);
        grid.attach(&btn_e2_pos, 7, 0, 1, 2);
        grid.attach(&gtk::Label::new(None), 0, 4, 1, 1); // I want an empty row
        grid.attach(&btn_x_home, 2, 5, 1, 2);
        grid.attach(&btn_y_home, 3, 5, 1, 2);
        grid.attach(&btn_z_home, 4, 5, 1, 2);
        grid.attach(&gtk::Label::new(None), 8, 0, 1, 1); // I want an empty coll

        vbox.pack_start(&grid, false, false, 20);

        // The Status widget

        // A Grid for the Position (2x3)
        let grid_pos = gtk::Grid::new();
        for _ in 0..3 {
            grid_pos.insert_column(0);
        }

        for label in ["X:", "Y:", "Z:"].iter() {
            grid_pos.insert_row(0);
            grid_pos.attach(&gtk::Label::new(Some(label)), 0, 0, 1, 1);
            grid_pos.attach(&gtk::Label::new(Some("mm")), 2, 0, 1, 1);
        }

        let label_x_pos = gtk::Label::new(Some("0.0"));
        label_x_pos.set_property_width_request(100);
        let label_y_pos = gtk::Label::new(Some("0.0"));
        label_y_pos.set_property_width_request(100);
        let label_z_pos = gtk::Label::new(Some("0.0"));
        label_z_pos.set_property_width_request(100);

        grid_pos.attach(&label_x_pos, 1, 0, 1, 1);
        grid_pos.attach(&label_y_pos, 1, 1, 1, 1);
        grid_pos.attach(&label_z_pos, 1, 2, 1, 1);

        // A Grid for the Temperature
        let grid_temp = gtk::Grid::new();
        for _ in 0..3 {
            grid_temp.insert_column(0);
        }

        for label in ["E1:", "Bed:"].iter() {
            grid_temp.insert_row(0);
            grid_temp.attach(&gtk::Label::new(Some(label)), 0, 0, 1, 1);
            grid_temp.attach(&gtk::Label::new(Some("°C")), 2, 0, 1, 1);
        }

        let label_e1_temp = gtk::Label::new(Some("0.0"));
        label_e1_temp.set_property_width_request(100);
        let label_bed_temp = gtk::Label::new(Some("0.0"));
        label_bed_temp.set_property_width_request(100);
        grid_temp.attach(&label_e1_temp, 1, 0, 1, 1);
        grid_temp.attach(&label_bed_temp, 1, 1, 1, 1);

        // Box to hold the Status
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 3);
        hbox.pack_start(&grid_pos, false, false, 50);
        hbox.pack_start(&grid_temp, false, false, 50);

        vbox.pack_start(&hbox, true, true, 20);

        Self {
            model,
            widgets: GtkWidgets {
                root: vbox,
                label_x_pos,
                label_y_pos,
                label_z_pos,
                label_e1_temp,
                label_bed_temp,
            },
        }
    }
}
