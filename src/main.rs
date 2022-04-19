mod args;
use args::Args;
use image::{
    imageops::FilterType::Triangle, io::Reader, DynamicImage, GenericImageView, ImageFormat,
};
use std::{convert::TryInto, fs::File, io::BufReader};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }

        self.data = data;
        Ok(())
    }
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();

    let (image1, image_format1) = find_img(args.img1);
    let (image2, image_format2) = find_img(args.img2);

    if image_format1 != image_format2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image1, image2) = resize_img_size(image1, image2);
    let mut output = FloatingImage::new(image1.width(), image1.height(), args.output);

    let combined_data = combine_img(image1, image2);
    output.set_data(combined_data)?;

    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_format1,
    )
    .unwrap();
    Ok(())
}

// open img with the given path, get the format and decode it
// return the decoded img and the format
fn find_img(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();

    (image, image_format)
}

fn get_smallest_img(res1: (u32, u32), res2: (u32, u32)) -> (u32, u32) {
    let pix1 = res1.0 * res1.1;
    let pix2 = res2.0 * res2.1;

    return if pix1 < pix2 { res1 } else { res2 };
}

fn resize_img_size(img1: DynamicImage, img2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_img(img1.dimensions(), img2.dimensions());
    //debug
    println!("Width: {}\nheight: {}\n", width, height);

    if img2.dimensions() == (width, height) {
        (img1.resize_exact(width, height, Triangle), img2)
    } else {
        (img1, img2.resize_exact(width, height, Triangle))
    }
}

fn combine_img(img1: DynamicImage, img2: DynamicImage) -> Vec<u8> {
    let vec1 = img1.to_rgba8().into_vec();
    let vec2 = img2.to_rgba8().into_vec();

    alternate_pixels(vec1, vec2)
}

fn alternate_pixels(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    // Creates a vector of u8 with the lenght of vec1, eg: vec1.len() == 3 -> [u8. u8. u8]
    let mut combined_data = vec![0u8; vec1.len()];

    let mut i = 0;
    while i < vec1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&vec1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec2, i, i + 3));
        }

        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();

    for i in start..=end {
        let value: u8 = match vec.get(i) {
            Some(v) => *v,
            None => panic!("Index out of bounds"),
        };

        rgba.push(value);
    }

    rgba
}
