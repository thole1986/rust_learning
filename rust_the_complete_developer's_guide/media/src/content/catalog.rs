use super::media::Media; // super -> go up to parent module.

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }
    
    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        
        // Get item as reference not move.
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            // Out of index -> no items
            None
        }
    }
}