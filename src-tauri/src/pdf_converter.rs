use lopdf::{content::Content, Document, Object};
use std::io;
use std::path::Path;
use dxf::Drawing;
use dxf::entities::{Entity, Line};

// --- Structures de Données ---

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Transform {
    pub fn identity() -> Self {
        Transform {
            a: 1.0, b: 0.0,
            c: 0.0, d: 1.0,
            e: 0.0, f: 0.0,
        }
    }

    pub fn multiply(&self, other: &Transform) -> Self {
        Transform {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
            e: self.e * other.a + self.f * other.c + other.e,
            f: self.e * other.b + self.f * other.d + other.f,
        }
    }

    pub fn apply(&self, p: Point) -> Point {
        Point {
            x: self.a * p.x + self.c * p.y + self.e,
            y: self.b * p.x + self.d * p.y + self.f,
        }
    }
}

#[derive(Debug)]
pub struct LineEntity {
    pub start: Point,
    pub end: Point,
}

// --- Fonctions Utilitaires ---

fn as_f64(obj: &Object) -> f64 {
    match obj {
        Object::Integer(i) => *i as f64,
        Object::Real(f) => *f as f64,
        _ => 0.0,
    }
}

fn bezier_to_lines(p0: Point, p1: Point, p2: Point, p3: Point, segments: usize) -> Vec<LineEntity> {
    let mut lines = Vec::with_capacity(segments);
    let mut prev_point = p0;

    for i in 1..=segments {
        let t = i as f64 / segments as f64;
        let t2 = t * t;
        let t3 = t2 * t;
        let u = 1.0 - t;
        let u2 = u * u;
        let u3 = u2 * u;

        let x = u3 * p0.x + 3.0 * u2 * t * p1.x + 3.0 * u * t2 * p2.x + t3 * p3.x;
        let y = u3 * p0.y + 3.0 * u2 * t * p1.y + 3.0 * u * t2 * p2.y + t3 * p3.y;

        let current_point = Point { x, y };
        lines.push(LineEntity {
            start: prev_point,
            end: current_point,
        });
        prev_point = current_point;
    }

    lines
}

fn parse_content_stream(
    doc: &Document,
    resources: Option<&lopdf::Dictionary>,
    content_data: &[u8],
    base_ctm: Transform,
    all_lines: &mut Vec<LineEntity>,
) {
    if let Ok(content) = Content::decode(content_data) {
        let mut ctm_stack: Vec<Transform> = Vec::new();
        let mut current_ctm = base_ctm;

        let mut current_point = Point { x: 0.0, y: 0.0 };
        let mut subpath_start = Point { x: 0.0, y: 0.0 };

        for op in &content.operations {
            match op.operator.as_str() {
                "q" => ctm_stack.push(current_ctm),
                "Q" => {
                    if let Some(matrix) = ctm_stack.pop() {
                        current_ctm = matrix;
                    }
                }
                "cm" => {
                    if op.operands.len() == 6 {
                        let new_matrix = Transform {
                            a: as_f64(&op.operands[0]),
                            b: as_f64(&op.operands[1]),
                            c: as_f64(&op.operands[2]),
                            d: as_f64(&op.operands[3]),
                            e: as_f64(&op.operands[4]),
                            f: as_f64(&op.operands[5]),
                        };
                        current_ctm = current_ctm.multiply(&new_matrix);
                    }
                }
                "m" => {
                    if op.operands.len() == 2 {
                        let p = current_ctm.apply(Point {
                            x: as_f64(&op.operands[0]),
                            y: as_f64(&op.operands[1]),
                        });
                        current_point = p;
                        subpath_start = p;
                    }
                }
                "l" => {
                    if op.operands.len() == 2 {
                        let p = current_ctm.apply(Point {
                            x: as_f64(&op.operands[0]),
                            y: as_f64(&op.operands[1]),
                        });
                        all_lines.push(LineEntity { start: current_point, end: p });
                        current_point = p;
                    }
                }
                "c" => {
                    if op.operands.len() == 6 {
                        let p1 = current_ctm.apply(Point { x: as_f64(&op.operands[0]), y: as_f64(&op.operands[1]) });
                        let p2 = current_ctm.apply(Point { x: as_f64(&op.operands[2]), y: as_f64(&op.operands[3]) });
                        let p3 = current_ctm.apply(Point { x: as_f64(&op.operands[4]), y: as_f64(&op.operands[5]) });
                        let curves = bezier_to_lines(current_point, p1, p2, p3, 10);
                        all_lines.extend(curves);
                        current_point = p3;
                    }
                }
                "v" => {
                    if op.operands.len() == 4 {
                        let p2 = current_ctm.apply(Point { x: as_f64(&op.operands[0]), y: as_f64(&op.operands[1]) });
                        let p3 = current_ctm.apply(Point { x: as_f64(&op.operands[2]), y: as_f64(&op.operands[3]) });
                        let curves = bezier_to_lines(current_point, current_point, p2, p3, 10);
                        all_lines.extend(curves);
                        current_point = p3;
                    }
                }
                "y" => {
                    if op.operands.len() == 4 {
                        let p1 = current_ctm.apply(Point { x: as_f64(&op.operands[0]), y: as_f64(&op.operands[1]) });
                        let p3 = current_ctm.apply(Point { x: as_f64(&op.operands[2]), y: as_f64(&op.operands[3]) });
                        let curves = bezier_to_lines(current_point, p1, p3, p3, 10);
                        all_lines.extend(curves);
                        current_point = p3;
                    }
                }
                "h" => {
                    all_lines.push(LineEntity { start: current_point, end: subpath_start });
                    current_point = subpath_start;
                }
                "re" => {
                    if op.operands.len() == 4 {
                        let x = as_f64(&op.operands[0]);
                        let y = as_f64(&op.operands[1]);
                        let w = as_f64(&op.operands[2]);
                        let h = as_f64(&op.operands[3]);

                        let p_ll = current_ctm.apply(Point { x, y });
                        let p_lr = current_ctm.apply(Point { x: x + w, y });
                        let p_ur = current_ctm.apply(Point { x: x + w, y: y + h });
                        let p_ul = current_ctm.apply(Point { x, y: y + h });

                        subpath_start = p_ll;
                        all_lines.push(LineEntity { start: p_ll, end: p_lr });
                        all_lines.push(LineEntity { start: p_lr, end: p_ur });
                        all_lines.push(LineEntity { start: p_ur, end: p_ul });
                        all_lines.push(LineEntity { start: p_ul, end: p_ll });
                        current_point = p_ll;
                    }
                }
                "Do" => {
                    if let Some(res) = resources {
                        if let Ok(xobjects) = res.get(b"XObject").and_then(|o| o.as_dict()) {
                            if let Object::Name(name) = &op.operands[0] {
                                if let Ok(Object::Reference(object_id)) = xobjects.get(name) {
                                    if let Ok(stream) = doc.get_object(*object_id).and_then(|o| o.as_stream()) {
                                        if let Ok(subtype) = stream.dict.get(b"Subtype").and_then(|o| o.as_name()) {
                                            if subtype == b"Form" {
                                                let mut form_ctm = current_ctm;
                                                if let Ok(matrix_array) = stream.dict.get(b"Matrix").and_then(|o| o.as_array()) {
                                                    if matrix_array.len() == 6 {
                                                        let form_matrix = Transform {
                                                            a: as_f64(&matrix_array[0]),
                                                            b: as_f64(&matrix_array[1]),
                                                            c: as_f64(&matrix_array[2]),
                                                            d: as_f64(&matrix_array[3]),
                                                            e: as_f64(&matrix_array[4]),
                                                            f: as_f64(&matrix_array[5]),
                                                        };
                                                        form_ctm = form_ctm.multiply(&form_matrix);
                                                    }
                                                }

                                                let mut form_resources = resources;
                                                if let Ok(form_res) = stream.dict.get(b"Resources").and_then(|o| o.as_dict()) {
                                                    form_resources = Some(form_res);
                                                }
                                                
                                                if let Ok(form_data) = stream.decompressed_content() {
                                                    parse_content_stream(doc, form_resources, &form_data, form_ctm, all_lines);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn extract_pdf_paths(doc: &Document) -> Vec<LineEntity> {
    let mut all_lines = Vec::new();
    let mut current_offset_x = 0.0;
    let page_margin = 200.0;

    for (_page_idx, (_, page_id)) in doc.get_pages().iter().enumerate() {
        let mut page_width = 1000.0;
        if let Ok(page_dict) = doc.get_dictionary(*page_id) {
            if let Ok(media_box) = page_dict.get(b"MediaBox").and_then(|o| o.as_array()) {
                if media_box.len() >= 4 {
                    let llx = as_f64(&media_box[0]);
                    let urx = as_f64(&media_box[2]);
                    page_width = (urx - llx).abs();
                }
            }
        }

        let page_offset_x = current_offset_x;
        current_offset_x += page_width + page_margin;

        if let Ok(content_data) = doc.get_page_content(*page_id) {
            let page_dict = doc.get_dictionary(*page_id).ok();
            let mut resources = None;
            if let Some(dict) = page_dict {
                if let Ok(res) = dict.get(b"Resources").and_then(|o| o.as_dict()) {
                    resources = Some(res);
                }
            }
            
            let mut base_page_ctm = Transform::identity();
            base_page_ctm.e = page_offset_x;

            parse_content_stream(
                doc,
                resources,
                &content_data,
                base_page_ctm,
                &mut all_lines,
            );
        }
    }

    all_lines
}

pub fn extract_images(doc: &Document, output_base_path: &Path) {
    let base_name = output_base_path.file_stem().and_then(|s| s.to_str()).unwrap_or("document");
    let parent_dir = output_base_path.parent().unwrap_or(Path::new(""));
    let mut image_counter = 1;

    for (_object_id, object) in doc.objects.iter() {
        if let Ok(stream) = object.as_stream() {
            if let Ok(subtype) = stream.dict.get(b"Subtype").and_then(|o| o.as_name()) {
                if subtype == b"Image" {
                    if let Ok(filter) = stream.dict.get(b"Filter") {
                        let is_jpeg = match filter {
                            Object::Name(name) if name == b"DCTDecode" => true,
                            Object::Array(arr) if arr.iter().any(|o| matches!(o, Object::Name(n) if n == b"DCTDecode")) => true,
                            _ => false,
                        };

                        if is_jpeg {
                            let file_name = format!("{}_img_{}.jpg", base_name, image_counter);
                            let file_path = parent_dir.join(&file_name);
                            if let Ok(mut file) = std::fs::File::create(&file_path) {
                                use std::io::Write;
                                let _ = file.write_all(&stream.content);
                            }
                            image_counter += 1;
                            continue;
                        }
                    }

                    if let Ok(width) = stream.dict.get(b"Width").and_then(|o| o.as_i64()) {
                        if let Ok(height) = stream.dict.get(b"Height").and_then(|o| o.as_i64()) {
                            if let Ok(decompressed) = stream.decompressed_content() {
                                let color_space = stream.dict.get(b"ColorSpace").and_then(|o| o.as_name()).unwrap_or(b"DeviceRGB");
                                let bits_per_component = stream.dict.get(b"BitsPerComponent").and_then(|o| o.as_i64()).unwrap_or(8);

                                if bits_per_component == 8 {
                                    let is_rgb = color_space == b"DeviceRGB" || (decompressed.len() as i64 == width * height * 3);
                                    let is_gray = color_space == b"DeviceGray" || (decompressed.len() as i64 == width * height);

                                    let mut image_buffer = None;

                                    if is_rgb && decompressed.len() as i64 >= width * height * 3 {
                                        image_buffer = image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(
                                            width as u32,
                                            height as u32,
                                            decompressed[0..(width * height * 3) as usize].to_vec(),
                                        ).map(|img| image::DynamicImage::ImageRgb8(img));
                                    } else if is_gray && decompressed.len() as i64 >= width * height {
                                        image_buffer = image::ImageBuffer::<image::Luma<u8>, _>::from_raw(
                                            width as u32,
                                            height as u32,
                                            decompressed[0..(width * height) as usize].to_vec(),
                                        ).map(|img| image::DynamicImage::ImageLuma8(img));
                                    }

                                    if let Some(img) = image_buffer {
                                        let file_name = format!("{}_img_{}.png", base_name, image_counter);
                                        let file_path = parent_dir.join(&file_name);
                                        let _ = img.save(&file_path);
                                        image_counter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn generate_dxf(lines: &[LineEntity], output_path: &str, scale_factor: f64, _unit: &str) -> io::Result<()> {
    let mut drawing = Drawing::new();
    // Revert to R12 - it's the most compatible version and lacks complex header tags that can cause corruption if not perfectly formed
    drawing.header.version = dxf::enums::AcadVersion::R12;
    
    // Convert PDF points (1/72 inch) to millimeters (25.4 mm / 72 pt)
    let pt_to_mm = 25.4 / 72.0;
    let final_scale = scale_factor * pt_to_mm;

    for line in lines {
        if (line.start.x - line.end.x).abs() > 0.001 || (line.start.y - line.end.y).abs() > 0.001 {
            let p1 = dxf::Point::new(line.start.x * final_scale, line.start.y * final_scale, 0.0);
            let p2 = dxf::Point::new(line.end.x * final_scale, line.end.y * final_scale, 0.0);
            
            let dxf_line = Line::new(p1, p2);
            let mut entity = Entity::new(dxf::entities::EntityType::Line(dxf_line));
            entity.common.layer = String::from("0");
            drawing.add_entity(entity);
        }
    }

    match drawing.save_file(output_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("Failed to generate DXF: {:?}", e))),
    }
}
