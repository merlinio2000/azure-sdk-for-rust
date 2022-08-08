#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-07-01-preview")]
pub mod package_2022_07_01_preview;
#[cfg(all(feature = "package-2022-07-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_07_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06-01-preview")]
pub mod package_2021_06_01_preview;
#[cfg(all(feature = "package-2021-06-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_06_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-09-01")]
pub mod package_2020_09_01;
#[cfg(all(feature = "package-2020-09-01", not(feature = "no-default-tag")))]
pub use package_2020_09_01::{models, Client, ClientBuilder};
