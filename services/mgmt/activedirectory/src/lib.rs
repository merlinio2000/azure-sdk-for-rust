#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2017-04-01")]
pub mod package_2017_04_01;
#[cfg(feature = "package-2020-03")]
pub mod package_2020_03;
#[cfg(feature = "package-preview-2017-04")]
pub mod package_preview_2017_04;
#[cfg(feature = "package-preview-2020-03")]
pub mod package_preview_2020_03;
#[cfg(feature = "package-preview-2020-07")]
pub mod package_preview_2020_07;
#[cfg(all(feature = "default_tag", feature = "package-2020-03"))]
pub use package_2020_03::*;
