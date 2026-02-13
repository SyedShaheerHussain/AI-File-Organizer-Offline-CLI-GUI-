use eframe::egui;
use crate::constants::APP_NAME;

pub fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}

struct App {
    path: String,
    logs: Vec<String>,
    dry_run: bool,
    use_ai: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            path: "".to_string(),
            logs: vec!["Welcome to AI File Organizer".to_string()],
            dry_run: true,
            use_ai: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(APP_NAME);
            
            ui.horizontal(|ui| {
                ui.label("Folder Path:");
                ui.text_edit_singleline(&mut self.path);
                if ui.button("Browse...").clicked() {
                    // Path picker would go here
                }
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.dry_run, "Dry Run");
                ui.checkbox(&mut self.use_ai, "Use AI");
            });

            ui.horizontal(|ui| {
                if ui.button("Scan").clicked() {
                    self.logs.push(format!("Scanning {:?}", self.path));
                }
                if ui.button("Organize").clicked() {
                    self.logs.push(format!("Organizing {:?}", self.path));
                }
                if ui.button("Undo").clicked() {
                    self.logs.push("Undoing last operation...".to_string());
                }
            });

            ui.separator();
            ui.label("Logs:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for log in &self.logs {
                    ui.label(log);
                }
            });
        });
    }
}
