#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2022-03-01-preview")]
pub mod package_2022_03_01_preview;
#[cfg(feature = "package-2022-04-01-preview")]
pub mod package_2022_04_01_preview;
#[cfg(feature = "package-2022-11-01")]
pub mod package_2022_11_01;
#[cfg(feature = "package-2023-06")]
pub mod package_2023_06;
#[cfg(feature = "package-2023-09")]
pub mod package_2023_09;
#[cfg(all(feature = "default_tag", feature = "package-2023-09"))]
pub use package_2023_09::*;
