pub mod heligman_pollard;
pub mod plot;

use plot::*;


#[repr(C)]
pub struct Buffer {
    data: *mut u8,
    len: usize,
    capacity: usize,
}

impl From<Vec<u8>> for Buffer {
    fn from(mut vec: Vec<u8>) -> Self {
        let buf = Buffer {
            data: vec.as_mut_ptr(),
            len: vec.len(),
            capacity: vec.capacity(),
        };
        std::mem::forget(vec);
        buf
    }
}

#[repr(C)]
pub struct Graphs {
    mortality: Buffer,
    infant_mortality: Buffer,
    first_year_mortality: Buffer,
    infant_mortality_dropoff: Buffer,
    accident_severity: Buffer,
    accident_spread: Buffer,
    accident_midpoint: Buffer,
    adult_mortality: Buffer,
    adult_mortality_increase: Buffer,
}

#[no_mangle]
pub extern "C" fn plot_8d_graphs(
    infant_mortality: f32,
    first_year_mortality: f32,
    infant_mortality_dropoff: f32,
    accident_severity: f32,
    accident_spread: f32,
    accident_midpoint: f32,
    adult_mortality: f32,
    adult_mortality_increase: f32,
) -> Graphs {

    macro_rules! plot_vec {
        ($plotname:ident) => {
            $plotname(
                infant_mortality,
                first_year_mortality,
                infant_mortality_dropoff,
                accident_severity,
                accident_spread,
                accident_midpoint,
                adult_mortality,
                adult_mortality_increase,
            ).unwrap()
        };
    }    

    let mortality = plot_vec!(plot_2d_mortality).into();

    let infant_mortality = plot_vec!(plot_3d_infant_mortality).into();

    let first_year_mortality = plot_vec!(plot_3d_first_year_mortality).into();

    let infant_mortality_dropoff = plot_vec!(plot_3d_infant_mortality_dropoff).into();

    let accident_severity = plot_vec!(plot_3d_accident_severity).into();

    let accident_spread = plot_vec!(plot_3d_accident_spread).into();

    let accident_midpoint = plot_vec!(plot_3d_accident_midpoint).into();

    let adult_mortality = plot_vec!(plot_3d_adult_mortality).into();

    let adult_mortality_increase = plot_vec!(plot_3d_adult_mortality_increase).into();

    Graphs { 
        mortality,
        infant_mortality,
        first_year_mortality,
        infant_mortality_dropoff,
        accident_severity,
        accident_spread,
        accident_midpoint,
        adult_mortality,
        adult_mortality_increase,
    }
}

#[no_mangle]
pub extern "C" fn free_buffer(buf: Buffer) {
    unsafe {
        Vec::from_raw_parts(buf.data, buf.len, buf.capacity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let result = heligman_pollard::baseline_female(2);
        assert_eq!(result, 0.00026683632);
    }

    #[test]
    fn plot_mortality() {
        plot::plot_2d_mortality(
            0.5,
            0.0192,
            0.1048,
            0.5,
            9.0,
            21.0,
            0.0001,
            1.1
        ).unwrap();
    }

    #[test]
    fn plot_infant_mortality() {
        plot::plot_3d_infant_mortality(
            0.5,
            0.0192,
            0.1048,
            0.5,
            9.0,
            21.0,
            0.0001,
            1.1
        ).unwrap();
    }

    #[test]
    fn plot_first_year_mortality() {
        plot::plot_3d_first_year_mortality(
            0.5,
            0.0192,
            0.1048,
            0.5,
            9.0,
            21.0,
            0.0001,
            1.1
        ).unwrap();
    }

    #[test]
    fn plot_infant_mortality_dropoff() {
        plot::plot_3d_infant_mortality_dropoff(
            0.5,
            0.0192,
            0.1048,
            0.5,
            9.0,
            21.0,
            0.0001,
            1.1
        ).unwrap();
    }

    #[test]
    fn plot_accident_severity() {
        plot::plot_3d_accident_severity(
            0.5,
            0.0192,
            0.1048,
            0.5,
            9.0,
            21.0,
            0.0001,
            1.1
        ).unwrap();
    }

    #[test]
    fn plot_accident_spread() {
        plot::plot_3d_accident_spread(
            0.5,
            0.0192,
            0.1048,
            0.5,
            9.0,
            21.0,
            0.0001,
            1.1
        ).unwrap();
    }

    #[test]
    fn plot_accident_midpoint() {
        plot::plot_3d_accident_midpoint(
            0.5,
            0.0192,
            0.1048,
            0.5,
            9.0,
            21.0,
            0.0001,
            1.1
        ).unwrap();
    }

    #[test]
    fn plot_adult_mortality() {
        plot::plot_3d_adult_mortality(
            0.5,
            0.0192,
            0.1048,
            0.5,
            9.0,
            21.0,
            0.5,
            1.1
        ).unwrap();
    }

    #[test]
    fn plot_adult_mortality_dropoff() {
        plot::plot_3d_adult_mortality_increase(
            0.5,
            0.0192,
            0.1048,
            0.5,
            9.0,
            21.0,
            0.0001,
            1.1
        ).unwrap();
    }

}
