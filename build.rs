use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
  // We only need to generate and compile the icon on Windows
  #[cfg(target_os = "windows")]
  {
    use resvg::usvg;

    let svg_data =
      std::fs::read("assets/sync.svg").expect("Failed to read sync.svg");
    let opt = usvg::Options::default();

    let rtree =
      usvg::Tree::from_data(&svg_data, &opt).expect("Failed to parse SVG");

    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    let sizes = [16, 32, 48, 64, 128, 256];

    for &size in &sizes {
      let mut pixmap = resvg::tiny_skia::Pixmap::new(size, size).unwrap();

      // Calculate scale factor to match target size
      let scale_x = size as f32 / rtree.size().width();
      let scale_y = size as f32 / rtree.size().height();
      let transform = resvg::tiny_skia::Transform::from_scale(scale_x, scale_y);

      resvg::render(&rtree, transform, &mut pixmap.as_mut());
      let png_bytes = pixmap.encode_png().expect("Failed to encode PNG");

      let icon_image = ico::IconImage::read_png(&png_bytes[..])
        .expect("Failed to read PNG into ICO");
      icon_dir.add_entry(
        ico::IconDirEntry::encode(&icon_image)
          .expect("Failed to encode ICO entry"),
      );
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let ico_path = Path::new(&out_dir).join("icon.ico");
    let mut out_file =
      File::create(&ico_path).expect("Failed to create icon.ico file");
    icon_dir
      .write(&mut out_file)
      .expect("Failed to write ICO file");

    // Now compile and embed the resource using winres
    let mut res = winres::WindowsResource::new();
    res.set_icon(ico_path.to_str().unwrap());
    res.compile().expect("Failed to compile Windows Resource");
  }
}
