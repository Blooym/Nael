use crate::net::RemoteResource;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fs::read_to_string, path::Path, str::FromStr};

// Note: https://kamori.goats.dev/Dalamud/Release/Meta exists and has caches of Dalamud releases and is how
// the official launchers perform updates.
// Right now this URL isn't being used, but it could be in the future to fetch releases in a better way, so
// it is being left as a note here.

/// Version information for a Dalamud release.
///
/// # Warning
/// The structure of version data may be changed by a upstream source at any time without warning
/// and break existing installations. Care should be put into handling cases like this where possible.
///
/// # Compatibility
/// This struct was built by manually looking at the `version` file on the official `goatcorp/dalamud-distrib` repository.
/// it may not work with 3rd party release sources.
///
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct DalamudVersionInfo {
    /// Depending on the branch, this will either be the assembly version `major.minor.patch.revision` or a Git commit hash.
    /// Somewhat confusing, this field is named 'assembly_version' remotely anyway.
    pub assembly_version: String,
    /// The supported version of FFXIV for this Dalamud release.
    pub supported_game_ver: Option<String>,
    /// The .NET runtime version used for running this release.
    pub runtime_version: String,
    /// Whether or not the .NET runtime is required to run the release.
    pub runtime_required: bool,
    /// The "beta key" that would be used to enable the release with launchers.
    pub key: Option<String>,
    /// The git commit hash of the release.
    pub git_sha: Option<String>,
    /// The revision number of the release.
    pub revision: Option<String>,
}

impl FromStr for DalamudVersionInfo {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

impl DalamudVersionInfo {
    /// Get the file at the given path and returns a [`DalamudVersionInfo`] from it.
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When a failure occurs reading the file at the given path.
    /// * When serialization fails.
    pub fn from_path_ref<P: AsRef<Path> + Debug>(path: &P) -> Result<Self> {
        read_to_string(path)
            .with_context(|| format!("failed read file at {path:?}"))?
            .parse::<Self>()
            .with_context(|| format!("unable to deserialize file at {path:?}"))
    }

    /// Get the version info from the [`RemoteResource`] and parse it into [`DalamudVersionInfo`].
    ///
    /// # Errors
    /// This function will return an error in the following situations, but is not limited to just these cases:
    /// * When any network failure occurs.
    /// * When serialization fails.
    pub async fn from_remote_file(file: &RemoteResource) -> Result<Self> {
        file.read_to_string()
            .await?
            .parse::<Self>()
            .with_context(|| format!("unable to deserialize resource at {}", file.url))
    }
}
