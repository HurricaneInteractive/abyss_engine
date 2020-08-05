use raylib::math::{Vector2};

pub trait ToVector2 {
    fn to_vector2(&self) -> Vector2;
}

/// Exports the property to a yml file, this will be used in the game engine to assign to a entity
/// and used to generate the entity config files
pub trait ToProperty {
    /// This method should generate a yml with the structure of property
    ///
    /// ```rust
    /// use std::io::{Error};
    /// use abyss_engine::math::{Vec2};
    /// use abyss_engine::transform::{Transform};
    ///
    /// impl ToProperty for Transform {
    ///     fn generate() -> Result<(), Error> {
    ///         // some logic to generate the yml file
    ///         // -> transform.yml generated
    ///     }
    /// }
    /// ```
    fn generate() -> Result<(), Box<dyn std::error::Error>>;
}