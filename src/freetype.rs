use crate::harfbuzz::{hb_face_t, hb_font_t};
use paidtype::freetype::FT_Face;

unsafe extern "C" {
    pub fn hb_ft_face_create_referenced(ft_face: FT_Face) -> *mut hb_face_t;
    pub fn hb_ft_font_create_referenced(ft_face: FT_Face) -> *mut hb_font_t;
}

#[cfg(test)]
mod tests {
    use crate::{
        freetype::{hb_ft_face_create_referenced, hb_ft_font_create_referenced},
        harfbuzz::{hb_face_destroy, hb_font_destroy},
    };
    use paidtype::freetype::{FT_Done_Face, FT_Init_FreeType, FT_Long, FT_New_Memory_Face};

    const FONT: &'static [u8] = include_bytes!("../unifont-17.0.04.otf");

    #[test]
    fn face_lifecycle() {
        let mut library = std::ptr::null_mut();
        unsafe { FT_Init_FreeType(&mut library) };

        let mut ft_face = std::ptr::null_mut();
        unsafe {
            FT_New_Memory_Face(
                library,
                FONT.as_ptr(),
                FONT.len() as FT_Long,
                0,
                &mut ft_face,
            )
        };

        let hb_face = unsafe { hb_ft_face_create_referenced(ft_face) };
        assert_eq!(hb_face.is_null(), false);

        unsafe { hb_face_destroy(hb_face) };
        unsafe { FT_Done_Face(ft_face) };
    }

    #[test]
    fn font_lifecycle() {
        let mut library = std::ptr::null_mut();
        unsafe { FT_Init_FreeType(&mut library) };

        let mut ft_face = std::ptr::null_mut();
        unsafe {
            FT_New_Memory_Face(
                library,
                FONT.as_ptr(),
                FONT.len() as FT_Long,
                0,
                &mut ft_face,
            )
        };

        let hb_font = unsafe { hb_ft_font_create_referenced(ft_face) };
        assert_eq!(hb_font.is_null(), false);

        unsafe { hb_font_destroy(hb_font) };
        unsafe { FT_Done_Face(ft_face) };
    }
}
