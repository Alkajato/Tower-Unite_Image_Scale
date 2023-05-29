// use const_format::concatcp;
use dioxus::prelude::*;

fn get_input() -> String {
    let mut input = String::from("");
    std::io::stdin().read_line(&mut input).unwrap();

    input
}

fn get_resolution(url: &str) -> Result<(f32, f32), String> {
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
    Ok((image.width() as f32, image.height() as f32))
}

fn url_to_scaling(url: &str) -> Result<(f32, f32), String> {
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

    println!("Ctrl C to close anytime");
    println!("Imgur URLs provably known to work, other URLs may fail\n");

    loop {
        println!("Provide an image URL: ");
        let input = get_input();

        match url_to_scaling(&input) {
            Err(error_msg) => println!("{error_msg}\n"),
            Ok((x_scale, y_scale)) => {
                if x_scale == 1.0 {
                    println!("Scale Y dimensions by {y_scale}");
                } else {
                    println!("Scale X dimensions by {x_scale}");
                }
            }
        }
    }
}

// Seem to require to look up how JSX does things.
// https://www.w3schools.com/react/react_css.asp
// Things "style" can include.
// https://developer.mozilla.org/en-US/docs/Web/CSS
// https://stackoverflow.com/questions/42125775/reactjs-react-router-how-to-center-div
fn app(cx: Scope) -> Element {
    // If url_state evaluates to a url string
    // run url_to_scaling on it to define ratio of X : Y
    // Set x_state and y_state appropriately.

    // This link makes X greater than Y in the ratio: https://i.imgur.com/7XW1LdK.png
    // This link makes Y greater than X in the ratio: https://i.imgur.com/LEQ7AB5.png
    let ratio = use_state(cx, || (f32::NAN, f32::NAN));

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
                        ratio.set((f32::NAN, f32::NAN));
                        match url_to_scaling(&evt.value) {
                            Err(error_msg) => println!("Failuring to resolve scaling: {error_msg}"),
                            Ok((x_scale, y_scale)) => {
                                ratio.set((x_scale, y_scale));

                                println!("Input evaluated");
                                println!("{:?}", (x_scale, y_scale));

                                // println!("x_scale into y_scale: {}", (y_scale / x_scale) * x_scale);
                                // println!("y_scale into x_scale: {}", (x_scale / y_scale) * y_scale); // Is this only if x > y?
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
                            if evt.value.is_empty() {
                                y_state.set(String::from(""));
                            }

                            // Share X size.
                            x_state.set(evt.value.clone());

                            // Y should change appropriately based on this new size.

                            // X can be inputted to while:
                            // ratio has both NAN || Y is empty || Y fails parsing to a f32

                            // if X in ratio is 1.0, Y should be ratio.y of what is entered into X input.
                            let (x_ratio, y_ratio) = (ratio.0, ratio.1);
                            dbg!(x_ratio);
                            dbg!(y_ratio);

                            if x_ratio > y_ratio {
                                println!("In x > y");

                                if let Ok(x_val) = evt.value.clone().parse::<f32>() {
                                    let y_val = format!("{}", y_ratio * x_val);
                                    y_state.set(y_val);
                                }
                                // let mut new_y = Sting::from("");
                                // let y_ratio = ratio.1;

                                // y_state.set(new_y);
                            } else {
                                println!("In y > x");
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
                            if evt.value.is_empty() {
                                x_state.set(String::from(""));
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
