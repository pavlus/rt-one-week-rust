use crate::types::{P3, Color, P2};

use super::Texture;

impl Texture for Color {
    fn value(&self, _: &P2, _: &P3) -> Color { *self }
}

