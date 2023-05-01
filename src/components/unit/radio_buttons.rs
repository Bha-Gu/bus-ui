use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: Vec<String>,
    pub handle_onchange: Callback<String>,
    pub selected: String,
}

#[function_component(RadioInput)]
pub fn radio_input(props: &Props) -> Html {
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
    // let radio = ;
    html! {
        <>
            {props.name.iter().map(|id| {
                html!{
                    <>
                        <input onchange={onchange.clone()} type="radio" checked={id.clone() == props.selected} value={id.clone()} /> {id.clone()} <p />
                    < />
                }
            }).collect::<Html>()}
        </>
    }
}
