# include_img
Macro to include image pixel data with optional conversion.

# Usage
```rs
use include_img::include_img;

const IMAGE_RGB8: &[u8] = &include_img!("./my_image.png", rgb8);
const IMAGE_RGBA32F: &[f32] = &include_img!("./my_image.png", rgba32f);
```
