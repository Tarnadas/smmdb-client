use iced::{svg, Svg};

lazy_static! {
    pub static ref ADD: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/add.svg").to_vec(),
    ));
    pub static ref SORT: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/sort.svg").to_vec(),
    ));
    pub static ref DELETE: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/delete.svg").to_vec(),
    ));
}
