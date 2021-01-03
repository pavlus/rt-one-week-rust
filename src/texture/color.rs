use crate::types::{P3, Distance, Color};

use super::Texture;

impl Texture for Color {
    fn value(&self, _: Distance, _: Distance, _: &P3) -> Color { *self }
}
