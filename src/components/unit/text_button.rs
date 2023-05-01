use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub onclick_callback: Option<Callback<()>>,
}

#[function_component(TextButton)]
pub fn text_button(props: &Props) -> Html {
    let onclick = props.onclick_callback.clone().map(|cloned_onclick_callback| Callback::from(move |_| {
            cloned_onclick_callback.emit(());
        }));
    html! {
        <button onclick={onclick}>{&props.label}</button>
    }
}
