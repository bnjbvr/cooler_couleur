use image;

pub enum DataRange<T> {
    Automatic,
    MinMax(T, T),
}

pub trait CastAsUsize {
    fn from_usize(x: usize) -> Self;
    fn to_usize(x: Self) -> usize;
}

pub trait ColorRegression:
    Sized
    + std::fmt::Display
    + Copy
    + CastAsUsize
    + std::ops::Sub<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Add<Output = Self>
    + std::cmp::PartialOrd
    + std::cmp::PartialEq
{
    fn min(data: &[Self]) -> Self;
    fn max(data: &[Self]) -> Self;
}

macro_rules! impl_cast {
    ($t:ty) => {
        impl CastAsUsize for $t {
            #[inline(always)]
            fn from_usize(x: usize) -> $t {
                x as $t
            }
            #[inline(always)]
            fn to_usize(x: Self) -> usize {
                x as usize
            }
        }
    };
}

impl_cast!(u8);
impl_cast!(u16);
impl_cast!(u32);

impl_cast!(f32);
impl_cast!(f64);

macro_rules! impl_fp {
    ($t:ident) => {
        impl ColorRegression for $t {
            #[inline(always)]
            fn min(a: &[$t]) -> $t {
                // f32/f64 don't have Ord, but if all the numbers aren't NaN, PartialOrd is
                // equivalent to Ord.
                assert!(a.iter().all(|x| !x.is_nan()));
                a.iter()
                    .fold(std::$t::INFINITY, |x, y| if x < *y { x } else { *y })
            }

            #[inline(always)]
            fn max(a: &[$t]) -> $t {
                // f32/f64 don't have Ord, but if all the numbers aren't NaN, PartialOrd is
                // equivalent to Ord.
                assert!(a.iter().all(|x| !x.is_nan()));
                a.iter()
                    .fold(-std::$t::INFINITY, |x, y| if x > *y { x } else { *y })
            }
        }
    };
}

impl_fp!(f32);
impl_fp!(f64);

macro_rules! impl_uint {
    ($t:ty) => {
        impl ColorRegression for $t {
            fn min(a: &[$t]) -> $t {
                *a.iter().min().unwrap()
            }
            fn max(a: &[$t]) -> $t {
                *a.iter().max().unwrap()
            }
        }
    };
}

impl_uint!(u8);
impl_uint!(u16);
impl_uint!(u32);

pub fn color<T>(
    w: u32,
    h: u32,
    data: &[T],
    color_map: &[[u8; 3]],
    data_range: DataRange<T>,
) -> image::RgbImage
where
    T: ColorRegression,
{
    assert_eq!((w as usize) * (h as usize), data.len());
    let mut img = image::RgbImage::new(w, h);

    if data.is_empty() {
        return img;
    }

    let (min, max) = match data_range {
        DataRange::Automatic => (T::min(&data), T::max(&data)),
        DataRange::MinMax(min, max) => (min, max),
    };

    assert!(min <= max);
    if min == max {
        // No contrast! Might as well return an unset image.
        return img;
    }

    // Interpolate the colormap index from the data value, based on min and max data values.
    // Determine a and b in f(x) = ax+b, such that
    //  . f(min) = 0;
    //  . f(max) = color_map.len() - 1
    let slope = T::from_usize(color_map.len() - 1) / (max - min);
    let minus_y_offset = slope * min;

    // TODO if T:=u8, max=255, min=0, color_map.len()<255, then slope is 0.
    // probably better use f64!
    println!(
        "map_len={}, slope={}, y_offset=-{}",
        color_map.len() - 1,
        slope,
        minus_y_offset
    );

    for y in 0..h {
        let l = y * h;
        for x in 0..w {
            let v = data[(l + x) as usize];
            debug_assert!(v >= min);
            debug_assert!(v <= max);

            let i = T::to_usize(slope * v - minus_y_offset);
            assert!(i < color_map.len());
            img.put_pixel(x, y, image::Rgb(color_map[i]));
        }
    }

    img
}
