//! Bible reading pages - Book-style interface with dual thumb index

use leptos::{prelude::*, reactive::computed::Memo, tachys::dom::window};
use leptos_router::hooks::use_params_map;
use shared::{Book, Testament};
use ui::use_theme;
use wasm_bindgen::{closure::Closure, prelude::*};

fn request_animation_frame(f: impl FnOnce() + 'static) {
    let closure = Closure::once_into_js(f);
    let _ = window().request_animation_frame(closure.unchecked_ref());
}

use crate::{api, components::BottomNav};

#[allow(dead_code)]
mod styles {
    stylance::import_crate_style!(pub reader, "src/styles/reader.module.css");
    stylance::import_crate_style!(pub header, "src/styles/header.module.css");
    stylance::import_crate_style!(pub books, "src/styles/books.module.css");
    stylance::import_crate_style!(pub colors, "src/styles/colors.module.css");
    stylance::import_crate_style!(pub settings, "src/styles/settings.module.css");
    stylance::import_crate_style!(pub chapters, "src/styles/chapters.module.css");
}
use styles::*;

/// Bible reader with book-style navigation
#[component]
pub fn Bible() -> impl IntoView {
    // Default to Genesis 1
    view! {
        <BibleReader initial_book=1 initial_chapter=1/>
    }
}

/// Bible chapter view (routed)
#[component]
pub fn BibleChapter() -> impl IntoView {
    let params = use_params_map();

    let book_id = Memo::new(move |_| {
        params
            .read()
            .get("book")
            .and_then(|s| s.parse::<i16>().ok())
            .unwrap_or(1)
    });

    let chapter = Memo::new(move |_| {
        params
            .read()
            .get("chapter")
            .and_then(|s| s.parse::<i16>().ok())
            .unwrap_or(1)
    });

    view! {
        <BibleReader initial_book=book_id.get_untracked() initial_chapter=chapter.get_untracked()/>
    }
}

/// Main Bible reader component
#[component]
fn BibleReader(initial_book: i16, initial_chapter: i16) -> impl IntoView {
    let app_state = expect_context::<crate::state::AppState>();
    let _theme_state = use_theme();

    let current_book = app_state.current_book;
    let current_chapter = app_state.current_chapter;
    app_state.current_book.set(initial_book);
    app_state.current_chapter.set(initial_chapter);

    let (books_open, set_books_open) = signal(false);
    let (chapters_open, set_chapters_open) = signal(false);
    let (verse_per_line, _set_verse_per_line) = signal(false);
    let (scroll_progress, set_scroll_progress) = signal::<Option<f64>>(None);
    let content_ref: NodeRef<leptos::html::Main> = NodeRef::new();

    let all_books = LocalResource::new(|| async { api::get_books_cached().await.ok() });

    let pericopes = LocalResource::new(move || {
        let b = current_book.get();
        async move { api::get_pericopes(b).await.ok() }
    });

    let chapters_info = LocalResource::new(move || {
        let b = current_book.get();
        async move { api::get_chapters_info(b).await.ok() }
    });

    let verses = LocalResource::new(move || {
        let b = current_book.get();
        let c = current_chapter.get();
        async move { api::get_chapter_cached(b, c).await.ok() }
    });

    // Reset scroll progress when book/chapter changes
    Effect::new(move |prev: Option<(i16, i16)>| {
        let book = current_book.get();
        let chapter = current_chapter.get();
        let current = (book, chapter);

        // Always reset when navigation changes
        if prev.is_none() || prev != Some(current) {
            set_scroll_progress.set(None);
        }

        current
    });

    // Calculate scroll progress after content renders
    Effect::new(move |_| {
        let has_content = verses.get().flatten().is_some();
        if !has_content {
            return;
        }

        // Triple RAF: 1) after effect, 2) after Suspense, 3) after layout
        if let Some(el) = content_ref.get() {
            request_animation_frame(move || {
                request_animation_frame(move || {
                    request_animation_frame(move || {
                        let scroll_height = el.scroll_height() as f64;
                        let client_height = el.client_height() as f64;
                        let max_scroll = scroll_height - client_height;
                        if max_scroll > 1.0 {
                            let scroll_top = el.scroll_top() as f64;
                            set_scroll_progress.set(Some(scroll_top / max_scroll));
                        } else {
                            set_scroll_progress.set(Some(1.0));
                        }
                    });
                });
            });
        }
    });

    let current_book_info = move || {
        all_books
            .get()
            .flatten()
            .and_then(|books| books.into_iter().find(|b| b.id == current_book.get()))
    };

    let navigate_to = move |book_id: i16| {
        current_book.set(book_id);
        current_chapter.set(1);
    };

    view! {
        <div class=reader::reader>
            <header class=header::header>
                <div
                    class=header::progress
                    style:width=move || format!("{}%", scroll_progress.get().unwrap_or(0.0) * 100.0)
                    style:opacity=move || if scroll_progress.get().is_some() { "1" } else { "0" }
                    style:background=move || get_book_category_var(current_book.get())
                ></div>
                <div></div>

                <div class=header::title>
                    <button
                        class=header::bookBtn
                        on:click=move |_| {
                            set_books_open.update(|v| *v = !*v);
                            set_chapters_open.set(false);
                        }
                    >
                        {move || current_book_info().map(|b| b.name_ru.clone()).unwrap_or_default()}
                        <ChevronDownIcon/>
                    </button>
                    <button
                        class=header::chapterBtn
                        on:click=move |_| {
                            set_chapters_open.update(|v| *v = !*v);
                            set_books_open.set(false);
                        }
                    >
                        {move || current_chapter.get()}
                        <ChevronDownIcon/>
                    </button>
                </div>

            </header>

            <div class=reader::content>
                <Suspense fallback=|| ()>
                    {move || all_books.get().flatten().map(|books| {
                        let left_books: Vec<_> = books.iter().take(33).cloned().collect();
                        view! {
                            <ThumbIndex
                                books=left_books
                                side=Side::Left
                                current_book=current_book
                                on_select=navigate_to
                            />
                        }
                    })}
                </Suspense>

                <main
                    node_ref=content_ref
                    class=reader::text
                    on:scroll=move |ev| {
                        let target = ev.target().unwrap();
                        let el = target.unchecked_ref::<web_sys::HtmlElement>();
                        let scroll_top = el.scroll_top() as f64;
                        let scroll_height = el.scroll_height() as f64;
                        let client_height = el.client_height() as f64;
                        let max_scroll = scroll_height - client_height;
                        if max_scroll > 0.0 {
                            set_scroll_progress.set(Some(scroll_top / max_scroll));
                        } else {
                            set_scroll_progress.set(Some(1.0));
                        }
                    }
                >
                <Show
                    when=move || books_open.get()
                    fallback=move || view! {
                        <Show
                            when=move || chapters_open.get()
                            fallback=move || view! {
                                <Suspense fallback=|| view! { <VersesLoading/> }>
                                    {move || verses.get().flatten().map(|verses| view! {
                                        <div>
                                            {verses.into_iter().map(|v| {
                                                let text = v.text.clone();
                                                view! {
                                                    <span class=move || if verse_per_line.get() { reader::verseBlock } else { "" }>
                                                        <sup class=reader::verseNum>{v.verse}</sup>
                                                        {text}
                                                        " "
                                                    </span>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    })}
                                </Suspense>
                                <ChapterNav
                                    current_book=current_book
                                    current_chapter=current_chapter
                                    all_books=all_books
                                />
                                <div class=reader::navSpacer></div>
                            }
                        >
                            {move || current_book_info().map(|book| {
                                let chapters_count = book.chapters_count;
                                let percs = pericopes.get().flatten().unwrap_or_default();
                                let ch_info = chapters_info.get().flatten().unwrap_or_default();
                                view! {
                                    <div class=chapters::panel>
                                        <ChaptersList
                                            chapters_count=chapters_count
                                            pericopes=percs
                                            chapters_info=ch_info
                                            current_chapter=current_chapter
                                            on_select=move |ch| {
                                                current_chapter.set(ch);
                                                set_chapters_open.set(false);
                                            }
                                        />
                                    </div>
                                }
                            })}
                        </Show>
                    }
                >
                    <Suspense fallback=|| ()>
                        {move || all_books.get().flatten().map(|books| {
                            view! {
                                <BooksPanel
                                    books=books
                                    current_book=current_book
                                    on_select=move |book_id| {
                                        current_book.set(book_id);
                                        current_chapter.set(1);
                                        set_books_open.set(false);
                                    }
                                />
                            }
                        })}
                    </Suspense>
                </Show>
                </main>

                <Suspense fallback=|| ()>
                    {move || all_books.get().flatten().map(|books| {
                        let right_books: Vec<_> = books.iter().skip(33).cloned().collect();
                        view! {
                            <ThumbIndex
                                books=right_books
                                side=Side::Right
                                current_book=current_book
                                on_select=navigate_to
                            />
                        }
                    })}
                </Suspense>
            </div>

            <BottomNav/>
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Right
}

/// Thumb index component
#[component]
fn ThumbIndex(
    books: Vec<Book>,
    side: Side,
    current_book: RwSignal<i16>,
    on_select: impl Fn(i16) + Copy + 'static
) -> impl IntoView {
    let side_class = match side {
        Side::Left => format!("{} {}", books::bookNav, books::left),
        Side::Right => format!("{} {}", books::bookNav, books::right)
    };

    view! {
        <nav class=side_class>
            {books.into_iter().map(|book| {
                let book_id = book.id;
                let abbrev = book.abbreviation.clone();
                let full_name = book.name_ru.clone();
                let color_class = get_book_color_class(book.id);
                let active_color = get_book_category_var(book.id);
                let is_active = move || current_book.get() == book_id;

                view! {
                    <button
                        class=move || format!("{} {} {}", books::book, color_class, if is_active() { books::active } else { "" })
                        style:background=move || if is_active() { active_color } else { "" }
                        on:click=move |_| on_select(book_id)
                    >
                        <span class=books::abbrev>{abbrev.clone()}</span>
                        <span class=books::full>{full_name.clone()}</span>
                    </button>
                }
            }).collect::<Vec<_>>()}
        </nav>
    }
}

/// Chapter navigation buttons (prev/next)
#[component]
fn ChapterNav(
    current_book: RwSignal<i16>,
    current_chapter: RwSignal<i16>,
    all_books: LocalResource<Option<Vec<Book>>>
) -> impl IntoView {
    let current_book_info = move || {
        all_books
            .get()
            .flatten()
            .and_then(|books| books.into_iter().find(|b| b.id == current_book.get()))
    };

    let can_go_prev = move || {
        let ch = current_chapter.get();
        let book = current_book.get();
        ch > 1 || book > 1
    };

    let can_go_next = move || {
        let ch = current_chapter.get();
        let book = current_book.get();
        let max_ch = current_book_info().map(|b| b.chapters_count).unwrap_or(1);
        ch < max_ch || book < 66
    };

    let go_prev = move |_| {
        let ch = current_chapter.get();
        if ch > 1 {
            current_chapter.set(ch - 1);
        } else {
            let book = current_book.get();
            if book > 1 {
                current_book.set(book - 1);
                // Set to last chapter of previous book
                if let Some(books) = all_books.get().flatten()
                    && let Some(prev_book) = books.iter().find(|b| b.id == book - 1)
                {
                    current_chapter.set(prev_book.chapters_count);
                }
            }
        }
    };

    let go_next = move |_| {
        let ch = current_chapter.get();
        let max_ch = current_book_info().map(|b| b.chapters_count).unwrap_or(1);
        if ch < max_ch {
            current_chapter.set(ch + 1);
        } else {
            let book = current_book.get();
            if book < 66 {
                current_book.set(book + 1);
                current_chapter.set(1);
            }
        }
    };

    view! {
        <div class=reader::chapterNav>
            <button
                class=reader::navBtn
                disabled=move || !can_go_prev()
                on:click=go_prev
            >
                <ChevronLeftIcon/>
                "Назад"
            </button>
            <button
                class=reader::navBtn
                disabled=move || !can_go_next()
                on:click=go_next
            >
                "Далее"
                <ChevronRightIcon/>
            </button>
        </div>
    }
}

/// Books panel with Old/New Testament tabs
#[component]
fn BooksPanel(
    books: Vec<Book>,
    current_book: RwSignal<i16>,
    on_select: impl Fn(i16) + Copy + Send + Sync + 'static
) -> impl IntoView {
    let (active_tab, set_active_tab) = signal(Testament::Old);

    let old_testament: Vec<_> = books
        .iter()
        .filter(|b| b.testament == Testament::Old)
        .cloned()
        .collect();

    let new_testament: Vec<_> = books
        .iter()
        .filter(|b| b.testament == Testament::New)
        .cloned()
        .collect();

    view! {
        <div class=chapters::panel>
            <div class=chapters::tabs>
                <button
                    class=move || format!("{} {}", chapters::tab, if active_tab.get() == Testament::Old { chapters::tabActive } else { "" })
                    on:click=move |_| set_active_tab.set(Testament::Old)
                >
                    "Ветхий Завет"
                </button>
                <button
                    class=move || format!("{} {}", chapters::tab, if active_tab.get() == Testament::New { chapters::tabActive } else { "" })
                    on:click=move |_| set_active_tab.set(Testament::New)
                >
                    "Новый Завет"
                </button>
            </div>

            <Show
                when=move || active_tab.get() == Testament::Old
                fallback=move || view! {
                    <ul class=chapters::booksList>
                        {new_testament.clone().into_iter().map(|book| {
                            let book_id = book.id;
                            let color_class = get_book_color_class(book.id);
                            view! {
                                <li>
                                    <button
                                        class=move || format!(
                                            "{} {} {}",
                                            chapters::bookItem,
                                            color_class,
                                            if current_book.get() == book_id { chapters::bookItemActive } else { "" }
                                        )
                                        on:click=move |_| on_select(book_id)
                                    >
                                        {book.name_ru}
                                    </button>
                                </li>
                            }
                        }).collect::<Vec<_>>()}
                    </ul>
                }
            >
                <ul class=chapters::booksList>
                    {old_testament.clone().into_iter().map(|book| {
                        let book_id = book.id;
                        let color_class = get_book_color_class(book.id);
                        view! {
                            <li>
                                <button
                                    class=move || format!(
                                        "{} {} {}",
                                        chapters::bookItem,
                                        color_class,
                                        if current_book.get() == book_id { chapters::bookItemActive } else { "" }
                                    )
                                    on:click=move |_| on_select(book_id)
                                >
                                    {book.name_ru}
                                </button>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
            </Show>
        </div>
    }
}

/// Chapters list with expandable pericopes
#[component]
fn ChaptersList(
    chapters_count: i16,
    pericopes: Vec<shared::Pericope>,
    chapters_info: Vec<shared::ChapterInfo>,
    current_chapter: RwSignal<i16>,
    on_select: impl Fn(i16) + Copy + Send + Sync + 'static
) -> impl IntoView {
    let (expanded, set_expanded) = signal::<Option<i16>>(None);

    // Group pericopes by chapter and calculate verse ranges
    let chapters_data: Vec<_> = (1..=chapters_count)
        .map(|ch| {
            let verse_count = chapters_info
                .iter()
                .find(|c| c.chapter == ch)
                .map(|c| c.verse_count)
                .unwrap_or(0);

            let mut chapter_percs: Vec<_> = pericopes
                .iter()
                .filter(|p| p.chapter == ch)
                .cloned()
                .collect();
            chapter_percs.sort_by_key(|p| p.verse);

            // Calculate end verse for each pericope
            let percs_with_range: Vec<_> = chapter_percs
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let end_verse = if i + 1 < chapter_percs.len() {
                        chapter_percs[i + 1].verse - 1
                    } else {
                        verse_count // Last pericope goes to end of chapter
                    };
                    (p.verse, end_verse, p.heading.clone())
                })
                .collect();

            (ch, verse_count, percs_with_range)
        })
        .collect();

    view! {
        <ul class=chapters::list>
            {chapters_data.into_iter().map(|(ch, verse_count, percs)| {
                let has_pericopes = !percs.is_empty();
                let percs_for_show = percs.clone();
                let is_current = move || ch == current_chapter.get();
                let is_expanded = move || expanded.get() == Some(ch);
                let content_label = if verse_count > 0 {
                    format!("{} ст.", verse_count)
                } else {
                    "Содержание".to_string()
                };

                view! {
                    <li class=chapters::chapterItem>
                        <div class=move || format!("{} {}", chapters::chapterRow, if is_current() { chapters::active } else { "" })>
                            <span class=chapters::chapterTitle>"Глава " {ch}</span>
                            <div class=chapters::actions>
                                <button
                                    class=chapters::readBtn
                                    on:click=move |_| on_select(ch)
                                >
                                    "Читать"
                                </button>
                                {if has_pericopes {
                                    view! {
                                        <button
                                            class=chapters::contentBtn
                                            on:click=move |_| {
                                                set_expanded.update(|v| {
                                                    *v = if *v == Some(ch) { None } else { Some(ch) }
                                                });
                                            }
                                        >
                                            {content_label.clone()}
                                            <Show when=is_expanded fallback=|| view! { <ChevronDownIcon/> }>
                                                <ChevronUpIcon/>
                                            </Show>
                                        </button>
                                    }.into_any()
                                } else {
                                    view! {
                                        <span class=chapters::verseCountBadge>
                                            {if verse_count > 0 { format!("{} ст.", verse_count) } else { String::new() }}
                                        </span>
                                    }.into_any()
                                }}
                            </div>
                        </div>
                        <Show when=is_expanded>
                            {
                                let percs = percs_for_show.clone();
                                view! {
                                    <ul class=chapters::pericopes>
                                        {percs.into_iter().map(|(start, end, heading)| {
                                            let range = if end > start {
                                                format!("{}-{}", start, end)
                                            } else {
                                                format!("{}", start)
                                            };
                                            view! {
                                                <li class=chapters::pericope>
                                                    <span class=chapters::verseRange>{range}</span>
                                                    <span class=chapters::pericopeText>{heading}</span>
                                                </li>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </ul>
                                }
                            }
                        </Show>
                    </li>
                }
            }).collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
fn ChevronUpIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="16" height="16">
            <polyline points="18 15 12 9 6 15"/>
        </svg>
    }
}

#[component]
fn VersesLoading() -> impl IntoView {
    view! {
        <div class="verses-loading">
            {(0..15).map(|i| view! {
                <div class="skeleton verse-skeleton" style:width=format!("{}%", 70 + (i * 7) % 30)/>
            }).collect::<Vec<_>>()}
        </div>
    }
}

fn get_book_color_class(book_id: i16) -> &'static str {
    match book_id {
        1..=5 => colors::torah,
        6..=17 => colors::history,
        18..=22 => colors::wisdom,
        23..=27 => colors::majorProphets,
        28..=39 => colors::minorProphets,
        40..=43 => colors::gospels,
        44 => colors::acts,
        45..=57 => colors::paul,
        58..=65 => colors::general,
        66 => colors::revelation,
        _ => ""
    }
}

/// Get book category CSS variable
fn get_book_category_var(book_id: i16) -> &'static str {
    match book_id {
        1..=5 => "var(--cat-torah)",
        6..=17 => "var(--cat-history)",
        18..=22 => "var(--cat-wisdom)",
        23..=27 => "var(--cat-major-prophets)",
        28..=39 => "var(--cat-minor-prophets)",
        40..=43 => "var(--cat-gospels)",
        44 => "var(--cat-acts)",
        45..=57 => "var(--cat-paul)",
        58..=65 => "var(--cat-general)",
        66 => "var(--cat-revelation)",
        _ => "var(--cat-gospels)"
    }
}

// Icons

#[component]
fn ChevronLeftIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="20" height="20">
            <polyline points="15 18 9 12 15 6"/>
        </svg>
    }
}

#[component]
fn ChevronRightIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="20" height="20">
            <polyline points="9 18 15 12 9 6"/>
        </svg>
    }
}

#[component]
fn ChevronDownIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="16" height="16">
            <polyline points="6 9 12 15 18 9"/>
        </svg>
    }
}
