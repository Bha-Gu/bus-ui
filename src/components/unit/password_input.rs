use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(PasswordInput)]
pub fn password_input(props: &Props) -> Html {
    let onchange = {
        let cloned_handle_onchange = props.handle_onchange.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            cloned_handle_onchange.emit(value);
        })
    };

    html! {
        <input onchange={onchange} placeholder={props.name.clone()} type="password" name={props.name.clone()} />
    }
}

// pub struct PasswordInput {}

// impl Component for PasswordInput {
//     type Message = Msg;

//     type Properties = Props;

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {}
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::Changed(value) => ctx.props().handle_onchange.emit(value),
//         }
//         false
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {

//     }
// }
