#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs::{self};

use eframe::egui::{self, load::SizedTexture};
use nimage::nsif::{export::export_to_jpeg, field::FieldValue, parse_string, NSIF};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };
    eframe::run_native(
        "NImage Viewer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<NImageViewer>::default()
        }),
    )
}

struct NImageViewer {
    zoom: f32,
    nsif: Option<NSIF>,
    texture: Option<egui::TextureHandle>,
}
impl Default for NImageViewer {
    fn default() -> Self {
        NImageViewer {
            zoom: 1.0,
            nsif: None,
            texture: None,
        }
    }
}
impl eframe::App for NImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_| {
            egui::TopBottomPanel::top("top-panel").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("NSIF files", &vec!["nsif"])
                                .pick_file()
                            {
                                if let Ok(file) = fs::File::open(path) {
                                    if let Ok(image) = NSIF::parse(&file) {
                                        let image_segment = image.image_segments.get(0).unwrap();
                                        let (height, width) = image_segment.dimensions();
                                        self.texture.get_or_insert(ctx.load_texture(
                                            "image-segment",
                                            egui::ColorImage::from_rgb(
                                                [width as _, height as _],
                                                &image_segment.data,
                                            ),
                                            Default::default(),
                                        ));
                                        self.nsif = Some(image);
                                        return;
                                    }
                                    eprintln!("Failed to parse given file")
                                }
                            }
                        }
                    });
                    ui.menu_button("Export", |ui| {
                        ui.add_enabled_ui(self.nsif.is_some(), |ui| {
                            let button = ui.button("Export Image Segment");
                            if button.clicked() {
                                if let Some(path) = rfd::FileDialog::new().save_file() {
                                    if let Some(image) = &self.nsif {
                                        let image_segment = image.image_segments.get(0).unwrap();
                                        export_to_jpeg(&image_segment, path).unwrap();
                                    }
                                }
                            }
                        })
                    });
                })
            });
            egui::SidePanel::left("details-panel").show(ctx, |ui| {
                egui::ScrollArea::both()
                    .min_scrolled_width(500.0)
                    .show(ui, |ui| {
                        if let Some(image) = &self.nsif {
                            egui::Grid::new("details-table")
                                .striped(true)
                                .show(ui, |ui| {
                                    for field in image.fields() {
                                        let value = match &field.value {
                                            FieldValue::Single(v) => parse_string(&v).unwrap(),
                                            FieldValue::Multiple(vs) => vs
                                                .iter()
                                                .map(|v| parse_string(&v).unwrap_or(String::new()))
                                                .filter(|v| !v.trim().is_empty())
                                                .collect::<Vec<String>>()
                                                .join(","),
                                            FieldValue::Nested(vss) => vss
                                                .iter()
                                                .map(|vs| {
                                                    vs.iter()
                                                        .map(|v| {
                                                            parse_string(&v)
                                                                .unwrap_or(String::new())
                                                        })
                                                        .filter(|v| !v.trim().is_empty())
                                                        .collect::<Vec<String>>()
                                                        .join(",")
                                                })
                                                .filter(|v| !v.trim().is_empty())
                                                .collect::<Vec<String>>()
                                                .join(";"),
                                        };
                                        ui.label(&field.name);
                                        ui.label(value);
                                        ui.end_row();
                                    }
                                });
                        }
                    })
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::both()
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                    .show(ui, |ui| {
                        if let Some(_) = &self.nsif {
                            ui.add(
                                egui::Image::from_texture(SizedTexture::from_handle(
                                    &self.texture.clone().unwrap(), // TODO need to clone?
                                ))
                                .fit_to_original_size(self.zoom),
                            );
                        }
                    });
            });
            egui::TopBottomPanel::bottom("bottom-panel").show(ctx, |ui| {
                if let Some(_) = &self.nsif {
                    ui.with_layout(
                        egui::Layout::left_to_right(egui::Align::Center).with_cross_justify(true),
                        |ui| {
                            ui.style_mut().spacing.slider_width = ui.available_width() - 45.0;
                            ui.add(egui::Slider::new(&mut self.zoom, 0.1..=5.0));
                        },
                    );
                }
            })
        });
    }
}
