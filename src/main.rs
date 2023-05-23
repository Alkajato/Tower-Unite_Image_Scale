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

