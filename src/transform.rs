use serde::{Deserialize, Serialize};
use crate::config::{ReadConfig, generate_struct_yml};
use crate::math::Vec2;
use crate::traits::ToProperty;
use std::ptr::null;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Transform {
    pub position: Vec2
}

#[derive(Deserialize, Serialize)]
pub struct TransformConfig {
    transform: Transform
}

impl ReadConfig<Transform> for Transform {
    fn read(s: &str) -> Result<Transform, serde_yaml::Error> {
        let deserialized: TransformConfig = serde_yaml::from_str(&s)?;
        Ok(deserialized.transform)
    }
}

impl ToProperty for TransformConfig {
    fn generate() -> Result<(), Box<dyn std::error::Error>> {
        let t = TransformConfig {
            transform: Transform {
                position: Vec2 {
                    x: 0.0,
                    y: 0.0
                }
            }
        };

        generate_struct_yml("./transform-config.yml", &t)?;

        Ok(())
    }
}
