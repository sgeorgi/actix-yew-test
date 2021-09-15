mod container;
mod components;

use crate::container::Container;

fn main() {
    yew::start_app::<Container>()
}
