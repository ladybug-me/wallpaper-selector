use log::{info, warn};

use crate::infrastructure::semantic::{
    SemanticPaths, SemanticQuery, SemanticResult, SemanticService,
};

#[allow(clippy::wildcard_imports)]
use super::super::*;

impl App {
    fn ensure_semantic_service(&mut self) -> bool {
        if self.runtime_state.semantic.is_none() {
            let paths = match SemanticPaths::discover(
                &self.config.cache_dir(),
                &self.config.str_path(skwd_config::keys::semantic::MANIFEST),
                &self.config.str_path(skwd_config::keys::semantic::INDEX_PROFILE),
            ) {
                Ok(paths) => paths,
                Err(error) => {
                    self.tags.semantic.pending = false;
                    self.tags.semantic.error = Some(error);
                    self.tags.semantic.ranked.clear();
                    self.tags.semantic.exclusions.clear();
                    self.refilter_from_start();
                    return false;
                }
            };
            let wake = self.runtime_state.wake_tx.clone();
            let threads = if self.config.battery_saver_active() {
                1
            } else {
                std::thread::available_parallelism().map_or(1, usize::from).min(4)
            };
            info!(
                "semantic search ready: model={} index={} threads={threads}",
                paths.manifest.display(),
                paths.index.display()
            );
            self.runtime_state.semantic =
                Some(SemanticService::start(paths, threads, move |result| {
                    let _ =
                        wake.unbounded_send(crate::infrastructure::runtime::Wake::Semantic(result));
                }));
        }
        true
    }

    pub(in crate::app) fn prewarm_semantic_search(&mut self) {
        if !self.ensure_semantic_service() {
            return;
        }
        let generation = u64::MAX;
        let top_k = self.library_session.library.catalog().items.len().max(1);
        if let Some(service) = self.runtime_state.semantic.as_ref() {
            let queued = service.query(SemanticQuery {
                generation,
                text: String::from("colourful wallpaper"),
                negative_query: None,
                exclusions: Vec::new(),
                top_k,
                max_results: 1,
                debounce: std::time::Duration::ZERO,
            });
            if !queued {
                warn!("semantic prewarm skipped: search service stopped");
                self.runtime_state.semantic = None;
            }
        }
    }

    pub(in crate::app) fn request_semantic_search(&mut self) {
        if self.tags.search_mode != SearchMode::Describe {
            self.clear_semantic_search();
            self.refilter_from_start();
            return;
        }
        self.tags.semantic.generation = self.tags.semantic.generation.wrapping_add(1);
        let generation = self.tags.semantic.generation;
        let query = self.tags.semantic.search.trim().to_string();
        if query.is_empty() {
            self.clear_semantic_search();
            self.refilter_from_start();
            return;
        }
        self.tags.semantic.sort_override = false;
        self.tags.semantic.resolved = false;
        let semantic_query = crate::domain::library::search::semantic_query(
            &query,
            &self.library_session.library.catalog().tag_vocab,
        );
        if !self.ensure_semantic_service() {
            return;
        }
        self.tags.semantic.pending = true;
        self.tags.semantic.error = None;
        let top_k = self.library_session.library.catalog().items.len().max(1);
        let debounce = if self.runtime_state.demo.is_some() {
            std::time::Duration::ZERO
        } else {
            crate::infrastructure::semantic::QUERY_DEBOUNCE
        };
        let queued = self.runtime_state.semantic.as_ref().is_some_and(|service| {
            service.query(SemanticQuery {
                generation,
                text: semantic_query.positive,
                negative_query: semantic_query.negative,
                exclusions: semantic_query.exclusions,
                top_k,
                max_results: 256,
                debounce,
            })
        });
        if !queued {
            self.tags.semantic.pending = false;
            self.tags.semantic.resolved = false;
            self.tags.semantic.error = Some(String::from("semantic search service stopped"));
            self.tags.semantic.ranked.clear();
            self.tags.semantic.exclusions.clear();
            self.runtime_state.semantic = None;
        }
    }

    pub(in crate::app) fn apply_semantic_result(&mut self, mut result: SemanticResult) {
        if self.tags.search_mode != SearchMode::Describe
            || result.generation != self.tags.semantic.generation
        {
            return;
        }
        self.tags.semantic.pending = false;
        self.tags.semantic.query_ms = result.query_ms;
        self.tags.semantic.search_ms = result.search_ms;
        if let Some(error) = result.error {
            warn!("semantic search failed: {error}");
            self.tags.semantic.error = Some(error);
            self.tags.semantic.resolved = false;
            self.tags.semantic.ranked.clear();
            self.tags.semantic.exclusions.clear();
            self.runtime_state.semantic = None;
        } else {
            self.tags.semantic.error = None;
            if !result.exclusions.is_empty() {
                let tags = &self.library_session.library.catalog().tags;
                result.keys.retain(|key| {
                    tags.get(key).is_none_or(|item_tags| {
                        !result
                            .exclusions
                            .iter()
                            .any(|excluded| item_tags.iter().any(|tag| tag == excluded))
                    })
                });
            }
            self.tags.semantic.exclusions = result.exclusions;
            self.tags.semantic.ranked = result.keys;
            self.tags.semantic.resolved = true;
        }
        self.refilter_semantic_from_start();
        self.retick();
    }

    pub(in crate::app) fn clear_semantic_search(&mut self) {
        self.tags.semantic.generation = self.tags.semantic.generation.wrapping_add(1);
        self.tags.semantic.pending = false;
        self.tags.semantic.resolved = false;
        self.tags.semantic.error = None;
        self.tags.semantic.ranked.clear();
        self.tags.semantic.exclusions.clear();
        self.tags.semantic.sort_override = false;
        self.tags.semantic.query_ms = 0.0;
        self.tags.semantic.search_ms = 0.0;
        self.runtime_state.semantic = None;
    }
}
