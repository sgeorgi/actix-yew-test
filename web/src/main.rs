use crate::container::Container;

mod components;
mod container;

fn main() {
    yew::start_app::<Container>()
}
