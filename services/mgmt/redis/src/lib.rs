#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2022-05")]
pub mod package_2022_05;
#[cfg(feature = "package-2022-06")]
pub mod package_2022_06;
#[cfg(feature = "package-2023-04")]
pub mod package_2023_04;
#[cfg(feature = "package-2023-08")]
pub mod package_2023_08;
#[cfg(feature = "package-preview-2023-05")]
pub mod package_preview_2023_05;
#[cfg(all(feature = "default_tag", feature = "package-2023-08"))]
pub use package_2023_08::*;
