//! Bible search page

use leptos::prelude::*;

use crate::{
    api,
    components::{Header, Loading, VerseCard}
};

#[allow(dead_code)]
mod styles {
    stylance::import_crate_style!(pub common, "src/styles/common.module.css");
}
use styles::common;

/// Search page
#[must_use]
#[component]
pub fn Search() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let search_type = RwSignal::new(SearchType::FullText);

    let results = LocalResource::new(move || {
        let q = query.get();
        let t = search_type.get();
        async move {
            if q.trim().is_empty() {
                return None;
            }
            match t {
                SearchType::FullText => api::search_bible(&q).await.ok(),
                SearchType::Symphony => api::get_symphony(&q).await.ok().map(|r| r.verses)
            }
        }
    });

    view! {
        <div class=common::page>
            <Header title="Поиск" back=true/>
            <div class=common::container>
                <SearchInput query=query/>
                <SearchTabs search_type=search_type/>
                <Suspense fallback=|| view! { <Loading/> }>
                    {move || {
                        let q = query.get();
                        if q.trim().is_empty() {
                            Some(view! { <SearchEmptyState/> }.into_any())
                        } else {
                            results.get().flatten().map(|r| view! { <SearchResults results=r/> }.into_any())
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}

#[must_use]
#[component]
fn SearchInput(query: RwSignal<String>) -> impl IntoView {
    view! {
        <div class=common::searchWrapper>
            <div class=common::searchIcon><SearchIcon/></div>
            <input
                type="search"
                placeholder="Введите слово или фразу..."
                class=common::searchInput
                prop:value=query
                on:input=move |ev| query.set(event_target_value(&ev))
            />
        </div>
    }
}

#[must_use]
#[component]
fn SearchTabs(search_type: RwSignal<SearchType>) -> impl IntoView {
    view! {
        <div class=common::tabs>
            <button
                class=move || if search_type.get() == SearchType::FullText {
                    format!("{} {}", common::tab, common::tabActive)
                } else { common::tab.to_string() }
                on:click=move |_| search_type.set(SearchType::FullText)
            >"Поиск"</button>
            <button
                class=move || if search_type.get() == SearchType::Symphony {
                    format!("{} {}", common::tab, common::tabActive)
                } else { common::tab.to_string() }
                on:click=move |_| search_type.set(SearchType::Symphony)
            >"Симфония"</button>
        </div>
    }
}

#[must_use]
#[component]
fn SearchEmptyState() -> impl IntoView {
    view! {
        <div class=common::emptyState>
            <div class=common::emptyIcon><SearchBigIcon/></div>
            <h2 class=common::emptyTitle>"Поиск по Библии"</h2>
            <p class=common::emptyDesc>"Введите слово или фразу для поиска"</p>
        </div>
    }
}

#[must_use]
#[component]
fn SearchResults(results: Vec<revelation_bible::SearchResult>) -> impl IntoView {
    if results.is_empty() {
        view! {
            <div class=common::emptyState>
                <div class=common::emptyIcon><NoResultsIcon/></div>
                <h2 class=common::emptyTitle>"Ничего не найдено"</h2>
                <p class=common::emptyDesc>"Попробуйте изменить запрос"</p>
            </div>
        }.into_any()
    } else {
        view! {
            <div>
                <p class=common::resultCount>"Найдено: " {results.len()} " стихов"</p>
                <div style="display: flex; flex-direction: column; gap: var(--space-xs);">
                    {results.into_iter().map(|r| view! {
                        <VerseCard verse=r.verse book_name=r.book_name/>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        }.into_any()
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SearchType {
    FullText,
    Symphony
}

#[must_use]
#[component]
fn SearchIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="20" height="20">
            <circle cx="11" cy="11" r="8"/>
            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
    }
}

#[must_use]
#[component]
fn SearchBigIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="1.5" stroke-linecap="round"
             stroke-linejoin="round" width="48" height="48">
            <circle cx="11" cy="11" r="8"/>
            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
    }
}

#[must_use]
#[component]
fn NoResultsIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="1.5" stroke-linecap="round"
             stroke-linejoin="round" width="48" height="48">
            <circle cx="11" cy="11" r="8"/>
            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
            <line x1="8" y1="8" x2="14" y2="14"/>
            <line x1="14" y1="8" x2="8" y2="14"/>
        </svg>
    }
}
