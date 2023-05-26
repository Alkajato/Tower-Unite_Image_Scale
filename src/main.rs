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

fn main() {
    dioxus_desktop::launch(app);

    println!("Ctrl C to close anytime");
    println!("Imgur URLs provably known to work, other URLs may fail\n");

    loop {
        println!("Provide an image URL: ");
        let input = get_input();

        let result = get_resolution(&input);
        if let Some(error_msg) = result.as_ref().err() {
            println!("{error_msg}\n");
        } else {
            let (width, height) = result.unwrap();
            if width < height {
                let new_x = width / height;
                println!("The X Scale to use is {new_x}\n");
            } else {
                let new_y = height / width;
                println!("The Y Scale to use is {new_y}\n");
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

    cx.render(rsx! {
        div {
            style: "width: 50%; justify-self: center;",
            margin_left: DIV_MARGIN,
            input {
                value: "Paste URL here",
                inputmode: "url",
                style: "width: 100%",
            }
        }
    })
}
