#[allow(clippy::wildcard_imports)]
use crate::app::*;

impl App {
    pub(super) fn on_browser_collections(
        &mut self,
        collections: Vec<crate::contracts::browser::BrowserCollection>,
        source: crate::contracts::browser::Source,
    ) {
        let Some(br) = self.source_browser.browser_for_source_mut(source) else {
            return;
        };
        br.request.wallhaven.collections =
            collections.into_iter().map(|collection| (collection.id, collection.label)).collect();
        self.retick();
    }

    pub(super) fn on_browser_download(
        &mut self,
        status: crate::contracts::browser::DownloadResponse,
        source: crate::contracts::browser::Source,
        id: &str,
    ) {
        if let Some(br) = self.source_browser.browser_for_source_mut(source)
            && let Some(it) = br.item_mut(id)
        {
            match status {
                crate::contracts::browser::DownloadResponse::Exists
                | crate::contracts::browser::DownloadResponse::Done => {
                    it.downloaded = true;
                    it.downloading = false;
                }
                crate::contracts::browser::DownloadResponse::Started => it.downloading = true,
                crate::contracts::browser::DownloadResponse::Other => {}
            }
        }
    }

    pub(in crate::app) fn on_browser_search(
        &mut self,
        page: crate::contracts::browser::SearchPage,
        source: crate::contracts::browser::Source,
        append: bool,
        generation: u64,
    ) {
        let active = self.source_browser.browser.as_ref().is_some_and(|br| br.source == source);
        let need_more = {
            let Some(br) = self.source_browser.browser_for_source_mut(source) else {
                return;
            };
            if generation != br.session.search_generation
                || page.generation.is_some_and(|response| response != generation)
            {
                return;
            }
            br.session.loading = false;
            br.session.page_failed = false;
            br.session.last_page = page.last_page;
            br.session.page = page.current_page;
            br.session.next_cursor = page.next_cursor;
            let new_items: Vec<crate::frontend::browser::BrowserItem> =
                page.results.into_iter().map(Into::into).collect();
            if append {
                br.session.items.extend(new_items);
            } else {
                br.session.items = new_items;
                br.view.thumb_fades.clear();
                br.view.hover_fades.clear();
            }
            br.session.page < br.session.last_page && br.session.items.is_empty()
        };
        if active && let Some(br) = self.source_browser.browser.as_ref() {
            self.source_browser.wall.rebuild_catalogue(br, !append);
        }
        if active && need_more {
            self.run_browser_search(true);
        }
    }
}
