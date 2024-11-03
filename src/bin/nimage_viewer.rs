#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{
    egui::{self, load::SizedTexture, scroll_area::ScrollBarVisibility, Context, Response, Window},
    emath::TSTransform,
};
use nimage::nsif::{export::export_to_jpeg, field::Value, NSIF};
use std::{env, fs, path::PathBuf, str::FromStr};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_fullscreen(true),
        ..Default::default()
    };
    eframe::run_native(
        "NImage Viewer",
        options,
        Box::new(|_| Ok(Box::<NImageViewer>::default())),
    )
}

struct NImageViewer {
    nsif: Option<NSIF>,
    texture: Option<egui::TextureHandle>,
    initial_path: Option<PathBuf>,
    transform: TSTransform,
    image_response: Option<Response>,
    selected_image_segment_index: Option<usize>,
    image_was_updated: bool,
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
            image_response: None,
            selected_image_segment_index: None,
            image_was_updated: false,
        }
    }
}
impl eframe::App for NImageViewer {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if let Some(path) = &self.initial_path.take() {
            self.load_nsif(path, ctx);
        }
        Window::new("Menu").default_width(600.0).show(ctx, |ui| {
            ui.heading("File");
            ui.horizontal(|ui| {
                if ui.button("Open File").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("NSIF files", &vec!["nsif", "nitf"])
                        .pick_file()
                    {
                        self.load_nsif(&path, ctx);
                    }
                }
                if let Some(current_segment) = self.selected_image_segment_index {
                    if ui.button("Export Current Segment").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
                            let image = self.nsif.as_ref().unwrap();
                            export_to_jpeg(
                                &image.image_segments.get(current_segment).unwrap(),
                                path,
                            )
                            .unwrap()
                        }
                    }
                }
            });
            ui.allocate_space(egui::Vec2::new(0.0, 2.0));
            ui.heading("Show");
            if let Some(image) = &self.nsif {
                let previous_selection = self.selected_image_segment_index;
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.selected_image_segment_index, None, "None");
                    for (i, _) in image.image_segments.iter().enumerate() {
                        ui.selectable_value(
                            &mut self.selected_image_segment_index,
                            Some(i),
                            format!("Image Segment {}", i + 1),
                        );
                    }
                });
                if self.selected_image_segment_index != previous_selection || self.image_was_updated
                {
                    self.update_image_segment_display(ctx);
                    self.image_was_updated = false;
                }
            }
            ui.allocate_space(egui::Vec2::new(0.0, 2.0));
            ui.heading("Details");
            egui::ScrollArea::both()
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                .auto_shrink(false)
                .show(ui, |ui| {
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
            let (id, rect) = ui.allocate_space(ui.available_size());
            let response = ui.interact(rect, id, egui::Sense::click_and_drag());
            let transform =
                TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;
            if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                if response.hovered() {
                    let pointer_in_layer = transform.inverse() * pointer;
                    let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                    let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta);
                    self.transform = self.transform
                        * TSTransform::from_translation(pointer_in_layer.to_vec2())
                        * TSTransform::from_scaling(zoom_delta)
                        * TSTransform::from_translation(-pointer_in_layer.to_vec2());
                    self.transform = TSTransform::from_translation(pan_delta) * self.transform;
                }
                if let Some(r) = &self.image_response {
                    if r.hovered() {
                        let pointer_in_layer = transform.inverse() * pointer;
                        let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                        let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta);
                        self.transform = self.transform
                            * TSTransform::from_translation(pointer_in_layer.to_vec2())
                            * TSTransform::from_scaling(zoom_delta)
                            * TSTransform::from_translation(-pointer_in_layer.to_vec2());
                        self.transform = TSTransform::from_translation(pan_delta) * self.transform;
                    }
                }
            }

            let window_layer = ui.layer_id();
            let id = egui::Area::new(id.with(("subarea", 1)))
                .order(egui::Order::Background)
                .show(ui.ctx(), |ui| {
                    ui.set_clip_rect(transform.inverse() * rect);
                    egui::Frame::default()
                        .stroke(ui.ctx().style().visuals.window_stroke)
                        .fill(ui.style().visuals.panel_fill)
                        .show(ui, |ui| {
                            self.image_response = self.texture.as_ref().and_then(|texture| {
                                Some(
                                    ui.add(
                                        egui::Image::from_texture(SizedTexture::from_handle(
                                            texture,
                                        ))
                                        .fit_to_exact_size(rect.size()),
                                    ),
                                )
                            })
                        });
                })
                .response
                .layer_id;
            ui.ctx().set_transform_layer(id, transform);
            ui.ctx().set_sublayer(window_layer, id);
        });
    }
}

impl NImageViewer {
    fn load_nsif(&mut self, path: &PathBuf, ctx: &Context) {
        if let Ok(file) = fs::File::open(path) {
            if let Ok(image) = NSIF::parse(&file) {
                if image.image_segments.len() > 0 {
                    self.selected_image_segment_index = Some(0);
                } else {
                    self.selected_image_segment_index = None;
                }
                self.nsif = Some(image);
            } else {
                self.nsif = None;
                self.texture = None;
                eprintln!("Failed to parse given file"); // TODO error popup?
            }
            self.image_was_updated = true
        }
    }

    fn update_image_segment_display(&mut self, ctx: &Context) {
        let image = self.nsif.as_ref().unwrap();
        if let Some(image_segment) = image
            .image_segments
            .get(self.selected_image_segment_index.unwrap())
        {
            let (height, width) = image_segment.dimensions();
            self.texture = image_segment.as_rgb().ok().map(|rgb| {
                ctx.load_texture(
                    "image-segment",
                    egui::ColorImage::from_rgb([width as _, height as _], &rgb),
                    Default::default(),
                )
            })
        }
    }
}
