use yew::prelude::*;

use super::list_elements::ListElements;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub list: Vec<String>,
}

#[function_component(OrderedList)]
pub fn ordered_list(props: &Props) -> Html {
    html! {
        <ol>
            <ListElements list={
                props
                .list
                .clone()
            } />
        </ol>
    }
}
