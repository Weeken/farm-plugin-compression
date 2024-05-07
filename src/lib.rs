#![deny(clippy::all)]

use farmfe_core::{
  config::{Config, Mode},
  context::CompilationContext,
  error::Result,
  plugin::{Plugin, PluginFinalizeResourcesHookParams},
  resource::{Resource, ResourceType},
  serde_json::{self, Value},
};

use farmfe_macro_plugin::farm_plugin;

use std::sync::Arc;

mod utils;
use crate::utils::{generate_compress_data, get_compress, insert_resource};

#[derive(serde::Deserialize, Debug)]
#[farm_plugin]
pub struct FarmPluginCompression {
  compression_option: String,
}

impl FarmPluginCompression {
  fn new(_: &Config, options: String) -> Self {
    Self {
      compression_option: options,
    }
  }
}

impl Plugin for FarmPluginCompression {
  fn name(&self) -> &str {
    "FarmPluginCompression"
  }

  fn priority(&self) -> i32 {
    90
  }

  fn finalize_resources(
    &self,
    _param: &mut PluginFinalizeResourcesHookParams,
    _context: &Arc<CompilationContext>,
  ) -> Result<Option<()>> {
    if matches!(_context.config.mode, Mode::Production) {
      let options: Value = serde_json::from_str(&self.compression_option).unwrap_or_default();
      println!("options: {}", options);
      let mut compression = get_compress("default");
      let mut exclude_list: Vec<String> = vec![];
      if let Some(level) = options.get("level") {
        let result = level.as_str().unwrap();
        // println!("compression: {}", result);
        compression = get_compress(result);
      }
      if let Some(exclude) = options.get("exclude") {
        let result = exclude.as_array().unwrap();
        for ele in result.iter() {
          exclude_list.push(ele.as_str().unwrap().to_string())
        }
      }
      println!("exclude_list: {:?}", exclude_list);
      let resource_map_clone = _param.resources_map.clone();

      for (name, resource) in resource_map_clone.iter() {
        let target_ext = resource.resource_type.to_ext();
        if target_ext != ResourceType::Custom("gz".to_string()).to_ext()
          && !exclude_list.contains(&target_ext)
        {
          // println!("name: {}", name);
          if !name.starts_with("FARM_RUNTIME_") {
            let file_name = format!("{}.gz", name);

            let gz_resource = Resource {
              name: file_name.clone(),
              bytes: generate_compress_data(&resource.bytes, compression),
              emitted: false,
              resource_type: ResourceType::Custom("gz".to_string()),
              origin: resource.origin.clone(),
              info: None,
            };

            insert_resource(_param.resources_map, file_name.clone(), gz_resource);
          }
        }
      }
    }
    Ok(None)
  }
}
