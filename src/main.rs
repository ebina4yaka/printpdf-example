fn main() {
    use printpdf::*;
    use std::fs::File;
    use std::io::BufWriter;

    let (mut doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");

    doc = doc.with_conformance(PdfConformance::Custom(CustomPdfConformance {
        requires_icc_profile: false,
        requires_xmp_metadata: false,
        ..Default::default()
    }));

    let current_layer = doc.get_page(page1).get_layer(layer1);

    let text = "Lorem ipsum";

    let font = doc.add_builtin_font(BuiltinFont::TimesBoldItalic).unwrap();
    current_layer.use_text(text, 48.0 as i64, Mm(10.0), Mm(200.0), &font);

    doc.save(&mut BufWriter::new(
        File::create("out/test_working.pdf").unwrap(),
    ))
    .unwrap();
}
