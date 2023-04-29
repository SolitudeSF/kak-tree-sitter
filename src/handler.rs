use crate::{
  config::Config,
  highlighting::Highlighters,
  languages,
  queries::Queries,
  request::{BufferId, Request, RequestPayload},
  response::Response,
  session::KakSession,
};
use std::{collections::HashMap, path::PathBuf};

/// Type responsible in handling requests.
///
/// This type is stateful, as requests might have side-effect (i.e. tree-sitter parsing generates trees/highlighters
/// that can be reused, for instance).
pub struct Handler {
  /// Per-language queries.
  queries: HashMap<String, Queries>,

  /// Map a highlighter to a [`BufferId`].
  highlighters: Highlighters,
}

impl Handler {
  pub fn new(config: &Config) -> Self {
    Self {
      queries: HashMap::new(),
      highlighters: Highlighters::new(config.highlight.hl_names.clone()),
    }
  }

  /// Handle the request and return whether the handler should shutdown.
  pub fn handle_request(&mut self, request: String) -> Option<(KakSession, Response)> {
    // parse the request and dispatch
    match serde_json::from_str::<Request>(&request) {
      Ok(req) => match req.payload {
        RequestPayload::Shutdown => {
          return Some((req.session, Response::status("kak-tree-sitter: quit", true)));
        }

        RequestPayload::Highlight {
          buffer_id,
          lang,
          path,
        } => {
          let resp = self.handle_highlight_req(buffer_id, lang, path);
          return Some((req.session, resp));
        }
      },

      Err(err) => {
        eprintln!("cannot parse request {request}: {err}");
      }
    }

    None
  }

  fn handle_highlight_req(
    &mut self,
    buffer_id: BufferId,
    lang_str: String,
    path: PathBuf,
  ) -> Response {
    if let Some(lang) = languages::get_lang(&lang_str) {
      if let Some(queries) = self.queries.get(&lang_str) {
        self.highlighters.highlight(lang, queries, buffer_id, path)
      } else {
        Response::status(format!("no highlight query for language {lang_str}"), false)
      }
    } else {
      Response::status(format!("unsupported language: {lang_str}"), false)
    }
  }
}
