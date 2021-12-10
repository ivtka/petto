use eframe::egui::{Color32, CtxRef, FontDefinitions, FontFamily, Label, Separator, TextStyle, Ui};
use std::borrow::Cow;

use customersapi::{Customer, MongoDB, date_to_string};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

pub(crate) struct Petto {
    mongo: MongoDB,
    customers: Vec<Customer>,
}

impl Petto {
    pub(crate) fn new() -> Petto {
        Petto {
            mongo: MongoDB::init(),
            customers: Vec::new(),
        }
    }

    pub(crate) fn configure_fonts(&self, ctx: &CtxRef) {
        // create font def object
        let mut font_def = FontDefinitions::default();
        // load up the font
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            Cow::Borrowed(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
        );
        // set the size of different text styles
        font_def
            .family_and_size
            .insert(TextStyle::Heading, (FontFamily::Proportional, 35.));
        font_def
            .family_and_size
            .insert(TextStyle::Body, (FontFamily::Proportional, 20.0));
        // load the font using the context objects
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());
        ctx.set_fonts(font_def);
    }

    pub(crate) fn render_customer_card(&mut self, ui: &mut Ui) {
        self.customers = self.mongo.get_customers(); 
        for customer in &self.customers {
            ui.add_space(PADDING);
            // render name
            let name = format!("▶ {}", customer.name);
            ui.colored_label(WHITE, name);
            // render anothers fields
            ui.add_space(PADDING);
            let datetime = date_to_string(&customer.order_datetime); 
            let order_datetime = Label::new(datetime).text_style(TextStyle::Button);
            ui.add(order_datetime);
            ui.add_space(PADDING);
            ui.label(&customer.phone);
            ui.add_space(PADDING);
            ui.label(&customer.email);
            ui.add_space(PADDING);
            ui.label(&customer.ordered_service);
            ui.add_space(PADDING);
            ui.label(&customer.examined_doctor);
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}

