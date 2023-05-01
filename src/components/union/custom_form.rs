use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::components::unit::{text_button::TextButton, text_input::TextInput};
use crate::stores::user_store::User;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Properties, Serialize, Deserialize)]
pub struct Saved {
    pub saved: bool,
}

// pub enum Msg {
//     UsernameChanged(String),
//     LanguageChanged(String),
//     FormSubmit(User),
//     Store(Rc<User>)
// }

#[derive(PartialEq, Properties)]
pub struct Props {
    pub on_submit: Callback<User>,
}

// pub struct CustomForm{
//     dispatch: Dispatch<User>,
//     username: String,
//     language: String,
//     saved: bool,
// }

// impl Component for CustomForm {
//     type Message = Msg;

//     type Properties = ();

//     fn create(ctx: &Context<Self>) -> Self {
//         let dispatch = Dispatch::<User>::subscribe(ctx.link().callback(Msg::Store));
//         Self { dispatch , saved: false, username: String::new(), language: String::new() }

//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let user = self.dispatch.get().clone();
//         let username = (&user.username).clone();
//         let language = (&user.language).clone();
//         let username_changed = {&ctx.link().callback(|name| {
//             Msg::UsernameChanged(name)
//         })};
//         let language_changed = {&ctx.link().callback(|lang| {
//             Msg::LanguageChanged(lang)
//         })};

//         let user = User{
//             username: username.clone(),
//             language: language.clone()
//         };

//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::UsernameChanged(name) => {
//                 self.username = name;
//                 self.saved = false;
//                 true
//             },
//             Msg::LanguageChanged(lang) => {
//                 self.language = lang;
//                 self.saved = false;
//                 true
//             },
//             Msg::FormSubmit(_) => {

//             true
//             },
//             Msg::Store(_) => false,
//         }
//     }
// }

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let state = use_state(User::default);
    let saved = use_state(|| false);

    let (store, dispatch_user) = use_store::<User>();

    let username_changed = {
        let cloned_state = state.clone();
        let cloned_saved = saved.clone();
        Callback::from(move |username| {
            cloned_saved.set(false);
            cloned_state.set(User {
                username,
                ..cloned_state.deref().clone()
            });
        })
    };

    let language_changed = {
        let cloned_state = state.clone();
        let cloned_saved = saved.clone();
        Callback::from(move |language| {
            cloned_saved.set(false);
            cloned_state.set(User {
                language,
                ..cloned_state.deref().clone()
            });
        })
    };

    let button_clicked = {
        let cloned_dispatch = dispatch_user;
        //let cloned_on_submit = props.on_submit.clone();
        let cloned_state = state.clone();
        let cloned_saved = saved.clone();
        Callback::from(move |event: SubmitEvent| {
            cloned_saved.set(true);
            event.prevent_default();
            let state = cloned_state.deref().clone();
            cloned_dispatch.reduce_mut(|store| {
                store.username = state.username;
                store.language = state.language;
            })
        })
    };

    html! {
        <form onsubmit={button_clicked}>
            <TextInput name="User Name" handle_onchange={username_changed}/>
            <p />
            <TextInput name="Fav Lang" handle_onchange={language_changed}/>
            <p />
            <TextButton label="Save" />
            if !*saved {
                <p>{"Username: "}{state.username.clone()}{", using: "}{state.language.clone()}{" (Not saved)"}</p>
            }
            if state.username != store.username || state.language != store.language || *saved {
                <p>{"Username: "}{store.username.clone()}{", using: "}{store.language.clone()}{" (Saved)"}</p>
            }

        </form>

    }
}
