use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Strip Line Numbers",
        options,
        Box::new(|_cc| Ok(Box::<StripLinesApp>::default())),
    )
}

struct StripLinesApp {
    input_text: String,
    output_text: String,
}

impl Default for StripLinesApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            output_text: String::new(),
        }
    }
}

impl eframe::App for StripLinesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Strip Line Numbers");
            ui.add_space(10.0);
            
            // Input section
            ui.label("Input (paste code with line numbers):");
            let input_response = ui.add(
                egui::TextEdit::multiline(&mut self.input_text)
                    .desired_width(f32::INFINITY)
                    .desired_rows(12)
                    .font(egui::TextStyle::Monospace)
            );
            
            // Auto-process when text changes
            if input_response.changed() {
                self.output_text = strip_line_numbers(&self.input_text);
            }
            
            ui.add_space(10.0);
            
            // Output section
            ui.horizontal(|ui| {
                ui.label("Output:");
                if ui.button("Copy to Clipboard").clicked() {
                    ui.output_mut(|o| o.copied_text = self.output_text.clone());
                }
            });
            
            ui.add(
                egui::TextEdit::multiline(&mut self.output_text)
                    .desired_width(f32::INFINITY)
                    .desired_rows(12)
                    .font(egui::TextStyle::Monospace)
            );
        });
    }
}

fn strip_line_numbers(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let code_start = line
                .chars()
                .position(|c| !c.is_ascii_digit())
                .unwrap_or(line.len());
            &line[code_start..]
        })
        .collect::<Vec<_>>()
        .join("\n")
}
