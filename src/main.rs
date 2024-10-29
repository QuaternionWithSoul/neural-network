use mnist::*;

struct DataSet {
    learn_images: Vec<u8>,
    learn_result: Vec<u8>,
    test_images: Vec<u8>,
    test_result: Vec<u8>,
}

fn load_date_set(path: &str) -> DataSet {
    let Mnist {
        trn_img,
        trn_lbl,
        tst_img,
        tst_lbl,
        ..
    } = MnistBuilder::new()
        .base_path(path)
        .finalize();

    return DataSet {
        learn_images: trn_img,
        learn_result: trn_lbl,
        test_images: tst_img,
        test_result: tst_lbl,
    };
}

fn get_nomber(images: Vec<u8>, result: Vec<u8>, index: usize) {
    println!("Result: {}", result[index]);

    let first_image = &images[index*784..(index + 1)*784];
    
    for row in 0..28 {
        for col in 0..28 {
            let pixel = first_image[row * 28 + col];
            if pixel > 0 { print!("##"); }
            else { print!("  "); }
        }
        print!("\n");
    }
}

fn main() {
    let DataSet {
        learn_images: images, 
        learn_result: result, 
        ..
    } = load_date_set("data/mnist");

    get_nomber(images, result, 0);
}
