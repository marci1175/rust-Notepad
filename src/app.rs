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

    window_open: bool,
    value: f32,
    color: Color32,
    rainbow: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "".to_owned(),
            value: 2.7,
            window_open: false,
            rainbow: false,
            color: Color32::from_rgb(255, 255, 255),
        }
    }
}
impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}



impl eframe::App for TemplateApp {
    
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        
        eframe::set_value(storage, eframe::APP_KEY, self);
        
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.window_open {
            egui::Window::new("Settings")
                .open(&mut self.window_open)
                .show(ctx, |ui| {
                    ui.checkbox(&mut self.rainbow, "Rainbow text");
                    
                    egui::color_picker::color_picker_color32(ui, &mut self.color, Alpha::Opaque);
                    
                });
        }
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        if self.rainbow{
            //rainbow insert here
        }
        

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
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            //let desired_lenght:usize = 100;
            
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.label)
                    .text_color(self.color)
                    .desired_rows(32));
                });
            });
            
            
            
            egui::warn_if_debug_build(ui);
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
