#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2021-12-01-preview")]
pub mod package_2021_12_01_preview;
#[cfg(feature = "package-2022-04-15-preview")]
pub mod package_2022_04_15_preview;
#[cfg(feature = "package-2022-12-01")]
pub mod package_2022_12_01;
#[cfg(all(feature = "default_tag", feature = "package-2022-12-01"))]
pub use package_2022_12_01::*;
