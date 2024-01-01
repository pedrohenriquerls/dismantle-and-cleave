extern crate opencv;
use opencv::core::{Mat, Size, CV_8UC1, CV_8UC3};
use opencv::imgcodecs::imread;
use opencv::imgproc::{
    adaptive_threshold, contour_area, draw_contours, find_contours, morphology_ex, CHAIN_APPROX_SIMPLE,
    MORPH_CLOSE, RETR_EXTERNAL, THRESH_BINARY_INV,
};

// TODO:
// Handle different images
// Slice the panels in separate image
// Create metadata to allow reconstruct the whole page panel by panel on correct order
// Convert all images to ebook?

fn main() {
    let image = imread("comic_page.jpg", 1).unwrap();

    let mut gray = Mat::default();
    opencv::imgproc::cvt_color(
        &image,
        &mut gray,
        opencv::imgproc::COLOR_BGR2GRAY,
        0,
    )
    .unwrap();

    let mut thresh = Mat::default();
    adaptive_threshold(
        &gray,
        &mut thresh,
        255.0,
        opencv::imgproc::ADAPTIVE_THRESH_GAUSSIAN_C,
        THRESH_BINARY_INV,
        11,
        2.0,
    )
    .unwrap();

    let kernel = Mat::ones(Size::new(5.0, 5.0), CV_8UC1, Default::default()).unwrap();
    let mut morphed = Mat::default();
    morphology_ex(&thresh, &mut morphed, MORPH_CLOSE, &kernel, Default::default(), 1, Default::default())
        .unwrap();

    let mut contours = vector_vector_of_point::VectorOfVectorOfPoint::new();
    find_contours(&morphed, &mut contours, RETR_EXTERNAL, CHAIN_APPROX_SIMPLE, Default::default())
        .unwrap();

    let mut filtered_contours = vector_vector_of_point::VectorOfVectorOfPoint::new();
    for contour in contours.iter() {
        let bounding_rect = opencv::imgproc::bounding_rect(contour).unwrap();
        let aspect_ratio = bounding_rect.width as f64 / bounding_rect.height as f64;
        let area = contour_area(contour, false).unwrap();
        if aspect_ratio < 5.0 && area > 500.0 {
            filtered_contours.push(contour.clone());
        }
    }

    let mut result = image.clone().unwrap();
    draw_contours(
        &mut result,
        &filtered_contours,
        -1,
        opencv::core::Scalar::new(0.0, 0.0, 255.0, 0.0),
        2,
        8,
        Default::default(),
        0,
        Default::default(),
    )
    .unwrap();

    opencv::highgui::imshow("Render test image", &result).unwrap();
    opencv::highgui::wait_key(0).unwrap();
}
