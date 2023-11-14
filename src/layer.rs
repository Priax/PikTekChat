//
// PROJECT, 2023
// PicTekChat
// File description:
// Layer
//
use crate::my_draw::pik::Layer;

pub fn create_new_layer() -> Layer {
    Layer {
        pixels: Vec::new(),
        visible: true
    }
}

pub fn delete_layer(layers: &mut Vec<Layer>, index: usize) {
    if index < layers.len() {
        layers.remove(index);
    }
}
