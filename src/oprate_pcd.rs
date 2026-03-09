use anyhow::Result;
use pcd_rs::{PcdDeserialize, PcdSerialize, Reader};

#[derive(Debug, Clone, PcdDeserialize, PcdSerialize)]
pub struct PointXYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PcdDeserialize, PcdSerialize)]
pub struct PointXYZCovs {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub cov11: f32,
    pub cov12: f32,
    pub cov13: f32,
    pub cov21: f32,
    pub cov22: f32,
    pub cov23: f32,
    pub cov31: f32,
    pub cov32: f32,
    pub cov33: f32,
}

#[derive(Debug, Clone, PcdDeserialize, PcdSerialize)]
pub struct PointXYZWithShapeFeat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub linearity: f64,
    pub planarity: f64,
    pub scattering: f64,
    pub l1: f64,
    pub l2: f64,
    pub l3: f64,
    pub normal_z: f64,
}

#[derive(Debug, Clone, PcdDeserialize, PcdSerialize)]
pub struct PointXYZNormal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub normal_x: f32,
    pub normal_y: f32,
    pub normal_z: f32,
}

#[derive(Debug, Clone, PcdDeserialize, PcdSerialize)]
pub struct PointXYZRGB {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rgb: f32,
}

pub fn load_pcd_xyz(file_path: &str) -> Result<Vec<PointXYZ>> {
    let reader = match Reader::open(file_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open PCD file: {}", e);
            return Err(anyhow::anyhow!("Failed to open PCD file: {}", e));
        }
    };

    let points: Vec<PointXYZ> = match reader.collect() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to read PCD data: {}", e);
            return Err(anyhow::anyhow!("Failed to read PCD data: {}", e));
        }
    };

    Ok(points)
}

pub fn load_pcd_xyzrgb(file_path: &str) -> Result<Vec<PointXYZ>> {
    let reader = match Reader::open(file_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open PCD file: {}", e);
            return Err(anyhow::anyhow!("Failed to open PCD file: {}", e));
        }
    };

    let points_rgb: Vec<PointXYZRGB> = match reader.collect() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to read PCD data: {}", e);
            return Err(anyhow::anyhow!("Failed to read PCD data: {}", e));
        }
    };

    let points = points_rgb
        .into_iter()
        .map(|p| PointXYZ {
            x: p.x,
            y: p.y,
            z: p.z,
        })
        .collect();

    Ok(points)
}

pub fn save_pcd(points: &[PointXYZNormal], file_path: &str) -> Result<()> {
    let mut writer = pcd_rs::WriterInit {
        width: 1,
        height: points.len() as u64,
        viewpoint: Default::default(),
        data_kind: pcd_rs::DataKind::Ascii,
        schema: None,
    }
    .create(file_path)?;

    for point in points {
        writer.push(point)?;
    }
    writer.finish()?;

    Ok(())
}

pub fn save_xyz_pcd(points: &[PointXYZ], file_path: &str) -> Result<()> {
    let mut writer = pcd_rs::WriterInit {
        width: 1,
        height: points.len() as u64,
        viewpoint: Default::default(),
        data_kind: pcd_rs::DataKind::Ascii,
        schema: None,
    }
    .create(file_path)?;

    for point in points {
        writer.push(point)?;
    }
    writer.finish()?;

    Ok(())
}

pub fn save_pcd_with_covs(points: &[PointXYZCovs], file_path: &str) -> Result<()> {
    let mut writer = pcd_rs::WriterInit {
        width: 1,
        height: points.len() as u64,
        viewpoint: Default::default(),
        data_kind: pcd_rs::DataKind::Ascii,
        schema: None,
    }
    .create(file_path)?;

    for point in points {
        writer.push(point)?;
    }
    writer.finish()?;

    Ok(())
}

pub fn save_pcd_with_shape_feats(points: &[PointXYZWithShapeFeat], file_path: &str) -> Result<()> {
    let mut writer = pcd_rs::WriterInit {
        width: 1,
        height: points.len() as u64,
        viewpoint: Default::default(),
        data_kind: pcd_rs::DataKind::Ascii,
        schema: None,
    }
    .create(file_path)?;

    for point in points {
        writer.push(point)?;
    }
    writer.finish()?;

    Ok(())
}

pub fn convert_vecf32_to_pcd_xyz(points: &[[f32; 3]]) -> Vec<PointXYZ> {
    points
        .iter()
        .map(|p| PointXYZ {
            x: p[0],
            y: p[1],
            z: p[2],
        })
        .collect()
}

pub fn convert_vecf32_to_pcd_xyz_covs(points: &[[f32; 3]], covs: &[[f32; 9]]) -> Vec<PointXYZCovs> {
    points
        .iter()
        .zip(covs.iter())
        .map(|(p, c)| PointXYZCovs {
            x: p[0],
            y: p[1],
            z: p[2],
            cov11: c[0],
            cov12: c[1],
            cov13: c[2],
            cov21: c[3],
            cov22: c[4],
            cov23: c[5],
            cov31: c[6],
            cov32: c[7],
            cov33: c[8],
        })
        .collect()
}

pub fn convert_pcd_xyz_to_xyz_color(points: &[PointXYZ], color: (u8, u8, u8)) -> Vec<PointXYZRGB> {
    points
        .iter()
        .map(|p| PointXYZRGB {
            x: p.x,
            y: p.y,
            z: p.z,
            rgb: rgb_to_float(color.0, color.1, color.2),
        })
        .collect()
}

pub fn rgb_to_float(r: u8, g: u8, b: u8) -> f32 {
    let rgb_int: u32 = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    f32::from_bits(rgb_int)
}

pub fn save_pcd_with_color(points: &[PointXYZRGB], file_path: &str) -> Result<()> {
    let colored_points: Vec<PointXYZRGB> = points
        .iter()
        .map(|p| PointXYZRGB {
            x: p.x,
            y: p.y,
            z: p.z,
            rgb: p.rgb,
        })
        .collect();

    let mut writer = pcd_rs::WriterInit {
        width: 1,
        height: colored_points.len() as u64,
        viewpoint: Default::default(),
        data_kind: pcd_rs::DataKind::Ascii,
        schema: None,
    }
    .create(file_path)?;

    for point in &colored_points {
        writer.push(point)?;
    }
    writer.finish()?;

    Ok(())
}
