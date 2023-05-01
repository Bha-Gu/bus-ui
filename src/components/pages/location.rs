use std::{borrow::Cow, ops::Deref};

use gloo::console::log;
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::unit::{text_button::TextButton, text_input::TextInput};

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = [])]
//     async fn get();
// }

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Locations {
    pub plate: Cow<'static, str>,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f32,
}

async fn api_post(id: &str, lat: f64, long: f64) -> Result<String, reqwest::Error> {
    let url = "http://127.0.0.1:8000/bus";
    //let url = "https://script.google.com/macros/s/AKfycbylIG_r6TTX_XLJ0Wn1uajkQbhy9on4fV8QwntvDGDcFA3-Si4PwzriKbjxXtm5Vrlx/exec";
    let body = json!({
        "busid": id,
        "latitude": lat,
        "longitude": long
    });

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

#[function_component(Location)]
pub fn location() -> Html {
    let state_list = [
        "AN", "AP", "AR", "AS", "BH", "BR", "CH", "CG", "DD", "DL", "GA", "GJ", "HR", "HP", "JK",
        "JH", "KA", "KL", "LA", "LD", "MP", "MH", "MN", "ML", "MZ", "NL", "OD", "PY", "PB", "RJ",
        "SK", "TN", "TS", "TR", "UP", "UK", "WB",
    ];
    let state = use_geolocation();
    let valid_plate = use_state(|| false);
    let plate_state = use_state(|| "".to_owned());
    let plate_change = {
        let cloned_valid_plate = valid_plate.clone();
        let cloned_plate_state = plate_state.clone();
        Callback::from(move |plate: String| {
            let plate = plate.to_uppercase();
            let plate_trim = plate.chars().filter(|ch| ch.is_ascii_alphanumeric());
            let state =
                state_list.contains(&plate_trim.clone().take(2).collect::<String>().as_str());
            let region_code = plate_trim
                .clone()
                .skip(2)
                .take(2)
                .all(|ch| ch.is_ascii_digit());
            let plate_id = plate_trim.clone().filter(|ch| ch.is_ascii_digit()).count() == 6;
            let idsize = plate_trim
                .clone()
                .rev()
                .take(4)
                .map(|ch| ch.is_ascii_digit())
                .all(|b| b);
            let invalid_char = plate_trim.clone().any(|ch| ch == 'I' || ch == 'O');
            cloned_valid_plate.set(state && region_code && plate_id && idsize && !invalid_char);
            cloned_plate_state.set(plate_trim.collect());
        })
    };
    let onclick = {
        let cloned_plate_state = plate_state.clone();
        Callback::from(move |_| {
            let cloned_plate_state = cloned_plate_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let out = match api_post(
                    &cloned_plate_state,
                    state.latitude,
                    state.longitude,
                )
                .await
                {
                    Ok(a) => a,
                    Err(a) => a.to_string(),
                };
                log!("{}", out);
            })
        })
    };
    html! {
        <>
            <b>{ " Latitude: " }</b>
            { state.latitude }<p />
            <b>{ " Longitude: " }</b>
            { state.longitude }<p />
            <b>{ " Speed: " }</b>
            { match state.speed {
                Some(a) => format!("{}", a),
                None => String::from("NaN")
            } }<p />
            <b>{ " Timestamp: " }</b>
            { state.timestamp }<p />
            <TextInput name="Number Plate" handle_onchange={plate_change} />
            <p />
            {"Your Plate Number: "}{plate_state.deref()}
            <p />
            if !state.loading {
                //<button onclick={onclick}>{"Send"}</button>
                if *valid_plate {
                    <TextButton label={"Send"} onclick_callback={onclick}/>
                } else {
                    {"Plate format is invalid"}
                    <p />
                    <TextButton label={"ReValidate"}/>
                }
            }
            else {
                {"Loading location"}
            }
    </>
    }
}
