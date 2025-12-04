use cudarc::{driver::{CudaContext, DeviceRepr, LaunchConfig, PushKernelArg, ValidAsZeroBits}, nvrtc::Ptx};
use anyhow::Result;
use cudarc_practice::file_handler::load_pcd_xyzt;
use core::f32;
use std::time::Instant;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

unsafe impl DeviceRepr for Point3 {}
unsafe impl ValidAsZeroBits for Point3 {}

fn main() -> Result<()> {
    let pcd_path = "data/input/frame_100.pcd";
    let points = match load_pcd_xyzt(pcd_path) {
        Ok(p) => p,
        Err(e) => {
            // eprintln!("Error loading PCD file: {}", e);
            return Err(e);
        }
    };

    let points_vec: Vec<Point3> = points.iter().map(|p| {
        Point3 {
            x: p.x,
            y: p.y,
            z: p.z
        }
    }).collect();

    let mut min_bound = Point3 { x: f32::MAX, y: f32::MAX, z: f32::MAX };
    let mut max_bound = Point3 { x: f32::MIN, y: f32::MIN, z: f32::MIN };

    for p in &points_vec {
        min_bound.x = min_bound.x.min(p.x);
        min_bound.y = min_bound.y.min(p.y);
        min_bound.z = min_bound.z.min(p.z);
        max_bound.x = max_bound.x.max(p.x);
        max_bound.y = max_bound.y.max(p.y);
        max_bound.z = max_bound.z.max(p.z);
    }

    let range_x = max_bound.x - min_bound.x;
    let range_y = max_bound.y - min_bound.y;
    let range_z = max_bound.z - min_bound.z;
    let max_range = range_x.max(range_y).max(range_z);
    let scale = 2097152.0 / max_range; // 2^21 - 1

    println!("Point cloud stats:");
    println!("  Number of points: {}", points_vec.len());
    println!("  Min bound: ({:.3}, {:.3}, {:.3})", min_bound.x, min_bound.y, min_bound.z);
    println!("  Max bound: ({:.3}, {:.3}, {:.3})", max_bound.x, max_bound.y, max_bound.z);
    println!("  Scale: {:.3}", scale);

    let ctx = CudaContext::new(0)?;
    let stream = ctx.default_stream();

    let points_dev = stream.clone_htod(&points_vec)?;

    let num_points = points.len();
    let mut codes_dev = stream.alloc_zeros::<u64>(num_points)?;
    let mut indices_dev = stream.alloc_zeros::<i32>(num_points)?;

    // Load the kernel
    let module = ctx.load_module(Ptx::from_file("src/kernel/morton3d.ptx"))?;
    let morton_kernel = module.load_function("compute_morton_codes")?;

    let cfg = LaunchConfig::for_num_elems(num_points as u32);

    let start_morton = Instant::now();

    unsafe {
        stream.launch_builder(&morton_kernel)
            .arg(&points_dev)
            .arg(&mut codes_dev)
            .arg(&mut indices_dev)
            .arg(&(num_points as i32))
            .arg(&min_bound)
            .arg(&scale)
            .launch(cfg)?;
    }

    stream.synchronize()?;

    let duration = start_morton.elapsed();
    println!("Morton3D kernel execution time: {:.3} ms", duration.as_secs_f64() * 1000.0);
    
    let morton_codes = stream.clone_dtoh(&codes_dev)?;
    let indices = stream.clone_dtoh(&indices_dev)?;

    println!("\nMorton codes (first 10):");
    for i in 0..10.min(morton_codes.len()) {
        println!("  Point {}: code = 0x{:016X}, index = {}", 
                 i, morton_codes[i], indices[i]);
    }

    println!("\nSorting on CPU...");
    let mut pairs: Vec<(u64, i32)> = morton_codes.iter()
        .zip(indices.iter())
        .map(|(&c, &i)| (c, i))
        .collect();
    pairs.sort_unstable_by_key(|k| k.0);

    let sorted_indices: Vec<i32> = pairs.iter().map(|k| k.1).collect();
    let sorted_codes: Vec<u64> = pairs.iter().map(|k| k.0).collect();

    println!("Sorted! First 10 codes:");
    for i in 0..10 {
        println!("  [{}] Code: 0x{:016X}, Original Index: {}", 
                i, sorted_codes[i], sorted_indices[i]);
    }

    let sorted_indices_dev = stream.clone_htod(&sorted_indices)?;
    let sorted_codes_dev = stream.clone_htod(&sorted_codes)?;

    let mut sorted_points_dev = stream.alloc_zeros::<Point3>(num_points)?;

    let module2 = ctx.load_module(Ptx::from_file("src/kernel/sort.ptx"))?;
    let sort_kernel = module2.load_function("sort_points")?;

    let start_sort = Instant::now();
    unsafe {
        stream.launch_builder(&sort_kernel)
            .arg(&points_dev)
            .arg(&mut sorted_points_dev)
            .arg(&sorted_indices_dev)
            .arg(&(num_points as i32))
            .launch(cfg)?;
    }
    stream.synchronize()?;
    let duration_sort = start_sort.elapsed();
    println!("Sort kernel execution time: {:.3} ms", duration_sort.as_secs_f64() * 1000.0);

    let sorted_points = stream.clone_dtoh(&sorted_points_dev)?;
    println!("First point: {:?}", sorted_points[0]);
    println!("Second point: {:?}", sorted_points[1]);
    println!("Third point: {:?}", sorted_points[2]);

    let query_points_raw: Vec<Point3> = points_vec.iter().take(500).cloned().collect();

    let num_query = query_points_raw.len();
    println!("Number of query points: {}", num_query);

    let query_points_dev = stream.clone_htod(&query_points_raw)?;

    println!("\nStarting simple search...");
    // let mut correspondence_indices_dev = stream.alloc_zeros::<i32>(num_points)?;
    // let mut correspondence_dists_dev = stream.alloc_zeros::<f32>(num_points)?;
    let mut output_indices_dev = stream.alloc_zeros::<i32>(num_query)?;
    let mut output_dists_dev = stream.alloc_zeros::<f32>(num_query)?;

    let module3 = ctx.load_module(Ptx::from_file("src/kernel/find_points.ptx"))?;
    let search_kernel = module3.load_function("find_correspondence_points")?;

    let search_window_size = 32;
    let num_query_points = num_query;

    let start_search = Instant::now();
    unsafe {
        stream.launch_builder(&search_kernel)
            // .arg(&sorted_points_dev)  // Source
            .arg(&query_points_dev)  // Source
            .arg(&sorted_points_dev)  // Target
            .arg(&sorted_codes_dev)
            // .arg(&mut correspondence_indices_dev)
            // .arg(&mut correspondence_dists_dev)
            .arg(&mut output_indices_dev)
            .arg(&mut output_dists_dev)
            .arg(&(num_query_points as i32))
            .arg(&(num_points as i32))
            .arg(&min_bound)
            .arg(&scale)
            .arg(&search_window_size)
            .launch(cfg)?;
    }
    stream.synchronize()?;
    let duration_search = start_search.elapsed();
    println!("Search kernel execution time: {:.3} ms", duration_search.as_secs_f64() * 1000.0);

    println!("Search completed.");
    println!("Search for {} points completed.", num_query);

    // let results_idx = stream.clone_dtoh(&correspondence_indices_dev)?;
    // let results_dist = stream.clone_dtoh(&correspondence_dists_dev)?;
    let results_idx = stream.clone_dtoh(&output_indices_dev)?;
    let results_dist = stream.clone_dtoh(&output_dists_dev)?;

    for i in 0..10 {
        println!("Point {}: Found neighbor index {}, DistSq = {:.6}", 
                i, results_idx[i], results_dist[i]);
    }

    Ok(())
}