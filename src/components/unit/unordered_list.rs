use yew::prelude::*;

use super::list_elements::ListElements;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub list: Vec<String>,
}

#[function_component(UnorderedList)]
pub fn unordered_list(props: &Props) -> Html {
    html! {
        <ul>
            <ListElements list={
                props
                .list
                .clone()
            } />
        </ul>
    }
}
