use std::sync::Arc;

use rand::Rng;
use rand::seq::IndexedRandom;
use teloxide::types::FileId;

#[derive(Clone)]
pub struct StickerPicker {
    stickers: Arc<Vec<FileId>>,
}

impl StickerPicker {
    pub fn new(stickers: Vec<FileId>) -> Self {
        assert!(
            !stickers.is_empty(),
            "Sticker set is empty: check STICKER_SET_NAME"
        );
        Self {
            stickers: Arc::new(stickers),
        }
    }

    pub fn len(&self) -> usize {
        self.stickers.len()
    }

    pub fn pick_one(&self) -> FileId {
        let idx = rand::rng().random_range(0..self.stickers.len());
        self.stickers[idx].clone()
    }

    pub fn pick_many(&self, count: usize) -> Vec<FileId> {
        let mut rng = rand::rng();
        self.stickers
            .choose_multiple(&mut rng, count)
            .cloned()
            .collect()
    }
}
