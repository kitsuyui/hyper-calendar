//! Human-readable time, behind the `humanize` feature: how one instant
//! reads from another, *3 hours ago*; which calendar day a day is, seen
//! from another, *yesterday at 15:05*; and how long a span is, *2 hours
//! and 30 minutes*; each in a locale and a style, from `hc-humanize`. The
//! lines are `hyper_calendar::humanize_lines`', shared with the
//! C library.

hc::exports!("humanize", w_exports);
