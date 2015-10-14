

use num;

/// Maps from normalized window coordinates to pixel coordinates.
pub fn map_point_to_pixel<T>(point: (T,T),
                             dimensions: (T,T)) -> (T,T)
    where T: num::Num {

    let one = T::one();
    let two = one+one;

    let (hwidth,hheight) = match dimensions {
        (w,h) => (w / two, h / two),
    };

    let (x,y) = point;

    let pixel_x = hwidth * (x+one);
    let pixel_y = hheight * (y+one);

    (pixel_x, pixel_y)
}

/// Maps from pixel coordinates to normalized window coordinates.
pub fn map_pixel_to_point<T>(point: (T,T),
                             dimensions: (T,T)) -> (T,T)
    where T: num::Num {
    
    let one = T::one();
    let two = one+one;

    let (inv_hwidth,inv_hheight) = match dimensions {
        (w,h) => (two / w, two / h),
    };

    let (x,y) = point;

    let norm_x = (inv_hwidth * x) - one;
    let norm_y = (inv_hheight * y) - one;

    (norm_x, norm_y)
}
