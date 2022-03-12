use core::ops;
use chrono::{Datelike, Duration};
use eframe::{
    egui::{self, Color32},
    epi,
};
use egui_datepicker::*;

struct ExampleApp {
    date: NaiveDateTime,
    range: ops::RangeInclusive<NaiveDateTime>,
}

impl Default for ExampleApp {

    fn default() -> Self {
        let date = chrono::Utc::now().naive_utc();
        let range = (date - Duration::weeks(2))..=(date + Duration::weeks(1));
        Self { date, range }
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "Datepicker example"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        // ctx.set_debug_on_hover(true);
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("exaamples_grid").show(ui, |ui| {
                ui.label("Default");
                ui.add(DatePicker::<ops::Range<NaiveDateTime>>::new("default", &mut self.date));
                ui.end_row();
                ui.label("Sunday first");
                ui.add(DatePicker::<ops::Range<NaiveDateTime>>::new("sundayfirst", &mut self.date).sunday_first(true));
                ui.end_row();
                ui.label("Movable popup");
                ui.add(DatePicker::<ops::Range<NaiveDateTime>>::new("movable", &mut self.date).movable(true));
                ui.end_row();
                ui.label("Different format");
                ui.add(DatePicker::<ops::Range<NaiveDateTime>>::new("differentformat", &mut self.date).date_format(&"%d/%m/%Y"));
                ui.end_row();
                ui.label("Disable weekend highlight");
                ui.add(
                    DatePicker::<ops::Range<NaiveDateTime>>::new("noweekendhighlight", &mut self.date).highlight_weekend(false),
                );
                ui.end_row();
                ui.label("Different weekend color");
                ui.add(
                    DatePicker::<ops::Range<NaiveDateTime>>::new("differentweekendcolor", &mut self.date)
                        .highlight_weekend_color(Color32::from_rgb(0, 196, 0)),
                );
                ui.end_row();
                ui.label("Different weekend days, i.e. holidays, Christmas, etc");
                ui.add(
                    DatePicker::<ops::Range<NaiveDateTime>>::new("differentweekenddays", &mut self.date)
                        .weekend_days(|date| date.day() % 2 == 0),
                );
                ui.end_row();
                ui.label("Restrict the range of input values.");
                ui.add(
                    DatePicker::new("restrictrange", &mut self.date).restrict_range(&self.range),
                );
                ui.end_row();
                ui.label("Align to centre");
                ui.add(
                    DatePicker::<ops::Range<NaiveDateTime>>::new("aligncentre", &mut self.date)
                        .placement(egui::Align2::CENTER_BOTTOM),
                );
                ui.end_row();
                ui.label("Offset from default position");
                ui.add(
                    DatePicker::<ops::Range<NaiveDateTime>>::new("offset", &mut self.date)
                        .position_offset([12., 13.]),
                );
                ui.end_row();
                ui.label("Offset from set position");
                ui.add(
                    DatePicker::<ops::Range<NaiveDateTime>>::new("alignandoffset", &mut self.date)
                        .placement(egui::Align2::LEFT_BOTTOM)
                        .position_offset([10., -10.]),
                );
            });
        });
    }
}

fn main() {
    let app = ExampleApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
