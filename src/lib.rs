#![warn(missing_docs)]
//! Cross-Platform GUI Application Framework for rust
//!
use git_version::git_version;


const GIT_VERSION: &str = git_version!();
/// Returns the gafw crate version
/// 
/// # Examples
/// 
/// ```
/// // Prints the crate version
/// println!( "Version {}", gafw::version() );
/// ```
pub fn version() -> String {
    std::format!("{} {}", std::env!("CARGO_PKG_VERSION"), GIT_VERSION )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_version_funcion() {
        let result = version();
        let expecting = std::format!("0.1.0 {}", GIT_VERSION);
        assert_eq!(result, expecting);
    }
}
