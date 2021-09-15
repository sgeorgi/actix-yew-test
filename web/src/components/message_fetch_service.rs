use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(Debug)]
pub enum Msg {
    GetMessage,
    ReceiveResponse(Result<Message, anyhow::Error>),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    title: String,
    body: String,
}

pub struct MessageFetchService {
    fetch_task: Option<FetchTask>,
    message: Option<Message>,
    error: Option<String>,
    link: ComponentLink<Self>,
}

impl MessageFetchService {
    fn view_message(&self) -> Html {
        match self.message {
            Some(ref message) => {
                return html! {
                    <>
                        <p>{ "Here is a message:" }</p>
                        <p>{ format!("Title: {}", message.title) }</p>
                        <p>{ format!("Body: {}", message.body) }</p>
                        <button onclick=self.link.callback(|_| Msg::GetMessage)>
                             { "Fetch another message" }
                         </button>
                    </>
                };
            }
            None => {
                return html! {
                     <button type="button" class=classes!("bg-blue-100", "ring-4", "rounded", "ml-auto") onclick=self.link.callback(|_| Msg::GetMessage)>
                         { "Fetch message from server" }
                     </button>
                };
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            return html! { <p>{ "Fetching data..." }</p> };
        } else {
            return html! { <p></p> };
        }
    }

    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            return html! { <p>{ error.clone() }</p> };
        } else {
            return html! {};
        }
    }
}

impl Component for MessageFetchService {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            message: None,
            error: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            ReceiveResponse(result) => {
                match result {
                    Ok(message) => self.message = Some(message),
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                true
            }
            GetMessage => {
                let request = Request::get("/api/v1/hello")
                    .body(Nothing)
                    .expect("Could not build request!");
                let callback = self.link.callback(
                    |response: Response<Json<Result<Message, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                let fetch_call =
                    FetchService::fetch(request, callback).expect("Could not send request");
                self.fetch_task = Some(fetch_call);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        return html! {
            <>
                { self.view_fetching() }
                { self.view_message() }
                { self.view_error() }
            </>
        };
    }
}
