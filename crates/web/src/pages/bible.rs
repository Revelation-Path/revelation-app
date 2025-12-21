//! Bible reading pages - Book-style interface with dual thumb index

use leptos::{prelude::*, reactive::computed::Memo};
use leptos_router::hooks::use_params_map;
use shared::Book;

use crate::{api, components::BottomNav};

stylance::import_crate_style!(reader, "src/styles/reader.module.css");
stylance::import_crate_style!(header, "src/styles/header.module.css");
stylance::import_crate_style!(thumb, "src/styles/thumb.module.css");
stylance::import_crate_style!(colors, "src/styles/colors.module.css");
stylance::import_crate_style!(settings, "src/styles/settings.module.css");
stylance::import_crate_style!(chapters, "src/styles/chapters.module.css");

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
    let (current_book, set_current_book) = signal(initial_book);
    let (current_chapter, set_current_chapter) = signal(initial_chapter);
    let (settings_open, set_settings_open) = signal(false);
    let (chapters_open, set_chapters_open) = signal(false);

    let (font_size, set_font_size) = signal(18_u8);
    let (theme, set_theme) = signal(Theme::Light);
    let (font_family, set_font_family) = signal(FontFamily::Serif);

    let all_books = LocalResource::new(|| async { api::get_books().await.ok() });

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
        async move { api::get_chapter(b, c).await.ok() }
    });

    let current_book_info = move || {
        all_books
            .get()
            .flatten()
            .and_then(|books| books.into_iter().find(|b| b.id == current_book.get()))
    };

    let navigate_to = move |book_id: i16| {
        set_current_book.set(book_id);
        set_current_chapter.set(1);
    };

    view! {
        <div
            class=reader::reader
            style:font-size=move || format!("{}px", font_size.get())
            class:theme-dark=move || theme.get() == Theme::Dark
            class:theme-sepia=move || theme.get() == Theme::Sepia
            class:font-serif=move || font_family.get() == FontFamily::Serif
            class:font-sans=move || font_family.get() == FontFamily::Sans
        >
            <header class=header::header>
                <div></div>

                <div class=header::title>
                    <span class=header::book>
                        {move || current_book_info().map(|b| b.name_ru.clone()).unwrap_or_default()}
                    </span>
                    <button
                        class=header::chapterBtn
                        on:click=move |_| set_chapters_open.update(|v| *v = !*v)
                    >
                        "Гл. " {move || current_chapter.get()}
                        <ChevronDownIcon/>
                    </button>
                </div>

                <button class=header::btn on:click=move |_| set_settings_open.update(|v| *v = !*v)>
                    <SettingsIcon/>
                </button>
            </header>

            <Show when=move || settings_open.get()>
                <SettingsPanel
                    font_size=font_size
                    set_font_size=set_font_size
                    theme=theme
                    set_theme=set_theme
                    font_family=font_family
                    set_font_family=set_font_family
                    on_close=move || set_settings_open.set(false)
                />
            </Show>

            // Thumb indexes - fixed position layers
            <Suspense fallback=|| ()>
                {move || all_books.get().flatten().map(|books| {
                    let left_books: Vec<_> = books.iter().take(33).cloned().collect();
                    let right_books: Vec<_> = books.iter().skip(33).cloned().collect();
                    view! {
                        <ThumbIndex
                            books=left_books
                            side=Side::Left
                            current_book=current_book
                            on_select=navigate_to
                        />
                        <ThumbIndex
                            books=right_books
                            side=Side::Right
                            current_book=current_book
                            on_select=navigate_to
                        />
                    }
                })}
            </Suspense>

            <main class=reader::text>
                <Show
                    when=move || chapters_open.get()
                    fallback=move || view! {
                        <Suspense fallback=|| view! { <VersesLoading/> }>
                            {move || verses.get().flatten().map(|verses| view! {
                                <div>
                                    {verses.into_iter().map(|v| view! {
                                        <span>
                                            <sup class=reader::verseNum>{v.verse}</sup>
                                            {v.text}
                                            " "
                                        </span>
                                    }).collect::<Vec<_>>()}
                                </div>
                            })}
                        </Suspense>
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
                                        set_current_chapter.set(ch);
                                        set_chapters_open.set(false);
                                    }
                                />
                            </div>
                        }
                    })}
                </Show>
            </main>

            <BottomNav/>
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Right
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Theme {
    #[default]
    Light,
    Dark,
    Sepia
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum FontFamily {
    #[default]
    Serif,
    Sans
}

/// Thumb index component
#[component]
fn ThumbIndex(
    books: Vec<Book>,
    side: Side,
    current_book: ReadSignal<i16>,
    on_select: impl Fn(i16) + Copy + 'static
) -> impl IntoView {
    let state = expect_context::<crate::state::AppState>();
    let collapsed = state.sidebar_collapsed;

    let side_class = move || match side {
        Side::Left => format!(
            "{} {} {}",
            thumb::index,
            thumb::left,
            if collapsed.get() {
                thumb::leftCollapsed
            } else {
                ""
            }
        ),
        Side::Right => format!("{} {}", thumb::index, thumb::right)
    };

    view! {
        <nav class=side_class>
            {books.into_iter().map(|book| {
                let book_id = book.id;
                let abbrev = book.abbreviation.clone();
                let full_name = book.name_ru.clone();
                let color_class = get_book_color_class(book.id);
                let is_active = move || current_book.get() == book_id;

                view! {
                    <button
                        class=move || format!("{} {} {}", thumb::tab, color_class, if is_active() { thumb::active } else { "" })
                        on:click=move |_| on_select(book_id)
                    >
                        <span class=thumb::abbrev>{abbrev.clone()}</span>
                        <span class=thumb::full>{full_name.clone()}</span>
                    </button>
                }
            }).collect::<Vec<_>>()}
        </nav>
    }
}

/// Settings panel
#[component]
fn SettingsPanel(
    font_size: ReadSignal<u8>,
    set_font_size: WriteSignal<u8>,
    theme: ReadSignal<Theme>,
    set_theme: WriteSignal<Theme>,
    font_family: ReadSignal<FontFamily>,
    set_font_family: WriteSignal<FontFamily>,
    on_close: impl Fn() + Copy + 'static
) -> impl IntoView {
    view! {
        <div class=settings::panel>
            <div class=settings::header>
                <span class=settings::title>"Настройки"</span>
                <button class=header::btn on:click=move |_| on_close()>
                    <CloseIcon/>
                </button>
            </div>

            <div class=settings::row>
                <span class=settings::label>"Размер текста"</span>
                <div class=settings::control>
                    <button on:click=move |_| set_font_size.update(|s| *s = (*s).saturating_sub(2).max(12))>"A-"</button>
                    <span>{move || font_size.get()}</span>
                    <button on:click=move |_| set_font_size.update(|s| *s = (*s + 2).min(32))>"A+"</button>
                </div>
            </div>

            <div class=settings::row>
                <span class=settings::label>"Тема"</span>
                <div class=settings::control>
                    <button on:click=move |_| set_theme.set(Theme::Light)/>
                    <button on:click=move |_| set_theme.set(Theme::Sepia)/>
                    <button on:click=move |_| set_theme.set(Theme::Dark)/>
                </div>
            </div>

            <div class=settings::row>
                <span class=settings::label>"Шрифт"</span>
                <div class=settings::control>
                    <button on:click=move |_| set_font_family.set(FontFamily::Serif)>"Aa"</button>
                    <button on:click=move |_| set_font_family.set(FontFamily::Sans)>"Aa"</button>
                </div>
            </div>
        </div>
    }
}

/// Chapters list with expandable pericopes
#[component]
fn ChaptersList(
    chapters_count: i16,
    pericopes: Vec<shared::Pericope>,
    chapters_info: Vec<shared::ChapterInfo>,
    current_chapter: ReadSignal<i16>,
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

#[component]
fn SettingsIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="20" height="20">
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
    }
}

#[component]
fn CloseIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="20" height="20">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
    }
}
