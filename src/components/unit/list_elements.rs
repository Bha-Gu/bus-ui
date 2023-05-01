use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub list: Vec<String>,
}

#[function_component(ListElements)]
pub fn list_elements(props: &Props) -> Html {
    html! {
        props.list
            .iter()
            .map(|task|
                html!{
                    <li>{task}</li>
                }
            )
            .collect::<Html>()
    }
}

pub fn vec_str_to_string(input: Vec<&str>) -> Vec<String> {
    input.iter().map(|&s| String::from(s)).collect()
}
