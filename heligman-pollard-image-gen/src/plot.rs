use plotters::prelude::*;
use crate::heligman_pollard;

pub const X_RESOLUTION: u32 = 200;
pub const Y_RESOLUTION: u32 = 200;
pub const RGB_PIXEL_SIZE: u32 = 3;
pub const IMG_BYTE_SIZE: usize = ((X_RESOLUTION*Y_RESOLUTION)*RGB_PIXEL_SIZE) as usize;
pub const SCALE: f64 = 0.65;

pub fn plot_2d_mortality(
    infant_mortality: f32,
    first_year_mortality: f32,
    infant_mortality_dropoff: f32,
    accident_severity: f32,
    accident_spread: f32,
    accident_midpoint: f32,
    adult_mortality: f32,
    adult_mortality_increase: f32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let mut buffer = vec![0u8; (850*375*RGB_PIXEL_SIZE) as usize];
    
    let area = BitMapBackend::with_buffer(&mut buffer, (850, 375)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (0i32..100).step(5);
    let y_axis = (0.0f32..1.0).step(0.5);
    
    let mut chart = ChartBuilder::on(&area)
        .set_all_label_area_size(40)
        .caption("Mortality Rate by Age: Heligman-Pollard Equation", ("sans-serif", 16))
        .build_cartesian_2d(x_axis.clone(), y_axis.clone())?;
    
    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(20)
        .y_label_formatter(&|y| format!("{:.1}", y))
        .draw()?;

    chart.draw_series(
        LineSeries::new(
            (0..=100)
                .map(|x:i32| {
                    (x, heligman_pollard::mortality_at_age(
                        x,
                        infant_mortality,
                        first_year_mortality,
                        infant_mortality_dropoff,
                        accident_severity,
                        accident_spread,
                        accident_midpoint,
                        adult_mortality,
                        adult_mortality_increase,
                    ))
                }
            ),
            &RED,
        )
    )?;

    drop(chart);
    drop(area);

    Ok(buffer)
}

pub fn plot_3d_infant_mortality(
    infant_mortality: f32,
    first_year_mortality: f32,
    infant_mortality_dropoff: f32,
    accident_severity: f32,
    accident_spread: f32,
    accident_midpoint: f32,
    adult_mortality: f32,
    adult_mortality_increase: f32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    let mut buffer = vec![0u8; IMG_BYTE_SIZE];
    
    let area = BitMapBackend::with_buffer(&mut buffer, (X_RESOLUTION, Y_RESOLUTION)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (0i32..100).step(5);
    let y_axis = (0.0f32..1.0).step(0.5);
    let z_axis = (0.0f32..1.0).step(0.5);
    
    let mut chart = ChartBuilder::on(&area)
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;
    
    chart.with_projection(|mut pb| {
        pb.yaw = -0.5;
        pb.pitch = 1.0;
        pb.scale = SCALE;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .draw()?;

    chart.draw_series(
        SurfaceSeries::xoz(
            0..=100,
            (0..=100).step_by(5).map(|x| x as f32 / 100.0),
            |x, y| {
                heligman_pollard::mortality_at_age(
                    x,
                    y,
                    first_year_mortality,
                    infant_mortality_dropoff,
                    accident_severity,
                    accident_spread,
                    accident_midpoint,
                    adult_mortality,
                    adult_mortality_increase,
                )
            },
        )
        .style(BLUE.mix(0.5))
    )?;

    chart.draw_series(
        LineSeries::new(
            (0..=100)
                .map(|x:i32| {
                    (x, heligman_pollard::mortality_at_age(
                        x,
                        infant_mortality,
                        first_year_mortality,
                        infant_mortality_dropoff,
                        accident_severity,
                        accident_spread,
                        accident_midpoint,
                        adult_mortality,
                        adult_mortality_increase,
                    ), infant_mortality)
                }
            ),
            &RED,
        )
    )?;

    drop(chart);
    drop(area);

    Ok(buffer)
}

pub fn plot_3d_first_year_mortality(
    infant_mortality: f32,
    first_year_mortality: f32,
    infant_mortality_dropoff: f32,
    accident_severity: f32,
    accident_spread: f32,
    accident_midpoint: f32,
    adult_mortality: f32,
    adult_mortality_increase: f32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let mut buffer = vec![0u8; IMG_BYTE_SIZE];
    
    let area = BitMapBackend::with_buffer(&mut buffer, (X_RESOLUTION, Y_RESOLUTION)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (0..100).step(5);
    let y_axis = (0.0f32..1.0).step(0.5);
    let z_axis = (0.0f32..1.0).step(0.5);
    
    let mut chart = ChartBuilder::on(&area)
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;
    
    chart.with_projection(|mut pb| {
        pb.yaw = -0.5;
        pb.pitch = 1.0;
        pb.scale = SCALE;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .draw()?;

    chart.draw_series(
        SurfaceSeries::xoz(
            0..=100,
            (0..=100).step_by(5).map(|y| y as f32 / 100.0),
            |x, y| {
                heligman_pollard::mortality_at_age(
                    x,
                    infant_mortality,
                    y,
                    infant_mortality_dropoff,
                    accident_severity,
                    accident_spread,
                    accident_midpoint,
                    adult_mortality,
                    adult_mortality_increase,
                )
            },
        )
        .style(BLUE.mix(0.5))
    )?;

    chart.draw_series(
        LineSeries::new(
            (0..=100)
                .map(|x:i32| {
                    (x, heligman_pollard::mortality_at_age(
                        x,
                        infant_mortality,
                        first_year_mortality,
                        infant_mortality_dropoff,
                        accident_severity,
                        accident_spread,
                        accident_midpoint,
                        adult_mortality,
                        adult_mortality_increase,
                    ), first_year_mortality)
                }
            ),
            &RED,
        )
    )?;

    drop(chart);
    drop(area);

    Ok(buffer)
}

pub fn plot_3d_infant_mortality_dropoff(
    infant_mortality: f32,
    first_year_mortality: f32,
    infant_mortality_dropoff: f32,
    accident_severity: f32,
    accident_spread: f32,
    accident_midpoint: f32,
    adult_mortality: f32,
    adult_mortality_increase: f32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let mut buffer = vec![0u8; IMG_BYTE_SIZE];
    
    let area = BitMapBackend::with_buffer(&mut buffer, (X_RESOLUTION, Y_RESOLUTION)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (0..100).step(5);
    let y_axis = (0.0f32..1.0).step(0.5);
    let z_axis = (0.0f32..1.0).step(0.5);
    
    let mut chart = ChartBuilder::on(&area)
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;
    
    chart.with_projection(|mut pb| {
        pb.yaw = -0.5;
        pb.pitch = 1.0;
        pb.scale = SCALE;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .draw()?;

    chart.draw_series(
        SurfaceSeries::xoz(
            0..=100,
            (0..=100).step_by(5).map(|z| z as f32 / 100.0),
            |x, z| {
                heligman_pollard::mortality_at_age(
                    x,
                    infant_mortality,
                    first_year_mortality,
                    z,
                    accident_severity,
                    accident_spread,
                    accident_midpoint,
                    adult_mortality,
                    adult_mortality_increase,
                )
            },
        )
        .style(BLUE.mix(0.5))
    )?;

    chart.draw_series(
        LineSeries::new(
            (0..=100)
                .map(|x:i32| {
                    (x, heligman_pollard::mortality_at_age(
                        x,
                        infant_mortality,
                        first_year_mortality,
                        infant_mortality_dropoff,
                        accident_severity,
                        accident_spread,
                        accident_midpoint,
                        adult_mortality,
                        adult_mortality_increase,
                    ), infant_mortality_dropoff)
                }
            ),
            &RED,
        )
    )?;

    drop(chart);
    drop(area);

    Ok(buffer)
}

pub fn plot_3d_accident_severity(
    infant_mortality: f32,
    first_year_mortality: f32,
    infant_mortality_dropoff: f32,
    accident_severity: f32,
    accident_spread: f32,
    accident_midpoint: f32,
    adult_mortality: f32,
    adult_mortality_increase: f32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let mut buffer = vec![0u8; IMG_BYTE_SIZE];
    
    let area = BitMapBackend::with_buffer(&mut buffer, (X_RESOLUTION, Y_RESOLUTION)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (0..100).step(5);
    let y_axis = (0.0f32..1.0).step(0.5);
    let z_axis = (0.0f32..1.0).step(0.5);
    
    let mut chart = ChartBuilder::on(&area)
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;
    
    chart.with_projection(|mut pb| {
        pb.yaw = -0.5;
        pb.pitch = 1.0;
        pb.scale = SCALE;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .draw()?;

    chart.draw_series(
        SurfaceSeries::xoz(
            0..=100,
            (0..=100).step_by(5).map(|z| z as f32 / 100.0),
            |x, z| {
                heligman_pollard::mortality_at_age(
                    x,
                    infant_mortality,
                    first_year_mortality,
                    infant_mortality_dropoff,
                    z,
                    accident_spread,
                    accident_midpoint,
                    adult_mortality,
                    adult_mortality_increase,
                )
            },
        )
        .style(BLUE.mix(0.5))
    )?;

    chart.draw_series(
        LineSeries::new(
            (0..=100)
                .map(|x:i32| {
                    (x, heligman_pollard::mortality_at_age(
                        x,
                        infant_mortality,
                        first_year_mortality,
                        infant_mortality_dropoff,
                        accident_severity,
                        accident_spread,
                        accident_midpoint,
                        adult_mortality,
                        adult_mortality_increase,
                    ), accident_severity)
                }
            ),
            &RED,
        )
    )?;

    drop(chart);
    drop(area);

    Ok(buffer)
}

pub fn plot_3d_accident_spread(
    infant_mortality: f32,
    first_year_mortality: f32,
    infant_mortality_dropoff: f32,
    accident_severity: f32,
    accident_spread: f32,
    accident_midpoint: f32,
    adult_mortality: f32,
    adult_mortality_increase: f32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let mut buffer = vec![0u8; IMG_BYTE_SIZE];
    
    let area = BitMapBackend::with_buffer(&mut buffer, (X_RESOLUTION, Y_RESOLUTION)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (0..100).step(5);
    let y_axis = (0.0f32..1.0).step(0.5);
    let z_axis = (0.0f32..100.0).step(5.0);
    
    let mut chart = ChartBuilder::on(&area)
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;
    
    chart.with_projection(|mut pb| {
        pb.yaw = -0.5;
        pb.pitch = 1.0;
        pb.scale = SCALE;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .draw()?;

    chart.draw_series(
        SurfaceSeries::xoz(
            0..=100,
            (0..=20).map(|z| (z*5) as f32),
            |x, z| {
                heligman_pollard::mortality_at_age(
                    x,
                    infant_mortality,
                    first_year_mortality,
                    infant_mortality_dropoff,
                    accident_severity,
                    z as f32,
                    accident_midpoint,
                    adult_mortality,
                    adult_mortality_increase,
                )
            },
        )
        .style(BLUE.mix(0.5))
    )?;

    chart.draw_series(
        LineSeries::new(
            (0..=100)
                .map(|x:i32| {
                    (x, heligman_pollard::mortality_at_age(
                        x,
                        infant_mortality,
                        first_year_mortality,
                        infant_mortality_dropoff,
                        accident_severity,
                        accident_spread,
                        accident_midpoint,
                        adult_mortality,
                        adult_mortality_increase,
                    ), accident_spread)
                }
            ),
            &RED,
        )
    )?;

    drop(chart);
    drop(area);

    Ok(buffer)
}

pub fn plot_3d_accident_midpoint(
    infant_mortality: f32,
    first_year_mortality: f32,
    infant_mortality_dropoff: f32,
    accident_severity: f32,
    accident_spread: f32,
    accident_midpoint: f32,
    adult_mortality: f32,
    adult_mortality_increase: f32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let mut buffer = vec![0u8; IMG_BYTE_SIZE];
    
    let area = BitMapBackend::with_buffer(&mut buffer, (X_RESOLUTION, Y_RESOLUTION)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (0..100).step(5);
    let y_axis = (0.0f32..1.0).step(0.5);
    let z_axis = (0.0f32..100.0).step(5.0);
    
    let mut chart = ChartBuilder::on(&area)
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;
    
    chart.with_projection(|mut pb| {
        pb.yaw = -0.5;
        pb.pitch = 1.0;
        pb.scale = SCALE;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .draw()?;

    chart.draw_series(
        SurfaceSeries::xoz(
            0..=100,
            (0..=20).map(|z| (z*5) as f32),
            |x, z| {
                heligman_pollard::mortality_at_age(
                    x,
                    infant_mortality,
                    first_year_mortality,
                    infant_mortality_dropoff,
                    accident_severity,
                    accident_spread,
                    z as f32,
                    adult_mortality,
                    adult_mortality_increase,
                )
            },
        )
        .style(BLUE.mix(0.5))
    )?;

    chart.draw_series(
        LineSeries::new(
            (0..=100)
                .map(|x:i32| {
                    (x, heligman_pollard::mortality_at_age(
                        x,
                        infant_mortality,
                        first_year_mortality,
                        infant_mortality_dropoff,
                        accident_severity,
                        accident_spread,
                        accident_midpoint,
                        adult_mortality,
                        adult_mortality_increase,
                    ), accident_midpoint)
                }
            ),
            &RED,
        )
    )?;

    drop(chart);
    drop(area);

    Ok(buffer)
}

pub fn plot_3d_adult_mortality(
    infant_mortality: f32,
    first_year_mortality: f32,
    infant_mortality_dropoff: f32,
    accident_severity: f32,
    accident_spread: f32,
    accident_midpoint: f32,
    adult_mortality: f32,
    adult_mortality_increase: f32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let mut buffer = vec![0u8; IMG_BYTE_SIZE];
    
    let area = BitMapBackend::with_buffer(&mut buffer, (X_RESOLUTION, Y_RESOLUTION)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (0..100).step(5);
    let y_axis = (0.0f32..1.0).step(0.5);
    let z_axis = (0.0f32..1.0).step(0.5);

    let mut chart = ChartBuilder::on(&area)
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;

    chart.with_projection(|mut pb| {
        pb.yaw = -0.5;
        pb.pitch = 1.0;
        pb.scale = SCALE;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .draw()?;

    chart.draw_series(
        SurfaceSeries::xoz(
            0..=100,
            (0..=100).step_by(5).map(|z| z as f32 / 100.0),
            |x, z| {
                heligman_pollard::mortality_at_age(
                    x,
                    infant_mortality,
                    first_year_mortality,
                    infant_mortality_dropoff,
                    accident_severity,
                    accident_spread,
                    accident_midpoint,
                    z as f32,
                    adult_mortality_increase,
                )
            },
        )
        .style(BLUE.mix(0.5))
    )?;

    chart.draw_series(
        LineSeries::new(
            (0..=100)
                .map(|x:i32| {
                    (x, heligman_pollard::mortality_at_age(
                        x,
                        infant_mortality,
                        first_year_mortality,
                        infant_mortality_dropoff,
                        accident_severity,
                        accident_spread,
                        accident_midpoint,
                        adult_mortality,
                        adult_mortality_increase,
                    ), adult_mortality)
                }
            ),
            &RED,
        )
    )?;
    
    drop(chart);
    drop(area);

    Ok(buffer)
}

pub fn plot_3d_adult_mortality_increase(
    infant_mortality: f32,
    first_year_mortality: f32,
    infant_mortality_dropoff: f32,
    accident_severity: f32,
    accident_spread: f32,
    accident_midpoint: f32,
    adult_mortality: f32,
    adult_mortality_increase: f32,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    
    let mut buffer = vec![0u8; IMG_BYTE_SIZE];
    
    let area = BitMapBackend::with_buffer(&mut buffer, (X_RESOLUTION, Y_RESOLUTION)).into_drawing_area();

    area.fill(&WHITE)?;

    let x_axis = (0..100).step(5);
    let y_axis = (0.0f32..1.0).step(0.5);
    let z_axis = (1.0f32..2.0).step(0.5);

    let mut chart = ChartBuilder::on(&area)
        .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;

    chart.with_projection(|mut pb| {
        pb.yaw = -0.5;
        pb.pitch = 1.0;
        pb.scale = SCALE;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .draw()?;

    chart.draw_series(
        SurfaceSeries::xoz(
            0..=100,
            (10..=20).map(|z| z as f32/10.0),
            |x, z| {
                heligman_pollard::mortality_at_age(
                    x,
                    infant_mortality,
                    first_year_mortality,
                    infant_mortality_dropoff,
                    accident_severity,
                    accident_spread,
                    accident_midpoint,
                    adult_mortality,
                    z as f32,
                )
            },
        )
        .style(BLUE.mix(0.5))
    )?;

    chart.draw_series(
        LineSeries::new(
            (0..=100)
                .map(|x:i32| {
                    (x, heligman_pollard::mortality_at_age(
                        x,
                        infant_mortality,
                        first_year_mortality,
                        infant_mortality_dropoff,
                        accident_severity,
                        accident_spread,
                        accident_midpoint,
                        adult_mortality,
                        adult_mortality_increase,
                    ), adult_mortality_increase)
                }
            ),
            &RED,
        )
    )?;
    
    drop(chart);
    drop(area);

    Ok(buffer)
}
