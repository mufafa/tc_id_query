mod parser;
use parser::tcid::*;
use regex::Regex;



use eframe::{egui::{self, RichText}, epaint::Color32};
const SPACE : f32 = 12.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "TC Kimlik Numarası",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    id: String,
    name: String,
    color: Color32,
    status: RichText,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            id: generate_id_number().to_string(),
            name: String::new(),
            color: Color32::from_rgb(255,0,0),
            status: RichText::new(""),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(SPACE);
            
            ui.horizontal(|ui| {
                if ui.button("ID Oluştur").clicked() {
                    self.id = generate_id_number().to_string();
                }
                ui.text_edit_singleline(&mut self.id);
            });

            ui.add_space(SPACE);

            ui.horizontal(|ui| {
                ui.label("Sorgu için yazın");
                let output = egui::TextEdit::singleline(&mut self.name)
                            .char_limit(11)
                            .text_color(self.color)
                            .vertical_align(eframe::emath::Align::Center)
                            .hint_text("tc numarasını yazın!").show(ui);
                
                if output.response.changed() {
                    self.status = RichText::new("");
                    match self.name.parse::<u64>() {
                        Err(_) => {
                            self.status = RichText::new("Sadece rakam kullanın").color(Color32::RED);
                        },

                        Ok(parsed_number) => {
                            match extract_value(&self.name) {
                                Some(_) => {
                                    match validate_idnumber(parsed_number) {
                                        Ok(_) => {
                                            self.color =Color32::GREEN;
                                            self.status = RichText::new("Doğrulama başarılı");
                                        },
                                        Err(e) => {
                                            self.color =Color32::RED;
                                            self.status = RichText::new(e);
                                        },
                                    };
                                    
                                },
                                None => {
                                    self.status = RichText::new("Hata");
                                    self.color =Color32::RED;
                                },
                            };
                        },
                      }
                }
            });
            ui.add_space(SPACE);
            ui.code(self.status.text());
        });
    }
}


fn extract_value(input: &str) -> Option<&str> {
    
    let re: Regex = Regex::new(r"^[1-9]\d{10}$").unwrap();
    match re.captures(input) {
        Some(caps) => {
            let cap = caps.get(0).unwrap().as_str();
            return Some(cap);
        },
        None => {
            return  None;
        },
    }
    
}
