use gtk::GridExt;
use relm::Relm;
use relm_derive::Msg;

#[derive(Debug, Msg)]
pub enum Msg {}

#[derive(Debug, Clone, Copy, Default)]
pub struct Model {}

struct GtkWidgets {
    root: gtk::Grid,
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

    fn update(&mut self, event: Self::Msg) {}
}

impl relm::Widget for Widget {
    type Root = gtk::Grid;

    fn root(&self) -> Self::Root {
        self.widgets.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // Create all UI Elements
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
        for _ in 0..7 {
            grid.insert_column(0);
        }

        grid.attach(&btn_x_neg, 0, 1, 1, 2);
        grid.attach(&btn_x_pos, 2, 1, 1, 2);
        grid.attach(&btn_y_neg, 1, 2, 1, 2);
        grid.attach(&btn_y_pos, 1, 0, 1, 2);
        grid.attach(&btn_z_neg, 3, 2, 1, 2);
        grid.attach(&btn_z_pos, 3, 0, 1, 2);
        grid.attach(&gtk::Label::new(None), 4, 0, 1, 1); // I want an empty col
        grid.attach(&btn_e1_neg, 5, 2, 1, 2);
        grid.attach(&btn_e1_pos, 5, 0, 1, 2);
        grid.attach(&btn_e2_neg, 6, 2, 1, 2);
        grid.attach(&btn_e2_pos, 6, 0, 1, 2);
        grid.attach(&gtk::Label::new(None), 0, 4, 1, 1); // I want an empty row
        grid.attach(&btn_x_home, 1, 5, 1, 2);
        grid.attach(&btn_y_home, 2, 5, 1, 2);
        grid.attach(&btn_z_home, 3, 5, 1, 2);

        Self {
            model,
            widgets: GtkWidgets { root: grid },
        }
    }
}
