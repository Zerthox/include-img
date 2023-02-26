# include_img
Macro to include image pixel data with optional conversion.

Supported conversion formats are RGB8, RGBA8, RGB16, RGBA16, RGB32F, RGBA32F, Luma8, LumaAlpha8, Luma16 and LumaAlpha16.
Format names are case insensitive and may be shortened, for example `la8` works for LumaAlpha8.

By default the path needs to be specified from your `Cargo.toml` file.
Enabling the `nightly` feature and compiling on nightly allows paths relative to the current source file.

# Usage
```rs
use include_img::include_img;

const IMAGE_RGB8: &[u8] = &include_img!("./src/assets/my_image.png", rgb8);
const IMAGE_RGBA32F: &[f32] = &include_img!("./src/assets/my_image.png", rgba32f);
```
