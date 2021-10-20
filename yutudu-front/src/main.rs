use yew::prelude::*;
mod todolist;
use todolist::TodoList;

struct HomePage {
    link: ComponentLink<Self>,
}
impl Component for HomePage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <header>
                    <a href="#" class="logo">
                        {"Yutudu"}
                    </a>
                    <a href="#" class="button">
                        { "Home" }
                    </a>
                    <a href="#" class="button">
                        { "Login" }
                    </a>
                </header>
                <main>
                    <TodoList />
                </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<HomePage>();
}
