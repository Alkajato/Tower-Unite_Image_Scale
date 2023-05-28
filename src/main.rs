use const_format::concatcp;
use dioxus::prelude::*;

fn get_input() -> String {
    let mut input = String::from("");
    std::io::stdin().read_line(&mut input).unwrap();

    input
}

fn get_resolution(url: &str) -> Result<(f32, f32), String> {
    let img_bytes = reqwest::blocking::get(url);
    if img_bytes.is_err() {
        let error = img_bytes.err().unwrap();
        let msg = format!("Failed getting url: \"{:?}\"", error);

        return Err(msg);
    }

    let img_bytes = img_bytes.unwrap().bytes();
    if img_bytes.is_err() {
        let error = img_bytes.err().unwrap();
        let msg = format!("Failed getting data from image: \"{:?}\"", error);

        return Err(msg);
    }

    let image = image::load_from_memory(&img_bytes.unwrap());
    if image.is_err() {
        let error = image.err().unwrap();
        let msg = format!("Failed to read data from image: \"{:?}\"", error);

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
const DIV_WIDTH_PERCENT: i32 = 50;
const DIV_MARGIN: &str = concatcp!(DIV_WIDTH_PERCENT / 2, "%");

fn app(cx: Scope) -> Element {
    let line_container = "background-color: rgb(49, 46, 41); display: grid; width: 100%; height: 6px; padding: 2px 0px 2px 0px; margin: 2px 0px 2px 0px;";
    let divider_line = "background-color: rgb(119, 112, 100); text-align: center; justify-self: center; width: 60%; height: 6px;";
    let header_style = "color: rgb(255, 255, 255); background-color: rgb(32, 30, 27); text-align: center; position: relative; height: 100vh; width: 100%; min-width: 600px; max-width: 1280px;";

    let url_state = use_state(cx, || String::from(""));

    // If url_state evaluates to a url string
    // run url_to_scaling on it to define ratio of X : Y
    // Set x_state and y_state appropriately.
    let x_state = use_state(cx, || 1.0);
    let y_state = use_state(cx, || 1.0);

    cx.render(rsx! {
        div {
            style: "width: 50%; justify-self: center;",
            margin_left: DIV_MARGIN,
            input {
                oninput: move |evt| {
                    println!("Input seen");
                    match url_to_scaling(&evt.value) {
                        Err(error_msg) => println!("Failed to resolve scaling: {error_msg}"),
                        Ok((width, height)) => {
                            println!("Input evaluated");
                            println!("{width} : {height}");
                        }
                    }
                    // if evt.value.is_empty() {
                    //     url_state.set(default_url_text.clone());
                    // }

                    // url_state.set(evt.value.clone());
                    // url_state.set(evt.value.as_str());

                },
                placeholder: "Paste URL here",
                autofocus: "",
                inputmode: "url",
                style: "width: 100%",
            }
            div {
                style: "float: left",
                input {
                    placeholder: "X Dimensions",
                    inputmode: "decimal",
                    style: "text-align: center",
                }
                input {
                    placeholder: "Y Dimensions",
                    inputmode: "decimal",
                    style: "text-align: center",
                }
            }
        }
    })
}
