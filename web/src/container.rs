use yew::prelude::*;

use crate::components::MessageFetchService;

pub struct Container {}

impl Component for Container {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        return html! {
            <>
                <h1>{ "Horray" }</h1>
                <MessageFetchService />
            </>
        };
    }
}
