use super::*;
use std::iter::zip;
use wasm_bindgen::Clamped;

#[derive(Debug)]
pub struct OriginalImageData {
    dimension: PixelDimension,
    data: Vec<u8>,
}

impl OriginalImageData {
    pub fn new(image_layer: &ImageLayerData) -> Self {
        let dimension = image_layer.dimension();
        match image_layer {
            ImageLayerData::Colored { pixels } => todo!(),
            ImageLayerData::Binary28 { rows } => {
                let mut data = Vec::new();
                data.reserve(28 * 28 * 4);
                for i in 0..28 {
                    for j in 0..28 {
                        let v: u8 = ((rows[i] >> (28 - j)) & 1) as u8 * 255;
                        data.extend([v, v, v, 255].into_iter())
                    }
                }
                Self { dimension, data }
            }
        }
    }

    pub fn new_composed(image_layers: &[ImageLayerData]) -> Self {
        let mut composed_image = Self::new(&image_layers[0]);
        for image_layer in &image_layers[1..] {
            composed_image.join(&Self::new(image_layer))
        }
        composed_image
    }

    pub fn join(&mut self, other: &Self) {
        assert_eq!(self.dimension, other.dimension);
        zip(self.data.iter_mut(), other.data.iter()).for_each(|(u, v)| *u = (*u).max(*v));
    }

    pub fn to_image_data_scaled(&self, dimension: PixelDimension) -> ImageData {
        let mut data = vec![];
        data.reserve((dimension.width * dimension.height) as usize * 4);
        for i1 in 0..dimension.height {
            for j1 in 0..dimension.width {
                let i = i1 * self.dimension.height / dimension.height;
                let j = j1 * self.dimension.width / dimension.width;
                for c in 0..4 {
                    data.push(self.data[((i * self.dimension.width + j) * 4 + c) as usize])
                }
            }
        }
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut data),
            dimension.width,
            dimension.height,
        )
        .unwrap()
    }

    pub fn to_image_data(mut self) -> ImageData {
        log::info!("{:?} {:?}", self.dimension.width, self.dimension.height,);
        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut self.data),
            self.dimension.width,
            self.dimension.height,
        )
        .unwrap()
    }
}
