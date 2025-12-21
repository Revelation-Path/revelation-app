//! Bible search page

use leptos::prelude::*;

use crate::{
    api,
    components::{Header, Loading, VerseCard}
};

/// Search page
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
        <div class="min-h-screen pb-20">
            <Header title="Поиск" back=true/>

            <main class="p-4 max-w-lg mx-auto">
                // Search input with icon
                <div class="relative mb-4">
                    <div class="absolute left-4 top-1/2 -translate-y-1/2">
                        <SearchIcon/>
                    </div>
                    <input
                        type="search"
                        placeholder="Введите слово или фразу..."
                        class="search-input"
                        prop:value=query
                        on:input=move |ev| query.set(event_target_value(&ev))
                    />
                </div>

                // Search type tabs
                <div class="flex gap-2 mb-6">
                    <SearchTypeTab
                        label="Поиск"
                        value=SearchType::FullText
                        selected=search_type
                    />
                    <SearchTypeTab
                        label="Симфония"
                        value=SearchType::Symphony
                        selected=search_type
                    />
                </div>

                // Results
                <Suspense fallback=|| view! { <Loading/> }>
                    {move || {
                        let q = query.get();
                        if q.trim().is_empty() {
                            Some(view! {
                                <div class="empty-state">
                                    <div class="empty-state-icon">
                                        <SearchBigIcon/>
                                    </div>
                                    <h2 class="empty-state-title">"Поиск по Библии"</h2>
                                    <p class="empty-state-desc">
                                        "Введите слово или фразу для поиска"
                                    </p>
                                </div>
                            }.into_any())
                        } else {
                            results.get().flatten().map(|results| {
                                if results.is_empty() {
                                    view! {
                                        <div class="empty-state">
                                            <div class="empty-state-icon">
                                                <NoResultsIcon/>
                                            </div>
                                            <h2 class="empty-state-title">"Ничего не найдено"</h2>
                                            <p class="empty-state-desc">
                                                "Попробуйте изменить запрос"
                                            </p>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-2">
                                            <p class="text-sm mb-4" style="color: var(--color-text-muted)">
                                                "Найдено: " {results.len()} " стихов"
                                            </p>
                                            {results.into_iter().map(|r| view! {
                                                <VerseCard verse=r.verse book_name=r.book_name/>
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any()
                                }
                            })
                        }
                    }}
                </Suspense>
            </main>
        </div>
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SearchType {
    FullText,
    Symphony
}

#[component]
fn SearchTypeTab(
    label: &'static str,
    value: SearchType,
    selected: RwSignal<SearchType>
) -> impl IntoView {
    let is_selected = move || selected.get() == value;

    view! {
        <button
            class=move || if is_selected() {
                "btn-primary px-4 py-2"
            } else {
                "btn-ghost px-4 py-2"
            }
            on:click=move |_| selected.set(value)
        >
            {label}
        </button>
    }
}

#[component]
fn SearchIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="20" height="20" style="color: var(--color-text-light)">
            <circle cx="11" cy="11" r="8"/>
            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
    }
}

#[component]
fn SearchBigIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="1.5" stroke-linecap="round"
             stroke-linejoin="round" width="32" height="32">
            <circle cx="11" cy="11" r="8"/>
            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
    }
}

#[component]
fn NoResultsIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="1.5" stroke-linecap="round"
             stroke-linejoin="round" width="32" height="32">
            <circle cx="11" cy="11" r="8"/>
            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
            <line x1="8" y1="8" x2="14" y2="14"/>
            <line x1="14" y1="8" x2="8" y2="14"/>
        </svg>
    }
}
