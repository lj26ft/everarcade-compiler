pub fn camera_projection(camera_id: &str, x: i32, y: i32, zoom: u32) -> String { crate::stable_hash(&["camera", camera_id, &x.to_string(), &y.to_string(), &zoom.to_string()]) }
