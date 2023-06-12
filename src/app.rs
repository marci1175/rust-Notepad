
use egui::Color32;
use egui::color_picker::Alpha;
use rfd::FileDialog;
use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::fs::File;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // delete text left behind from previous session
    #[serde(skip)]
    label: String,
    #[serde(skip)]
    window_open: bool,
    #[serde(skip)]
    emoji_window_open: bool,

    value: f32,
    color: Color32,
    rainbow: bool,
    font_size: f32,
    //font: egui::FontId,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "".to_owned(),
            value: 2.7,
            window_open: false,
            emoji_window_open: false,
            rainbow: false,
            font_size: 18.0,
            color: Color32::from_rgb(255, 255, 255),
        }
    }
}
impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        
        Default::default()
    }
}

fn count_lines(text: &str) -> usize {
    text.split('\n').count()
}

impl eframe::App for TemplateApp {
    
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        
        eframe::set_value(storage, eframe::APP_KEY, self);
        
    }
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        if self.window_open {
            egui::Window::new("Settings")
                .open(&mut self.window_open)
                .show(ctx, |ui| {
                    ui.label("Text size");
                    ui.add(egui::Slider::new(&mut self.font_size, 1.0..=100.0));
                    ui.label("Text color");
                    egui::color_picker::color_picker_color32(ui, &mut self.color, Alpha::Opaque);
                    
                });
        }
        if self.emoji_window_open {
            egui::Window::new("Emojis")
                .open(&mut self.emoji_window_open)
                .show(ctx, |ui| {
                    egui::Grid::new("emoji_grid").num_columns(3).show(ui, |ui| {
                        if ui.button("😦").clicked(){
                            self.label = self.label.clone() + "😦";
                        };
                        if ui.button("📶").clicked(){
                            self.label = self.label.clone() + "📶";
                        };  
                        if ui.button("🔛").clicked(){
                            self.label = self.label.clone() + "🔛";
                        };  
                        ui.end_row();
                        if ui.button("✡").clicked(){
                            self.label = self.label.clone() + "✡";
                        };  
                        if ui.button("😀").clicked(){
                            self.label = self.label.clone() + "😀";
                        };  
                        if ui.button("🆓").clicked(){
                            self.label = self.label.clone() + "🆓";
                        };  
                        ui.end_row();
                        if ui.button("🔥").clicked(){
                            self.label = self.label.clone() + "🔥";
                        };  
                        if ui.button("🔊").clicked(){
                            self.label = self.label.clone() + "🔊";
                        };  
                        if ui.button("🔇").clicked(){
                            self.label = self.label.clone() + "🔇";
                        }; 
                        ui.end_row();
                        if ui.button("🕛").clicked(){
                            self.label = self.label.clone() + "🕛";
                        };  
                        if ui.button("🖶").clicked(){
                            self.label = self.label.clone() + "🖶";
                        };  
                        if ui.button("🚾").clicked(){
                            self.label = self.label.clone() + "🚾";
                        }; 
                        ui.end_row();        
                        if ui.button("⚛").clicked(){
                            self.label = self.label.clone() + "⚛";
                        };    
                        if ui.button("🚫").clicked(){
                            self.label = self.label.clone() + "🚫";
                        };   
                        if ui.button("⚡").clicked(){
                            self.label = self.label.clone() + "⚡";
                        };   
                        ui.end_row();
                        if ui.button("☢").clicked(){
                            self.label = self.label.clone() + "☢";
                        };   
                        if ui.button("👍").clicked(){
                            self.label = self.label.clone() + "👍";
                        };   
                        if ui.button("👎").clicked(){
                            self.label = self.label.clone() + "👎";
                        };   
                        ui.end_row();
                        if ui.button("🗑").clicked(){
                            self.label = self.label.clone() + "🗑";
                        };   
                        if ui.button("💤").clicked(){
                            self.label = self.label.clone() + "💤";
                        };
                        if ui.button("🛡").clicked(){
                            self.label = self.label.clone() + "🛡";
                        };
                        ui.end_row();
                    });
                });
        }
        // For inspiration and more examples, go to https://emilk.github.io/egui
        if self.rainbow{
            //rainbow insert here
        }
        //bottom
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui|{
                let lenght = self.label.len();
                let lines = count_lines(&self.label);
                let final_lenght = lenght - (lines - 1);
                ui.label(lines.to_string() + " : Lines");

                //separate self.label into a vector by whitespaces, then count them
                let trimmed_string = self.label.trim();
                let words: Vec<&str> = trimmed_string.split_whitespace().collect();
                ui.label(words.len().to_string() + " : Words");
                ui.label(final_lenght.to_string() + " : Characters");
                //emoji button
                if ui.button("Show emojis").clicked() {
                    self.emoji_window_open = true;
                }
            });
        });
        //top
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.label("📋 Notes");
                
                if ui.button("Save As").clicked() {
                    //save as
                    let files = FileDialog::new()
                        .set_title("Save")
                        .add_filter("", &["txt"])
                        .set_directory("/")
                        .save_file();

                    if let Some(file_path) = files {
                        let mut file = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .open(file_path)
                            .expect("Failed to open file");
                        
                        //pushback info

                        // Write some data to the file
                        match write!(file ,"{}", self.label){
                            Ok(_) => {},
                            Err(e) => {
                                println!("Error opening the file : {}", e);
                            }
                        }
                    }
                }
                if ui.button("Open").clicked() {
                    let files = FileDialog::new()
                        .set_title("Open")
                        .set_directory("/")
                        .pick_file();
                    //START
                    if let Some(file_path) = files {
                        let mut file = File::open(file_path)
                            .expect("Failed to open file");

                        let mut contents = String::new();
                        file.read_to_string(&mut contents)
                            .expect("Failed to read file");
    
                            self.label = contents;
                        }
                }
                if ui.button("Settings").clicked() {
                    self.window_open = true;
                    
                }
            });
        });
        //center
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.label)
                        .text_color(self.color)
                        .font(egui::FontId::proportional(self.font_size)));
                });
            });
        });
        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
