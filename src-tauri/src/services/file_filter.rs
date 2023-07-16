pub struct FileFilter {
    name: &'static str,
    extensions: &'static [&'static str],
}

impl FileFilter {
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn extensions(&self) -> &'static [&'static str] {
        self.extensions
    }
}

const USUAL_PIC_FILTER: FileFilter = FileFilter {
    name: "常用图片",
    extensions: &["png", "jpg", "jpeg", "jfif", "pjpeg", "pjp"],
};
const JPEG_FILTER: FileFilter = FileFilter {
    name: "JPEG",
    extensions: &["jpg", "jpeg", "jfif", "pjpeg", "pjp"],
};
const PNG_FILTER: FileFilter = FileFilter {
    name: "PNG",
    extensions: &["png"],
};
const WEBP_FILTER: FileFilter = FileFilter {
    name: "WEBP",
    extensions: &["webp"],
};
const SVG_FILTER: FileFilter = FileFilter {
    name: "SVG",
    extensions: &["svg"],
};
const GIF_FILTER: FileFilter = FileFilter {
    name: "GIF",
    extensions: &["gif"],
};
const ICO_FILTER: FileFilter = FileFilter {
    name: "ICO",
    extensions: &["ico", "cur"],
};
const BMP_FILTER: FileFilter = FileFilter {
    name: "BMP",
    extensions: &["bmp"],
};
const AVIF_FILTER: FileFilter = FileFilter {
    name: "AVIF",
    extensions: &["avif"],
};
const APNG_FILTER: FileFilter = FileFilter {
    name: "APNG",
    extensions: &["apng"],
};
const ALL_FILTER: FileFilter = FileFilter {
    name: "所有",
    extensions: &["*"],
};

pub const FILTER_ARR: [&FileFilter; 11] = [
    &USUAL_PIC_FILTER,
    &JPEG_FILTER,
    &PNG_FILTER,
    &WEBP_FILTER,
    &SVG_FILTER,
    &GIF_FILTER,
    &ICO_FILTER,
    &BMP_FILTER,
    &AVIF_FILTER,
    &APNG_FILTER,
    &ALL_FILTER,
];
