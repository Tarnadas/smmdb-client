use iced::{image, svg, Svg};

lazy_static! {
    pub static ref ADD: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/add.svg").to_vec(),
    ));
    pub static ref UPLOAD: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/upload.svg").to_vec(),
    ));
    pub static ref SORT: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/sort.svg").to_vec(),
    ));
    pub static ref DELETE: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/delete.svg").to_vec(),
    ));
    pub static ref SETTINGS: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/settings.svg").to_vec(),
    ));
    pub static ref DOWN_ARROW: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/down_arrow.svg").to_vec(),
    ));
    pub static ref DOWN_ARROW_RED: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/down_arrow_red.svg").to_vec(),
    ));
    pub static ref UP_ARROW: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/up_arrow.svg").to_vec(),
    ));
    pub static ref UP_ARROW_GREEN: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/up_arrow_green.svg").to_vec(),
    ));
    pub static ref EASY: image::Handle =
        image::Handle::from_memory(include_bytes!("../assets/icons/easy.png").to_vec(),);
    pub static ref NORMAL: image::Handle =
        image::Handle::from_memory(include_bytes!("../assets/icons/normal.png").to_vec(),);
    pub static ref EXPERT: image::Handle =
        image::Handle::from_memory(include_bytes!("../assets/icons/expert.png").to_vec(),);
    pub static ref SUPER_EXPERT: image::Handle =
        image::Handle::from_memory(include_bytes!("../assets/icons/superexpert.png").to_vec(),);
}
