#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-05")]
pub mod package_2022_05;
#[cfg(all(feature = "package-2022-05", not(feature = "no-default-tag")))]
pub use package_2022_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-01")]
pub mod package_2022_01;
#[cfg(all(feature = "package-2022-01", not(feature = "no-default-tag")))]
pub use package_2022_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-09")]
pub mod package_2021_09;
#[cfg(all(feature = "package-2021-09", not(feature = "no-default-tag")))]
pub use package_2021_09::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-05")]
pub mod package_2021_05;
#[cfg(all(feature = "package-2021-05", not(feature = "no-default-tag")))]
pub use package_2021_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(all(feature = "package-2021-03", not(feature = "no-default-tag")))]
pub use package_2021_03::{models, Client, ClientBuilder};
