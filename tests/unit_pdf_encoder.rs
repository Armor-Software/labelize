use image::{Rgba, RgbaImage};
use labelize::DrawerOptions;
use labelize::{encode_pdf, encode_pdf_multi};

#[test]
fn pdf_output_is_non_empty() {
    let img = RgbaImage::from_pixel(100, 100, Rgba([255, 255, 255, 255]));
    let opts = DrawerOptions {
        label_width_mm: 50.0,
        label_height_mm: 50.0,
        dpmm: 8,
        ..Default::default()
    };
    let mut buf = Vec::new();
    encode_pdf(&img, &opts, &mut buf).expect("encode_pdf failed");
    assert!(!buf.is_empty(), "PDF output should be non-empty");
}

#[test]
fn pdf_starts_with_pdf_header() {
    let img = RgbaImage::from_pixel(100, 100, Rgba([255, 255, 255, 255]));
    let opts = DrawerOptions::default();
    let mut buf = Vec::new();
    encode_pdf(&img, &opts, &mut buf).expect("encode_pdf failed");
    let header = std::str::from_utf8(&buf[..5]).unwrap_or("");
    assert_eq!(header, "%PDF-", "PDF should start with %PDF- header");
}

#[test]
fn pdf_contains_mediabox() {
    let img = RgbaImage::from_pixel(100, 100, Rgba([255, 255, 255, 255]));
    let opts = DrawerOptions {
        label_width_mm: 102.0,
        label_height_mm: 152.0,
        dpmm: 8,
        ..Default::default()
    };
    let mut buf = Vec::new();
    encode_pdf(&img, &opts, &mut buf).expect("encode_pdf failed");
    let content = String::from_utf8_lossy(&buf);
    assert!(content.contains("MediaBox"), "PDF should contain MediaBox");
}

#[test]
fn multi_page_pdf_contains_count_for_n_pages() {
    let img = RgbaImage::from_pixel(50, 50, Rgba([255, 255, 255, 255]));
    let imgs = vec![img.clone(), img.clone(), img];
    let opts = DrawerOptions {
        label_width_mm: 20.0,
        label_height_mm: 45.0,
        dpmm: 8,
        ..Default::default()
    };
    let mut buf = Vec::new();
    encode_pdf_multi(&imgs, &opts, &mut buf).expect("encode_pdf_multi failed");

    let content = String::from_utf8_lossy(&buf);
    assert!(
        content.starts_with("%PDF-"),
        "PDF should start with %PDF- header"
    );
    assert!(
        content.contains("/Count 3"),
        "Pages tree should declare Count 3 for a 3-image input"
    );
    // Three /Type /Page entries (one per page) — Pages tree has /Type /Pages.
    let page_occurrences = content.matches("/Type /Page\n").count()
        + content.matches("/Type /Page ").count()
        + content.matches("/Type/Page\n").count()
        + content.matches("/Type/Page ").count();
    // lopdf may serialize differently — at minimum we expect 3 Page dicts to exist.
    // Fall back to a looser check counting MediaBox occurrences (one per page).
    let mediabox_count = content.matches("MediaBox").count();
    assert!(
        page_occurrences >= 3 || mediabox_count >= 3,
        "expected at least 3 page entries, got pages={} mediabox={}",
        page_occurrences,
        mediabox_count
    );
}

#[test]
fn multi_page_single_image_matches_single_page_output_shape() {
    let img = RgbaImage::from_pixel(100, 100, Rgba([255, 255, 255, 255]));
    let opts = DrawerOptions::default();

    let mut single = Vec::new();
    encode_pdf(&img, &opts, &mut single).expect("encode_pdf failed");

    let mut multi = Vec::new();
    encode_pdf_multi(std::slice::from_ref(&img), &opts, &mut multi)
        .expect("encode_pdf_multi failed");

    // Both should be valid PDFs declaring Count 1.
    assert!(single.starts_with(b"%PDF-"));
    assert!(multi.starts_with(b"%PDF-"));
    let single_str = String::from_utf8_lossy(&single);
    let multi_str = String::from_utf8_lossy(&multi);
    assert!(single_str.contains("/Count 1"));
    assert!(multi_str.contains("/Count 1"));
}

#[test]
fn multi_page_pdf_rejects_empty_input() {
    let opts = DrawerOptions::default();
    let mut buf = Vec::new();
    let result = encode_pdf_multi(&[], &opts, &mut buf);
    assert!(result.is_err(), "expected error for empty input");
}
