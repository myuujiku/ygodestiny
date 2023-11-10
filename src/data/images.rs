use gtk::Image;

use super::dirs::IMAGES;

pub fn load_card(id: u32) -> Image {
    let filename = IMAGES.join(format!("{}.jpg", id));

    Image::from_file(filename)
}
