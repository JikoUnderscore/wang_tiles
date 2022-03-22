use std::time;
use image::{GenericImage, ImageBuffer, Rgb};
use rand::Rng;
use crate::vecs_def::{Vec2f, Vec3f};

macro_rules! smart_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
        impl std::convert::TryFrom<u32> for $name {
            type Error = ();

            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}



mod vecs_def;

// https://www.youtube.com/watch?v=_SufQh6OIzs&list=PLpM-Dvs8t0VYgJXZyQzWjfYUm3MxcvqR0&index=2
// 1.17 min
// https://www.youtube.com/watch?v=UiaVvVpWY4E&list=PLpM-Dvs8t0VYgJXZyQzWjfYUm3MxcvqR0&index=3

const TILE_WIDTH: u32 = 256;
const TILE_HEIGHT: u32 = 256;
const ROW: usize = 20;
const COL: usize = 20;

const GRID_WIDTH: u32 = TILE_WIDTH * ROW as u32;
const GRID_HEIGHT: u32 = TILE_HEIGHT * COL as u32;


fn make_rgb_bytes(r: f32, g: f32, b: f32) -> [u8; 3] {
    return [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]
}


fn generate_image_wang(tile: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, wang: fn(uv: Vec2f, bltr: u32) -> Vec3f, bltr: u32) {
    for (x, y, pixel) in tile.enumerate_pixels_mut() {
        let u = x as f32 / TILE_WIDTH as f32;
        let v = y as f32 / TILE_HEIGHT as f32;

        let p = wang(Vec2f::new(u, v), bltr);

        // println!("{:X} {:X} {:X}", p.r(), p.g(), p.b());
        *pixel = image::Rgb(make_rgb_bytes(p.r(), p.g(), p.b()));
    }
}


/*

fn generate_image(imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, shared: fn(Vec2f) -> Vec3f) {
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f32 / TILE_WIDTH as f32;
        let v = y as f32 / TILE_HEIGHT as f32;

        let p = shared(Vec2f::new(u, v));

        // println!("{:X} {:X} {:X}", p.r(), p.g(), p.b());
        *pixel = image::Rgb(make_rgb_bytes(p.r(), p.g(), p.b()));
    }
}

fn stripes(uv: Vec2f) -> Vec3f {
    let n = 20.0;

    // let mut  v = Vec3f::new((uv.u() * n).sin(), ((uv.u() + uv.v()) * n).sin(), (uv.v() * n).cos());
    // v.sum(Vec3f::from(1.0));
    // v.mul(Vec3f::from(0.5));
    // return v;


    let x = ((uv.u() * n).sin() + 1.0) * 0.5;
    let y = (((uv.u() + uv.v()) * n).sin() + 1.0) * 0.5;
    let z = ((uv.v() * n).cos() + 1.0) * 0.5;
    return Vec3f::new(x, y, z);
}

fn japan(mut uv: Vec2f) -> Vec3f {
    let r = 0.25;

    uv.sub(Vec2f::from(0.5));
    let b = (uv.sqr_len() > r * r) as i32 as f32;

    return Vec3f::new(1.0, b, b);
}

*/


const COLORS: [Vec3f; 2] = [
    // Vec3f::new_c(1.0, 0.0, 0.0),
    // Vec3f::new_c(0.0, 1.0, 1.0),

    // Vec3f::new_c(1.0, 1.0, 0.0),
    // Vec3f::new_c(0.0, 0.0, 1.0),

    // Vec3f::new_c(0.0, 1.0, 0.0),
    // Vec3f::new_c(1.0, 0.0, 1.0),

    Vec3f::new_c(0.0, 0.0, 0.0),
    Vec3f::new_c(1.0, 1.0, 1.0),
];


fn wang_circle(uv: Vec2f, mut bltr: u32) -> Vec3f {
    let r = 0.50_f32;


    let sides = [
        Vec2f::new(1.0, 0.5), // r
        Vec2f::new(0.5, 0.0), // t
        Vec2f::new(0.0, 0.5), // l
        Vec2f::new(0.5, 1.0), // b
    ];

    let mut result = Vec3f::from(0.0);
    for mut side in sides {
        side.sub(uv);
        let p = side.sqr_len().sqrt() / r;
        let t = 1.0 - minf!(p, 1.0_f32);

        result.lerp(&COLORS[(bltr & 1) as usize], &Vec3f::from(t));
        bltr = bltr >> 1;
    }

    result.pow(Vec3f::from(1.0 / 2.2));
    return result;
}

fn wang_tiangla(uv: Vec2f, mut bltr: u32) -> Vec3f {
    let ds = [
        1.0 - uv.x,     // r
        uv.y,           // t
        uv.x,           // l
        1.0 - uv.y,     // b
    ];

    let mut index = -1_isize;
    for i in 0..4_isize {
        if index < 0 || ds[index as usize] > ds[i as usize] {
            index = i;
        }
    }

    if ds[index as usize] > 0.25 {
        let intesity = bltr as f32 / 15.0;

        let mut result = Vec3f::from(intesity);
        result.lerp_l(&COLORS[1], &COLORS[0]);
        result.pow(Vec3f::from(1.0 / 2.2));

        return result;
    }

    while index > 0{

        bltr >>= 1;
        index -= 1;

    }

    return COLORS[(bltr as usize) & 1].clone();
}


smart_enum! {
#[derive(Clone, Copy, Debug)]
enum BLTR {
    Empty = 0,          //  = 0x0000,
    Right,          //  = 0x0001,
    Top,            //  = 0x0010,
    TopRight,       //  = 0x0011,
    Left,           //  = 0x0100,
    LeftRight,      //  = 0x0101,
    LeftTop,        //  = 0x0110,
    LeftRightTop,   //  = 0x0111,
    Bottom,         //  = 0x1000,
    BottomRight,    //  = 0x1001,
    TopBottom,      //  = 0x1010,
    LeftTopBottom,  //  = 0x1011,
    RightBottom,    //  = 0x1100,
    RightLeftBottom,//  = 0x1101,
    RightTopBottom, //  = 0x1110,
    Full,           //  = 0x1111,
}}

fn rand_tile(value: BLTR, position: BLTR) -> BLTR {
    let mut canditates = [BLTR::Empty; 16];
    let mut n = 0;

    for c in 0..16 {
        if (c & position as i32) == (value as i32 & position as i32) {
            canditates[n] = BLTR::try_from(c).unwrap();
            n += 1;
        }
    }


    let num = rand::thread_rng().gen_range(0..n);
    return canditates[num];
}


fn main() {
    const wang: fn(Vec2f, u32) -> Vec3f = wang_tiangla;

    let mut grid = image::ImageBuffer::new(GRID_WIDTH, GRID_HEIGHT);
    let mut tile = image::ImageBuffer::new(TILE_WIDTH, TILE_HEIGHT);

    let start = time::Instant::now();

    const TILES__: usize = ROW * COL;
    let mut atlas_values = [BLTR::Empty; TILES__];


    atlas_values[0] = rand_tile(BLTR::Empty, BLTR::Empty);
    for x in 1..ROW {
        let value = BLTR::try_from(((atlas_values[x - 1] as u32) & 1) << 2).unwrap();
        atlas_values[x] = rand_tile(value, BLTR::Left);
    }
    for y in 1..COL {
        let value = BLTR::try_from(((atlas_values[(y - 1) * ROW] as u32) & 8) >> 2).unwrap();
        atlas_values[y * ROW] = rand_tile(value, BLTR::Top);
    }
    for y in 1..COL {
        for x in 1..ROW {
            let value_t = ((atlas_values[y * ROW + x - 1] as u32) & 1) << 2;
            let value_l = ((atlas_values[(y - 1) * ROW + x] as u32) & 8) >> 2;

            let value = BLTR::try_from(value_l | value_t).unwrap();
            let position = BLTR::try_from(BLTR::Left as u32 | BLTR::Top as u32).unwrap();


            atlas_values[y * ROW + x] = rand_tile(value, position);
        }
    }


    // dbg!(&atlas_values);

    let mut atlas = image::ImageBuffer::new(TILE_WIDTH * 4, TILE_HEIGHT * 4);
//---- ATLAS

    {
        let mut tt = image::ImageBuffer::new(TILE_WIDTH, TILE_HEIGHT);

        let mut y = -1;
        let mut x;

        for i in 0..16 {
            generate_image_wang(&mut tt, wang, i as u32);

            x = i % 4;
            if x == 0 { y += 1; }

            atlas.copy_from(&tt, x as u32 * TILE_WIDTH, y as u32 * TILE_HEIGHT).unwrap();
        }
    }

//---- GRID
    {
        let mut y = -1_i32;
        let mut x;
        for (i, v) in atlas_values.iter().enumerate() {
            generate_image_wang(&mut tile, wang, (*v) as u32);

            x = i % ROW;
            if x == 0 { y += 1; }

            grid.copy_from(&tile, x as u32 * TILE_WIDTH, y as u32 * TILE_HEIGHT).unwrap();
        }
    }
    grid.save("./out_pic/wang_tiangla.png").unwrap();
    atlas.save("./out_pic/atlas_wang_tiangla.png").unwrap();
    println!("TOTAL: {}", start.elapsed().as_secs_f64());
}