use std::collections::HashMap;

use derivative::Derivative;
use napi::JsFunction;
use serde::Deserialize;

use crate::{options::sourcemap::SourceMap, types::binding_rendered_module::BindingRenderedModule};

#[napi_derive::napi(object)]
#[derive(Deserialize, Default, Derivative)]
#[serde(rename_all = "camelCase")]
#[derivative(Debug)]
pub struct PluginOptions {
  pub name: String,

  #[derivative(Debug = "ignore")]
  #[serde(skip_deserializing)]
  #[napi(ts_type = "() => Promise<void>")]
  pub build_start: Option<JsFunction>,

  #[derivative(Debug = "ignore")]
  #[serde(skip_deserializing)]
  #[napi(
    ts_type = "(specifier: string, importer?: string, options?: HookResolveIdArgsOptions) => Promise<undefined | ResolveIdResult>"
  )]
  pub resolve_id: Option<JsFunction>,

  #[derivative(Debug = "ignore")]
  #[serde(skip_deserializing)]
  #[napi(ts_type = "(id: string) => Promise<undefined | SourceResult>")]
  pub load: Option<JsFunction>,

  #[derivative(Debug = "ignore")]
  #[serde(skip_deserializing)]
  #[napi(ts_type = "(id: string, code: string) => Promise<undefined | SourceResult>")]
  pub transform: Option<JsFunction>,

  #[derivative(Debug = "ignore")]
  #[serde(skip_deserializing)]
  #[napi(ts_type = "(error: string) => Promise<void>")]
  pub build_end: Option<JsFunction>,

  #[derivative(Debug = "ignore")]
  #[serde(skip_deserializing)]
  #[napi(
    ts_type = "(code: string, chunk: RenderedChunk) => Promise<undefined | HookRenderChunkOutput>"
  )]
  pub render_chunk: Option<JsFunction>,

  #[derivative(Debug = "ignore")]
  #[serde(skip_deserializing)]
  #[napi(ts_type = "(bundle: Outputs, isWrite: boolean) => Promise<void>")]
  pub generate_bundle: Option<JsFunction>,

  #[derivative(Debug = "ignore")]
  #[serde(skip_deserializing)]
  #[napi(ts_type = "(bundle: Outputs) => Promise<void>")]
  pub write_bundle: Option<JsFunction>,
}

#[napi_derive::napi(object)]
#[derive(Deserialize, Default, Derivative)]
#[serde(rename_all = "camelCase")]
#[derivative(Debug)]
pub struct HookResolveIdArgsOptions {
  pub is_entry: bool,
  pub kind: String,
}

impl From<rolldown_plugin::HookResolveIdArgsOptions> for HookResolveIdArgsOptions {
  fn from(value: rolldown_plugin::HookResolveIdArgsOptions) -> Self {
    Self { is_entry: value.is_entry, kind: value.kind.to_string() }
  }
}

#[napi_derive::napi(object)]
#[derive(Deserialize, Default, Derivative)]
#[serde(rename_all = "camelCase")]
#[derivative(Debug)]
pub struct ResolveIdResult {
  pub id: String,
  pub external: Option<bool>,
}

impl From<ResolveIdResult> for rolldown_plugin::HookResolveIdOutput {
  fn from(value: ResolveIdResult) -> Self {
    Self { id: value.id, external: value.external }
  }
}

#[napi_derive::napi(object)]
#[derive(Deserialize, Default, Derivative)]
#[serde(rename_all = "camelCase")]
#[derivative(Debug)]
pub struct SourceResult {
  pub code: String,
  pub map: Option<SourceMap>,
}

impl From<SourceResult> for rolldown_plugin::HookLoadOutput {
  fn from(value: SourceResult) -> Self {
    Self { code: value.code, map: value.map.map(Into::into) }
  }
}

#[napi_derive::napi(object)]
#[derive(Deserialize, Default, Derivative)]
#[serde(rename_all = "camelCase")]
#[derivative(Debug)]
pub struct HookRenderChunkOutput {
  pub code: String,
}

impl From<HookRenderChunkOutput> for rolldown_plugin::HookRenderChunkOutput {
  fn from(value: HookRenderChunkOutput) -> Self {
    Self { code: value.code }
  }
}

#[napi_derive::napi(object)]
#[derive(Deserialize, Default, Derivative)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct PreRenderedChunk {
  // pub name: String,
  pub is_entry: bool,
  pub is_dynamic_entry: bool,
  pub facade_module_id: Option<String>,
  pub module_ids: Vec<String>,
  pub exports: Vec<String>,
}

impl From<rolldown::PreRenderedChunk> for PreRenderedChunk {
  fn from(value: rolldown::PreRenderedChunk) -> Self {
    Self {
      is_entry: value.is_entry,
      is_dynamic_entry: value.is_dynamic_entry,
      facade_module_id: value.facade_module_id,
      module_ids: value.module_ids,
      exports: value.exports,
    }
  }
}

#[napi_derive::napi(object)]
#[derive(Deserialize, Default, Derivative)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct RenderedChunk {
  // PreRenderedChunk
  pub is_entry: bool,
  pub is_dynamic_entry: bool,
  pub facade_module_id: Option<String>,
  pub module_ids: Vec<String>,
  pub exports: Vec<String>,
  // RenderedChunk
  pub file_name: String,
  #[serde(skip)]
  pub modules: HashMap<String, BindingRenderedModule>,
}

impl From<rolldown_common::RenderedChunk> for RenderedChunk {
  fn from(value: rolldown_common::RenderedChunk) -> Self {
    Self {
      is_entry: value.is_entry,
      is_dynamic_entry: value.is_dynamic_entry,
      facade_module_id: value.facade_module_id,
      module_ids: value.module_ids,
      exports: value.exports,
      file_name: value.file_name,
      modules: value.modules.into_iter().map(|(key, value)| (key, value.into())).collect(),
    }
  }
}
