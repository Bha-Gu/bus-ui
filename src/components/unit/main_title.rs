use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub label: Option<String>,
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    html! {
        if let Some(label) = &props.label {
            <h1>{label}</h1>
        }else {
            <h1> {"<Main Title>"} </h1>
        }
    }
}

// pub struct MainTitle {}

// impl Component for MainTitle {
//     type Message = ();

//     type Properties = Props;

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {}
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {

//         }
//     }
// }
