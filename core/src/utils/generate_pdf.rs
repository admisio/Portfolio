use image::DynamicImage;
use pdfium_render::prelude::{PdfDocument, Pdfium, PdfiumError, PdfFont, PdfPoints, PdfPageTextObject, PdfColor, PdfPageObjectCommon, PdfPageImageObject, PdfPageObjectsCommon};
use qrcode_generator::QrCodeEcc;

use crate::error::ServiceError;


struct Position {
    pub x: PdfPoints,
    pub y: PdfPoints,
}
impl Position {
    pub const fn new(x: PdfPoints, y: PdfPoints) -> Self {
        Self { x, y }
    }
}

const BASE_FONT_SIZE: PdfPoints = PdfPoints::new(24.0);
const QR_CODE_SIZE: usize = 200;
const APPLICATION_ID_POSITION: Position = Position::new(
    PdfPoints::new(400.0),
    PdfPoints::new(700.0)
); 
const PASSWORD_POSITION: Position = Position::new(
    PdfPoints::new(400.0),
    PdfPoints::new(660.0)
);
const PERSONAL_ID_POSITION: Position = Position::new(
    PdfPoints::new(400.0),
    PdfPoints::new(620.0)
);
const QR_POSITION: Position = Position::new(
    PdfPoints::new(355.0),
    PdfPoints::new(120.0)
);

type ApplicationObject<'a> = PdfPageTextObject<'a>;
type QrCodeObject<'a> = PdfPageImageObject<'a>;
type PassswordObject<'a> = PdfPageTextObject<'a>;
type PersonalIdObject<'a> = PdfPageTextObject<'a>;

pub struct LoginDocument;

impl LoginDocument {
    pub fn generate(application_id: i32, personal_id_number: &str, password: &str) -> Result<Vec<u8>, ServiceError> {
        Self::pdfium_generate(
            application_id,
            personal_id_number,
            password
        )
            .map_err(|e| {
                eprintln!("Pdf Error: {:?}", e);
                ServiceError::PdfError
            })
    }

    fn pdfium_generate(application_id: i32, personal_id_number: &str, password: &str) -> Result<Vec<u8>, PdfiumError> {
        // take libpdfium path from env var
        dotenv::dotenv().ok();
        let pdfium_bindings = match std::env::var("LIBPDFIUM_PATH") {
            Ok(path) => Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(path.as_str()))?,
            Err(_) => Pdfium::bind_to_system_library()?,
        };
        let pdfium = Pdfium::new(pdfium_bindings);


        let template_path = std::env::var("TEMPLATE_PATH").unwrap();
        let document = pdfium.load_pdf_from_file(&template_path, None)?;

        let mut page = document.pages().get(0)?;

        let application_object = Self::gen_application_object_with_pos(&document, application_id)?;
        let password_object = Self::gen_password_object_with_pos(&document, personal_id_number)?;
        let qr_code_object = Self::gen_qr_code_object(&document, "TODO: url")?;
        let personal_id_object = Self::gen_personal_id_object_with_pos(&document, password)?;

        page.objects_mut().add_text_object(application_object)?;
        page.objects_mut().add_text_object(password_object)?;
        page.objects_mut().add_image_object(qr_code_object)?;
        page.objects_mut().add_text_object(personal_id_object)?;
        
        document.save_to_bytes()
    }

    fn gen_application_object_with_pos<'a>(document: &'a PdfDocument<'a>, application_id: i32) -> Result<ApplicationObject<'a>, PdfiumError> {
        let mut object = Self::new_text_object(document, &application_id.to_string())?;
        object.translate(APPLICATION_ID_POSITION.x, APPLICATION_ID_POSITION.y)?;

        Ok(object)
    }

    fn gen_personal_id_object_with_pos<'a>(document: &'a PdfDocument<'a>, password: &str) -> Result<PersonalIdObject<'a>, PdfiumError> {
        let mut object = Self::new_text_object(document, password)?;
        object.translate(PERSONAL_ID_POSITION.x, PERSONAL_ID_POSITION.y)?;

        Ok(object)
    }

    fn gen_password_object_with_pos<'a>(document: &'a PdfDocument<'a>, password: &str) -> Result<PassswordObject<'a>, PdfiumError> {
        let mut object = Self::new_text_object(document, password)?;
        object.translate(PASSWORD_POSITION.x, PASSWORD_POSITION.y)?;

        Ok(object)
    }


    fn new_text_object<'a>(document: &'a PdfDocument<'a>, content: &str) -> Result<PdfPageTextObject<'a>, PdfiumError> {
        let font = PdfFont::courier(&document);
        let mut object = PdfPageTextObject::new(
            &document,
            content,
            &font,
            BASE_FONT_SIZE,
        )?;
        object.set_fill_color(PdfColor::new(0, 0, 0, 255))?;

        Ok(object)
    }

    fn gen_qr_code_object<'a>(document: &'a PdfDocument<'a>, url: &str) -> Result<QrCodeObject<'a>, PdfiumError> {
        let qr = qrcode_generator::to_png_to_vec(url, QrCodeEcc::Low, QR_CODE_SIZE).unwrap();
        let rgb_image = image::load_from_memory(&qr).unwrap().to_rgb8();
        let image = DynamicImage::ImageRgb8(rgb_image);

        let mut object = PdfPageImageObject::new_with_size(
            &document,
            &image,
            PdfPoints { value: image.width() as f32 },
            PdfPoints { value: image.height() as f32 },
        )?;
        object.translate(QR_POSITION.x, QR_POSITION.y)?;

        Ok(object)
    }
}

mod tests {
    #[test]
    #[ignore]
    fn test_generate() {
        let pdf = super::LoginDocument::generate(
            103151,
            "0202020000",
            "LWYA842B"
        );
        assert!(pdf.is_ok());
        // std::fs::write("test.pdf", pdf.unwrap()).unwrap();
    }
}