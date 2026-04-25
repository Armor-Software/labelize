use crate::elements::drawer_options::DrawerOptions;
use crate::error::LabelizeError;
use image::RgbaImage;
use std::io::Write;

/// Encodes a single label image as a single-page PDF.
///
/// Backwards-compatible wrapper around [`encode_pdf_multi`].
pub fn encode_pdf(
    img: &RgbaImage,
    opts: &DrawerOptions,
    w: &mut impl Write,
) -> Result<(), LabelizeError> {
    encode_pdf_multi(std::slice::from_ref(img), opts, w)
}

/// Encodes multiple label images as a multi-page PDF (one page per image).
///
/// Each image renders at the same `label_width_mm` × `label_height_mm` page size.
/// Returns an error if `imgs` is empty.
pub fn encode_pdf_multi(
    imgs: &[RgbaImage],
    opts: &DrawerOptions,
    w: &mut impl Write,
) -> Result<(), LabelizeError> {
    if imgs.is_empty() {
        return Err(LabelizeError::Encode(
            "encode_pdf_multi requires at least one image".to_string(),
        ));
    }

    use lopdf::dictionary;
    use lopdf::{Document, Object, Stream};

    let width_pt = opts.label_width_mm * 2.834645669; // mm to pt
    let height_pt = opts.label_height_mm * 2.834645669;

    let mut doc = Document::with_version("1.4");
    let pages_id = doc.new_object_id();
    let mut page_refs: Vec<Object> = Vec::with_capacity(imgs.len());

    for (i, img) in imgs.iter().enumerate() {
        let img_stream = Stream::new(
            dictionary! {
                "Type" => "XObject",
                "Subtype" => "Image",
                "Width" => img.width() as i64,
                "Height" => img.height() as i64,
                "ColorSpace" => "DeviceGray",
                "BitsPerComponent" => 8,
                "Filter" => "FlateDecode",
            },
            compress_gray(img),
        );
        let img_id = doc.add_object(img_stream);

        let img_name = format!("Im{}", i + 1);
        let resources = dictionary! {
            "XObject" => dictionary! {
                img_name.as_str() => img_id,
            },
        };

        let content = format!("q {} 0 0 {} 0 0 cm /{} Do Q", width_pt, height_pt, img_name);
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.into_bytes()));

        let page_id = doc.new_object_id();
        let page = dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "MediaBox" => vec![
                0.into(),
                0.into(),
                Object::Real(width_pt as f32),
                Object::Real(height_pt as f32),
            ],
            "Contents" => content_id,
            "Resources" => resources,
        };
        doc.objects.insert(page_id, Object::Dictionary(page));
        page_refs.push(page_id.into());
    }

    let count = page_refs.len() as i64;
    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => page_refs,
        "Count" => count,
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });
    doc.trailer.set("Root", catalog_id);

    let mut buf = Vec::new();
    doc.save_to(&mut buf)
        .map_err(|e| LabelizeError::Encode(format!("PDF save error: {}", e)))?;
    w.write_all(&buf)?;

    Ok(())
}

fn compress_gray(img: &RgbaImage) -> Vec<u8> {
    use flate2::write::ZlibEncoder;
    use flate2::Compression;

    let (w, h) = img.dimensions();
    let mut raw = Vec::with_capacity((w * h) as usize);
    for y in 0..h {
        for x in 0..w {
            let p = img.get_pixel(x, y);
            let val = if p[0] > 128 { 255u8 } else { 0u8 };
            raw.push(val);
        }
    }

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    std::io::Write::write_all(&mut encoder, &raw).unwrap();
    encoder.finish().unwrap()
}
