//! `disktree-core`: scanning, layout, space accounting and deletion, with no
//! UI dependency.
//!
//! The split exists so the parts that must be *correct* — size accounting,
//! hardlink de-duplication, aspect-ratio layout, and what may be deleted — can
//! be built and tested without GPUI, a display, or a GPU.

pub mod classify;
pub mod filter;
pub mod insights;
pub mod removal;
pub mod scan;
pub mod size;
pub mod space;
pub mod tree;
pub mod treemap;

/// The user's home directory, canonical so it matches a scanned root.
///
/// `HOME` is unset on Windows. [`std::env::home_dir`] reads
/// `USERPROFILE` there, and the passwd entry when `HOME` is unset on
/// Unix. Scanned roots are canonical too, so the home-directory guard
/// compares the same path, `\\?\` prefix included.
pub fn home_dir() -> Option<std::path::PathBuf> {
    std::env::home_dir().map(|path| path.canonicalize().unwrap_or(path))
}
