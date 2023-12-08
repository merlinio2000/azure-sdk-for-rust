#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(feature = "package-2022-12")]
pub mod package_2022_12;
#[cfg(feature = "package-2023-05")]
pub mod package_2023_05;
#[cfg(feature = "package-preview-2023-10")]
pub mod package_preview_2023_10;
#[cfg(all(feature = "default_tag", feature = "package-2023-05"))]
pub use package_2023_05::*;
