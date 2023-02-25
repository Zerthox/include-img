#![feature(proc_macro_span)]

use image::DynamicImage;
use proc_macro::Span;
use proc_macro2::TokenStream;
use quote::quote;
use strum::EnumString;
use syn::{parse::Parse, Ident, LitStr, Token};

/// Includes pixel data from an image file.
///
/// Optionally the image can be converted into a format specified by the 2nd parameter.
/// Supported formats are RGB8, RGBA8, RGB16, RGBA16, RGB32F, RGBA32F, Luma8, LumaAlpha8, Luma16 and LumaAlpha16.
/// Format names are case insensitive and may be shortened, for example `la8` works  for LumaAlpha8.
///
/// # Examples
/// ```ignore
/// use include_img::include_img;
///
/// const IMAGE_RGB8: &[u8] = &include_img!("./my_image.png", rgb8);
/// const IMAGE_RGBA32F: &[f32] = &include_img!("./my_image.png", rgba32f);
/// ```
#[proc_macro]
pub fn include_img(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as Input);

    let path = input.path.value();
    let source_file = Span::call_site().source_file();
    let full_path = source_file.path().parent().unwrap().join(&path);

    match image::open(full_path) {
        Ok(img) => match input.format {
            Some(format) => format.convert(img),
            None => {
                let bytes = img.into_bytes();
                quote! { [ #( #bytes ),* ] }
            }
        }
        .into(),

        Err(err) => syn::Error::new(input.path.span(), format!("couldn't read {path}: {err}"))
            .to_compile_error()
            .into(),
    }
}

struct Input {
    path: LitStr,
    format: Option<Format>,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.parse()?,
            format: if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                let ident: Ident = input.parse()?;
                let format = ident
                    .to_string()
                    .parse()
                    .map_err(|err: strum::ParseError| syn::Error::new(ident.span(), err))?;
                Some(format)
            } else {
                None
            },
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString)]
#[strum(ascii_case_insensitive)]
enum Format {
    Rgb8,
    Rgba8,
    Rgb16,
    Rgba16,
    Rgb32f,
    Rgba32f,

    #[strum(serialize = "L8", serialize = "Luma8")]
    L8,

    #[strum(serialize = "La8", serialize = "LumaA8", serialize = "LumaAlpha8")]
    La8,

    #[strum(serialize = "L16", serialize = "Luma16")]
    L16,

    #[strum(serialize = "La16", serialize = "LumaA16", serialize = "LumaAlpha16")]
    La16,
}

impl Format {
    fn convert(&self, img: DynamicImage) -> TokenStream {
        match self {
            Format::Rgb8 => {
                let img = img.into_rgb8();
                quote! { [ #( #img ),* ] }
            }
            Format::Rgba8 => {
                let img = img.into_rgba8();
                quote! { [ #( #img ),* ] }
            }
            Format::Rgb16 => {
                let img = img.into_rgb16();
                quote! { [ #( #img ),* ] }
            }
            Format::Rgba16 => {
                let img = img.into_rgba16();
                quote! { [ #( #img ),* ] }
            }
            Format::Rgb32f => {
                let img = img.into_rgb32f();
                quote! { [ #( #img ),* ] }
            }
            Format::Rgba32f => {
                let img = img.into_rgba32f();
                quote! { [ #( #img ),* ] }
            }
            Format::L8 => {
                let img = img.into_luma8();
                quote! { [ #( #img ),* ] }
            }
            Format::La8 => {
                let img = img.into_luma_alpha8();
                quote! { [ #( #img ),* ] }
            }
            Format::L16 => {
                let img = img.into_luma16();
                quote! { [ #( #img ),* ] }
            }
            Format::La16 => {
                let img = img.into_luma_alpha16();
                quote! { [ #( #img ),* ] }
            }
        }
    }
}
