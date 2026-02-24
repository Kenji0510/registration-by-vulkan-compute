use crate::oprate_pcd::PointXYZ;

pub fn pcd_to_vecf32(points: &[PointXYZ]) -> Vec<[f32; 3]> {
    points
        .iter()
        .map(|p| [p.x as f32, p.y as f32, p.z as f32])
        .collect()
}
