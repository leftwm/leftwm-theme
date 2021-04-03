use crate::errors;
use crate::models::LeftWm;
/// # Errors
///
/// Returns error if the `LeftWM` version cannot be obtained.
/// Returns error if the `LeftWM` version requirements cannot be parsed.
/// Returns error is the `LeftWM` version cannot be parsed.
pub fn check(vstring: &str) -> Result<bool, errors::LeftError> {
    use semver::{Version, VersionReq};
    let lwmv = LeftWm::get()?;
    let requirements = VersionReq::parse(&vstring)?;
    Ok(requirements.matches(&Version::parse(&lwmv.version)?))
}
