use gloo_storage::{LocalStorage, Storage};
use serde_derive::{Deserialize, Serialize};
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;
use yutudu_common::TodoAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    text: String,
    done: bool,
}

pub struct TodoList {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    input: String,
    tasks: Vec<Task>,
    fetch_task: Option<FetchTask>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TodoMsg {
    Action(TodoAction),
    Noop,
}

impl From<TodoAction> for TodoMsg {
    fn from(a: TodoAction) -> Self {
        TodoMsg::Action(a)
    }
}

impl Component for TodoList {
    type Message = TodoMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input: String::new(),
            tasks: LocalStorage::get("tasks").unwrap_or_default(),
            fetch_task: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let action = match msg {
            TodoMsg::Action(x) => x,
            TodoMsg::Noop => return false,
        };

        let post_request = Request::post("http://localhost:8080/api/todo")
            .header("Content-Type", "application/json")
            .body(Json(&action))
            .unwrap();
        let callback = self.link.batch_callback(
            |response: yew::services::fetch::Response<anyhow::Result<Vec<u8>>>| match response
                .body()
            {
                Err(_) => None,
                Ok(bin) => serde_json::from_slice(&bin[..])
                    .ok()
                    .map(|a| TodoMsg::Action(a)),
            },
        );
        self.fetch_task = Some(FetchService::fetch_binary(post_request, callback).unwrap());

        match action {
            TodoAction::AddTask(text) => {
                self.tasks.push(Task { text, done: false });
                LocalStorage::set("tasks", &self.tasks).unwrap_or(());
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            TodoAction::DeleteTask(id) => {
                self.tasks.remove(id);
                LocalStorage::set("tasks", &self.tasks).unwrap_or(());
                true
            }
            TodoAction::CompleteTask(id) => {
                for task in self.tasks.get_mut(id) {
                    task.done = true;
                }
                LocalStorage::set("tasks", &self.tasks).unwrap_or(());
                true
            }
            TodoAction::ReopenTask(id) => {
                for task in self.tasks.get_mut(id) {
                    task.done = false;
                }
                LocalStorage::set("tasks", &self.tasks).unwrap_or(());
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <h2> { "To-do List" } </h2>
                { for self.tasks.iter().enumerate().map(|(i, t)| html! {
                    <div class=classes!("row", if t.done { Some("done") } else { None }) key={i}>
                        <div class="col-sm-8">
                            { &t.text }
                        </div>
                        <div class="col-sm-2">
                            <a href="#" onclick=self.link.callback(move |_| TodoAction::DeleteTask(i))> { "cancel" } </a>
                        </div>
                        <div class="col-sm-2">
                            {
                                if t.done {
                                    html!{
                                        <a href="#" onclick=self.link.callback(move |_| TodoAction::ReopenTask(i))> { "reopen" } </a>
                                    }
                                } else {
                                    html!{
                                        <a href="#" onclick=self.link.callback(move |_| TodoAction::CompleteTask(i))> { "done" } </a>
                                    }
                                }
                            }
                        </div>
                    </div>
                }) }
                <input type="text"
                    value=self.input.clone()
                    placeholder={"Add a new task"}
                    onchange=self.link.batch_callback(|e: ChangeData| match e {
                        ChangeData::Value(v) => Some(TodoMsg::Action(TodoAction::AddTask(v))),
                        _ => None
                    })
                />
            </div>
        }
    }
}
