use geo::prelude::Contains; //trait to see if one geometrical object is inside another
use geo::{Coordinate, LineString, Polygon};
use rand::Rng;

const IMG_WIDTH: u32 = 400;
const IMG_HEIGHT: u32 = 400;
const NUMBER_OF_POINTS: u32 = 100000;
const FRACTION: f32 = 1. / 6.;
const NR_SIDES: i32 = 6;

///if you want to get a position located on a certain spot on a line between two points
fn get_point_on_fraction_between(
    point1: Coordinate<f32>,
    point2: Coordinate<f32>,
    fraction: f32,
) -> Coordinate<f32> {
    let (point_on_fraction_x, point_on_fraction_y) = (
        point1.x + fraction * (point2.x - point1.x),
        point1.y + fraction * (point2.y - point1.y),
    );
    Coordinate {
        x: point_on_fraction_x,
        y: point_on_fraction_y,
    }
}

///get's the x position of the nth vertex of a polygon, using a set radius
fn get_nth_x_of_polygon_with_radius(n:i32, radius:i32) -> f32 {
    radius as f32 * (2. * std::f32::consts::PI * n as f32 / NR_SIDES as f32).cos() + IMG_WIDTH as f32 / 2.
}
///get's the y position of the nth vertex of a polygon, using a set radius
fn get_nth_y_of_polygon_with_radius(n:i32, radius:i32) -> f32 {
    radius as f32 * (2. * std::f32::consts::PI * n as f32 / NR_SIDES as f32).sin() + IMG_HEIGHT as f32 / 2.
}
/// returns an n sided polygon with radius r, and the coordinates of it's vertices
fn get_polygon_and_edges_of_radius(nr_sides:i32, radius:i32) -> (Polygon<f32>, Vec<Coordinate<f32>>) {
    let edges = (0..nr_sides)
        .map(|x| Coordinate {
            x: get_nth_x_of_polygon_with_radius(x,radius),
            y: get_nth_y_of_polygon_with_radius(x,radius),
        })
        .collect::<Vec<Coordinate<f32>>>();
    (Polygon::new(LineString::from(edges.clone()), vec![]), edges)
}

fn main() {
    let mut img_buffer = image::ImageBuffer::new(IMG_WIDTH + 1, IMG_HEIGHT + 1);
    let mut random_numer_generator = rand::thread_rng();
    
    ///get the polygon and it's edges, with a radius of half the image width or image height,
    ///depending on which one's smallest
    let (polygon, polygon_edges) = get_polygon_and_edges_of_radius(NR_SIDES,(std::cmp::min(IMG_WIDTH, IMG_HEIGHT) / 2) as i32);

    ///generate the first point, which should be inside the polygon that is being filled
    let mut random_x_in_polygon = random_numer_generator.gen_range(0..IMG_WIDTH) as f32;
    let mut random_y_in_polygon = random_numer_generator.gen_range(0..IMG_HEIGHT) as f32;

    ///iterate over all the pixels in the image buffer and set them to white
    for (_, _, pixel) in img_buffer.enumerate_pixels_mut() {
        *pixel = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
    }
    ///if the first point we created happened to fall outside the polygon, new ones are generated
    ///untill they are inside
    while !polygon.contains(&Coordinate {
        x: random_x_in_polygon,
        y: random_y_in_polygon,
    }) {
        random_x_in_polygon = random_numer_generator.gen_range(0..IMG_WIDTH) as f32;
        random_y_in_polygon = random_numer_generator.gen_range(0..IMG_HEIGHT) as f32;
    }

    for _ in 0..NUMBER_OF_POINTS {
        ///generate a point between the point generated in a previous iteration, and a random vertex in our polygon
        let xy_randomvertex_midpoint = get_point_on_fraction_between(
            polygon_edges[random_numer_generator.gen_range(0..polygon_edges.len())],
            Coordinate {
                x: random_x_in_polygon,
                y: random_y_in_polygon,
            },
            FRACTION,
        );
        ///color the pixel corresponding to the point that is just generated
        img_buffer.put_pixel(
            xy_randomvertex_midpoint.x as u32,
            xy_randomvertex_midpoint.y as u32,
            image::Rgb([0 as u8, 0 as u8, 0 as u8 * 2]),
        );
        ///set the point for the next iteration 
        random_x_in_polygon = xy_randomvertex_midpoint.x;
        random_y_in_polygon = xy_randomvertex_midpoint.y;
    }
    ///save the image to disk as a png
    img_buffer.save("fractal.png").unwrap();
}
