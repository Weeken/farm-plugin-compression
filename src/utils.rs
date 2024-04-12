use farmfe_core::resource::Resource;
use flate2::{write::GzEncoder, Compression};
use std::collections::HashMap;
use std::io::Write;

pub fn get_compress(level: &str) -> Compression {
  let compression: Compression = match level {
    "default" => Compression::default(),
    "none" => Compression::none(),
    "fast" => Compression::fast(),
    "best" => Compression::best(),
    _ => Compression::default(),
  };
  return compression;
}

pub fn generate_compress_data(buf: &[u8], compression: Compression) -> Vec<u8> {
  let mut encoder = GzEncoder::new(Vec::new(), compression);
  encoder.write_all(buf).unwrap();
  let compressed_data = encoder.finish().unwrap();
  return compressed_data;
}

pub fn insert_resource(
  resources_map: &mut HashMap<String, Resource>,
  name: String,
  resource: Resource,
) {
  if !resources_map.contains_key(&name) {
    resources_map.insert(name, resource);
  }
}
