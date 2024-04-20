#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{
    egui::{self, load::SizedTexture, Context},
    emath::TSTransform,
};
use nimage::nsif::{export::export_to_jpeg, field::Value, NSIF};
use std::{env, fs, path::PathBuf, str::FromStr};

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
    nsif: Option<NSIF>,
    texture: Option<egui::TextureHandle>,
    initial_path: Option<PathBuf>,
    transform: TSTransform,
}
impl Default for NImageViewer {
    fn default() -> Self {
        NImageViewer {
            nsif: None,
            texture: None,
            initial_path: env::args()
                .collect::<Vec<String>>()
                .get(1)
                .and_then(|s| PathBuf::from_str(s.as_str()).ok()),
            transform: TSTransform::default(),
        }
    }
}
impl eframe::App for NImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(path) = &self.initial_path.take() {
            self.load_nsif(path, ctx);
        }
        egui::CentralPanel::default().show(ctx, |_| {
            egui::TopBottomPanel::top("top-panel").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open").clicked() {
                            ui.close_menu();
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("NSIF files", &vec!["nsif", "nitf"])
                                .pick_file()
                            {
                                self.load_nsif(&path, ctx);
                            }
                        }
                    });
                    ui.add_enabled_ui(self.nsif.is_some(), |ui| {
                        ui.menu_button("Export", |ui| {
                            if let Some(image) = &self.nsif {
                                for (i, &ref image_segment) in
                                    &mut image.image_segments.iter().enumerate()
                                {
                                    let button =
                                        ui.button(format!("Export Image Segment {}", i + 1));
                                    if button.clicked() {
                                        ui.close_menu();
                                        if let Some(path) = rfd::FileDialog::new().save_file() {
                                            export_to_jpeg(&image_segment, path).unwrap();
                                        }
                                    }
                                }
                            }
                        })
                    });
                })
            });
            egui::SidePanel::left("details-panel").show(ctx, |ui| {
                ui.set_width(400.0);
                egui::ScrollArea::both().show(ui, |ui| {
                    if let Some(image) = &self.nsif {
                        egui::Grid::new("details-table").show(ui, |ui| {
                            for (header, fields) in image.fields() {
                                egui::CollapsingHeader::new(&header).show(ui, |ui| {
                                    egui::Grid::new(&header).striped(true).show(ui, |ui| {
                                        for field in fields {
                                            let value = match &field.value {
                                                Value::SingleAlphanumeric(v) => v.value.clone(),
                                                Value::SingleNumeric(v) => v.value.clone(),
                                                Value::MultipleAlphanumeric(vs) => vs
                                                    .iter()
                                                    .map(|v| v.value.clone())
                                                    .filter(|v| !v.trim().is_empty())
                                                    .collect::<Vec<String>>()
                                                    .join(","),
                                                Value::MultipleNumeric(vs) => vs
                                                    .iter()
                                                    .map(|v| v.value.clone())
                                                    .filter(|v| !v.trim().is_empty())
                                                    .collect::<Vec<String>>()
                                                    .join(","),
                                                Value::NestedAlphaNumeric(vss) => vss
                                                    .iter()
                                                    .map(|vs| {
                                                        vs.iter()
                                                            .map(|v| v.value.clone())
                                                            .filter(|v| !v.trim().is_empty())
                                                            .collect::<Vec<String>>()
                                                            .join(",")
                                                    })
                                                    .filter(|v| !v.trim().is_empty())
                                                    .collect::<Vec<String>>()
                                                    .join(";"),
                                                Value::NestedNumeric(vss) => vss
                                                    .iter()
                                                    .map(|vs| {
                                                        vs.iter()
                                                            .map(|v| v.value.clone())
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
                                });
                                ui.end_row();
                            }
                        });
                    }
                });
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                egui::ScrollArea::both()
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                    .show(ui, |ui| {
                        if let Some(_) = &self.nsif {
                            let image_response = ui.add(
                                // TODO padding?, auto-scale, panning
                                egui::Image::from_texture(SizedTexture::from_handle(
                                    &self.texture.clone().unwrap(), // TODO need to clone?
                                ))
                                .fit_to_original_size(self.transform.scaling), // TODO smoother
                                                                               // scrolling? zoom on cursor position
                            );
                            if image_response.contains_pointer() {
                                let zoom_delta = ctx.input(|i| i.zoom_delta());
                                self.transform =
                                    self.transform * TSTransform::from_scaling(zoom_delta);
                            }
                        }
                    });
            });

            egui::TopBottomPanel::bottom("bottom-panel").show(ctx, |ui| {
                if let Some(_) = &self.nsif {
                    ui.with_layout(
                        egui::Layout::left_to_right(egui::Align::Center).with_cross_justify(true),
                        |ui| {
                            if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                                ui.label(format!("({},{})", pointer.x, pointer.y));
                                // TODO world
                                // coordinates,
                                // bounds check
                            }
                        },
                    );
                }
            })
        });
    }
}

impl NImageViewer {
    fn load_nsif(&mut self, path: &PathBuf, ctx: &Context) {
        if let Ok(file) = fs::File::open(path) {
            if let Ok(image) = NSIF::parse(&file) {
                let image_segment = image.image_segments.get(0).unwrap();
                let (height, width) = image_segment.dimensions();
                self.texture = Some(ctx.load_texture(
                    "image-segment",
                    egui::ColorImage::from_rgb([width as _, height as _], &image_segment.as_rgb()),
                    Default::default(),
                ));
                self.nsif = Some(image);
            } else {
                eprintln!("Failed to parse given file")
            }
        }
    }
}
