fn CountFiles(images: &str) {
    let mut count = 0;
    for image in images.lines() {
        if image.ends_with(".png") || image.ends_with(".jpg") {
            count += 1;
        }
    }
    println!("Total number of image files: {}", count);
}

fn main() {
    let images = "./images/*";
    let countingFiles = CountFiles(images);
}
