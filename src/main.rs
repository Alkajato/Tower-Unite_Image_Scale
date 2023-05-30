use dioxus::prelude::*;

fn get_resolution(url: &str) -> Result<(f64, f64), String> {
    let img_bytes = reqwest::blocking::get(url);
    if let Err(error) = img_bytes {
        let msg = format!("Failed getting url: \"{error:?}\"");

        return Err(msg);
    }

    let img_bytes = img_bytes.unwrap().bytes();
    if let Err(error) = img_bytes {
        let msg = format!("Failed getting data from image: \"{error:?}\"");

        return Err(msg);
    }

    let image = image::load_from_memory(&img_bytes.unwrap());
    if let Err(error) = image {
        let msg = format!("Failed to read data from image: \"{error:?}\"");

        return Err(msg);
    }

    let image = image.unwrap();
    Ok((image.width() as f64, image.height() as f64))
}

fn url_to_scaling(url: &str) -> Result<(f64, f64), String> {
    match get_resolution(url) {
        Err(error_msg) => Err(error_msg),
        Ok((width, height)) => {
            let x_scale;
            let y_scale;
            if width < height {
                x_scale = width / height;
                y_scale = 1.0;
            } else {
                y_scale = height / width;
                x_scale = 1.0;
            }

            Ok((x_scale, y_scale))
        }
    }
}

fn main() {
    dioxus_desktop::launch(app);
}

// Seem to require to look up how JSX does things.
// https://www.w3schools.com/react/react_css.asp
// Things "style" can include.
// https://developer.mozilla.org/en-US/docs/Web/CSS
// https://stackoverflow.com/questions/42125775/reactjs-react-router-how-to-center-div

// This link makes X greater than Y in the ratio: https://i.imgur.com/7XW1LdK.png
// This link makes Y greater than X in the ratio: https://i.imgur.com/LEQ7AB5.png
fn app(cx: Scope) -> Element {
    let ratio = use_state(cx, || (f64::NAN, f64::NAN));

    let x_state = use_state(cx, || String::from(""));
    let y_state = use_state(cx, || String::from(""));

    cx.render(rsx! {
        style {
            include_str!("../src/style_dark.css")
        }

        div {
            class: "body bg1",

            div {
                class: "title",
                "Tower Unite Image Scaler"
            }

            div {
                class: "urlinputdiv radius bg2",
                "Input URL"

                // URL input struct.
                input {
                    class: "urlinput radius",
                    oninput: move |evt| {
                        ratio.set((f64::NAN, f64::NAN));
                        x_state.set(String::from(""));
                        y_state.set(String::from(""));

                        match url_to_scaling(&evt.value) {
                            Err(error_msg) => println!("{error_msg}"),
                            Ok((x_scale, y_scale)) => {
                                ratio.set((x_scale, y_scale));

                                println!("Ratio Determined: {:?}", (x_scale, y_scale));
                            }
                        }
                    },
                    placeholder: "Paste image address",
                    autofocus: "",
                    inputmode: "url",
                }
            }

            div {
                class: "scaleinputdiv radius bg2",

                div {
                    style: "display: flex; flex-direction: column;",
                    "X Scale"

                    // X input struct.
                    input {
                        class: "scaleinput radius",
                        oninput: move |evt| {
                            x_state.set(evt.value.clone());

                            if evt.value.is_empty() {
                                y_state.set(String::from(""));
                                return;
                            }

                            let (x_ratio, y_ratio) = (ratio.0, ratio.1);
                            if x_ratio.is_nan() || y_ratio.is_nan() {
                                return;
                            }

                            if let Ok(x_val) = evt.value.parse::<f64>() {
                                let y_val = if x_ratio > y_ratio {
                                    format!("{:.2}", y_ratio * x_val)
                                } else {
                                    let scale_up = y_ratio / x_ratio;
                                    format!("{:.2}", scale_up * x_val)
                                };

                                y_state.set(y_val);
                            }
                        },
                        placeholder: "Input X size",
                        value: "{x_state}",
                        inputmode: "decimal",
                    }
                }

                div {
                    style: "display: flex; flex-direction: column;",
                    "Y Scale"

                    // Y input struct.
                    input {
                        class: "scaleinput radius",
                        oninput: move |evt| {
                            y_state.set(evt.value.clone());

                            if evt.value.is_empty() {
                                x_state.set(String::from(""));
                                return;
                            }

                            let (x_ratio, y_ratio) = (ratio.0, ratio.1);
                            if x_ratio.is_nan() || y_ratio.is_nan() {
                                return;
                            }

                            if let Ok(y_val) = evt.value.parse::<f64>() {
                                let x_val = if y_ratio > x_ratio {
                                    format!("{:.2}", x_ratio * y_val)
                                } else {
                                    let scale_up = x_ratio / y_ratio;
                                    format!("{:.2}", scale_up * y_val)
                                };

                                x_state.set(x_val);
                            }
                        },
                        placeholder: "Input Y size",
                        value: "{y_state}",
                        inputmode: "decimal",
                    }
                }
            }
        }
    })
}
