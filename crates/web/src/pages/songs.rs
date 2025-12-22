//! Songs/Songbook pages

use leptos::prelude::*;
use leptos_router::{
    components::A,
    hooks::{use_navigate, use_params_map}
};
use shared::{Song, SongSummary, Songbook};
use songbook::ChordProParser;
use uuid::Uuid;

use crate::api;

/// Main songs page - shows songbooks
#[component]
pub fn Songs() -> impl IntoView {
    let songbooks =
        LocalResource::new(|| async { api::get_songbooks().await.unwrap_or_default() });

    view! {
        <div class="page-container">
            <header class="page-header">
                <h1 class="page-title">"Песни"</h1>
                <A href="/songs/search" attr:class="btn-icon">
                    <SearchIcon/>
                </A>
            </header>

            <div class="px-4">
                <Suspense fallback=|| view! { <LoadingSpinner/> }>
                    {move || Suspend::new(async move {
                        let books = songbooks.await;
                        view! {
                            <div class="space-y-3">
                                <For
                                    each=move || books.clone()
                                    key=|s| s.id
                                    children=|songbook| view! { <SongbookCard songbook=songbook/> }
                                />
                            </div>
                        }
                    })}
                </Suspense>
            </div>
        </div>
    }
}

/// Songbook card component
#[component]
fn SongbookCard(songbook: Songbook) -> impl IntoView {
    let year_info = match (songbook.year_first_published, songbook.edition_name.clone()) {
        (Some(year), Some(edition)) => Some(format!("с {} г. • {}", year, edition)),
        (Some(year), None) => Some(format!("с {} г.", year)),
        (None, Some(edition)) => Some(edition),
        (None, None) => None
    };

    let publisher = songbook.publisher.clone();
    let denomination = songbook.denomination.clone();

    view! {
        <A href=format!("/songs/book/{}", songbook.id) attr:class="block">
            <div class="card hover:bg-[var(--color-surface-secondary)] transition-colors">
                <div class="flex items-center gap-4">
                    <div class="w-14 h-14 rounded-lg flex items-center justify-center shrink-0"
                         style="background: linear-gradient(135deg, var(--color-gold-500), var(--color-gold-600))">
                        <MusicIcon class="w-7 h-7 text-white"/>
                    </div>
                    <div class="flex-1 min-w-0">
                        <h3 class="font-semibold text-[var(--color-text-primary)]">
                            {songbook.name_ru.clone()}
                        </h3>
                        {year_info.map(|info| view! {
                            <p class="text-sm text-[var(--color-text-secondary)]">
                                {info}
                            </p>
                        })}
                        <div class="flex items-center gap-2 mt-1 flex-wrap">
                            <span class="text-xs text-[var(--color-text-muted)]">
                                {format!("{} песен", songbook.songs_count)}
                            </span>
                            {denomination.map(|d| view! {
                                <span class="text-xs px-2 py-0.5 rounded-full bg-[var(--color-surface-secondary)] text-[var(--color-text-muted)]">
                                    {d}
                                </span>
                            })}
                        </div>
                        {publisher.map(|p| view! {
                            <p class="text-xs text-[var(--color-text-muted)] mt-1">
                                {p}
                            </p>
                        })}
                    </div>
                    <ChevronRightIcon class="w-5 h-5 text-[var(--color-text-muted)] shrink-0"/>
                </div>
            </div>
        </A>
    }
}

/// Songbook detail page - shows info and editions
#[component]
pub fn SongbookDetail() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();

    let songbook_id = move || {
        params
            .read()
            .get("id")
            .and_then(|s| Uuid::parse_str(&s).ok())
    };

    let songbook = LocalResource::new(move || async move {
        if let Some(id) = songbook_id() {
            api::get_songbook(id).await.ok()
        } else {
            None
        }
    });

    let editions = LocalResource::new(move || async move {
        if let Some(id) = songbook_id() {
            api::get_songbook_editions(id).await.unwrap_or_default()
        } else {
            vec![]
        }
    });

    view! {
        <div class="page-container">
            <header class="page-header">
                <button
                    class="btn-icon"
                    on:click=move |_| { navigate("/songs", Default::default()); }
                >
                    <BackIcon/>
                </button>
                <h1 class="page-title">"Сборник"</h1>
                <A href="/songs/search" attr:class="btn-icon">
                    <SearchIcon/>
                </A>
            </header>

            <div class="px-4">
                <Suspense fallback=|| view! { <LoadingSpinner/> }>
                    {move || Suspend::new(async move {
                        let sb = songbook.await;
                        let eds = editions.await;

                        match sb {
                            Some(sb) => {
                                let sb_id = sb.id;
                                let has_editions = !eds.is_empty();

                                view! {
                                    <div class="space-y-6">
                                        // Songbook header
                                        <div class="text-center py-4">
                                            <div class="w-20 h-20 mx-auto rounded-xl flex items-center justify-center mb-4"
                                                 style="background: linear-gradient(135deg, var(--color-gold-500), var(--color-gold-600))">
                                                <MusicIcon class="w-10 h-10 text-white"/>
                                            </div>
                                            <h1 class="text-2xl font-bold text-[var(--color-text-primary)]">
                                                {sb.name_ru.clone()}
                                            </h1>
                                            {sb.year_first_published.map(|y| view! {
                                                <p class="text-sm text-[var(--color-text-secondary)] mt-1">
                                                    {format!("с {} года", y)}
                                                </p>
                                            })}
                                            <p class="text-sm text-[var(--color-text-muted)] mt-2">
                                                {format!("{} песен в базе", sb.songs_count)}
                                            </p>
                                        </div>

                                        // History
                                        {sb.history.clone().map(|h| view! {
                                            <div class="card">
                                                <h3 class="font-medium text-[var(--color-text-primary)] mb-2">"История"</h3>
                                                <p class="text-sm text-[var(--color-text-secondary)] leading-relaxed">
                                                    {h}
                                                </p>
                                            </div>
                                        })}

                                        // Editions
                                        {has_editions.then(|| view! {
                                            <div>
                                                <h3 class="font-medium text-[var(--color-text-primary)] mb-3">"Издания"</h3>
                                                <div class="space-y-2">
                                                    <For
                                                        each=move || eds.clone()
                                                        key=|e| e.id
                                                        children=move |edition| {
                                                            let ed_name = edition.edition_name.clone();
                                                            let ed_year = edition.year_published;
                                                            let ed_count = edition.songs_count;
                                                            let ed_publisher = edition.publisher.clone();

                                                            view! {
                                                                <A href=format!("/songs/book/{}/songs", sb_id) attr:class="block">
                                                                    <div class="card hover:bg-[var(--color-surface-secondary)] transition-colors">
                                                                        <div class="flex items-center justify-between">
                                                                            <div>
                                                                                <h4 class="font-medium text-[var(--color-text-primary)]">
                                                                                    {ed_name}
                                                                                </h4>
                                                                                <p class="text-sm text-[var(--color-text-secondary)]">
                                                                                    {format!("{} год • {} песен", ed_year, ed_count)}
                                                                                </p>
                                                                                {ed_publisher.map(|p| view! {
                                                                                    <p class="text-xs text-[var(--color-text-muted)]">{p}</p>
                                                                                })}
                                                                            </div>
                                                                            <ChevronRightIcon class="w-5 h-5 text-[var(--color-text-muted)]"/>
                                                                        </div>
                                                                    </div>
                                                                </A>
                                                            }
                                                        }
                                                    />
                                                </div>
                                            </div>
                                        })}

                                        // Direct link to all songs
                                        <A href=format!("/songs/book/{}/songs", sb_id) attr:class="block">
                                            <div class="btn-primary text-center">
                                                {format!("Все песни ({})", sb.songs_count)}
                                            </div>
                                        </A>
                                    </div>
                                }.into_any()
                            },
                            None => view! {
                                <div class="text-center py-8 text-[var(--color-text-muted)]">
                                    "Сборник не найден"
                                </div>
                            }.into_any()
                        }
                    })}
                </Suspense>
            </div>
        </div>
    }
}

/// Songbook songs list page
#[component]
pub fn SongbookSongs() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();

    let songbook_id = move || {
        params
            .read()
            .get("id")
            .and_then(|s| Uuid::parse_str(&s).ok())
    };

    let songbook = LocalResource::new(move || async move {
        if let Some(id) = songbook_id() {
            api::get_songbook(id).await.ok()
        } else {
            None
        }
    });

    let songs = LocalResource::new(move || async move {
        if let Some(id) = songbook_id() {
            api::get_songs_by_songbook(id, None, Some(1000))
                .await
                .unwrap_or_default()
        } else {
            vec![]
        }
    });

    view! {
        <div class="page-container">
            <header class="page-header">
                <button
                    class="btn-icon"
                    on:click=move |_| {
                        if let Some(id) = songbook_id() {
                            navigate(&format!("/songs/book/{}", id), Default::default());
                        } else {
                            navigate("/songs", Default::default());
                        }
                    }
                >
                    <BackIcon/>
                </button>
                <Suspense fallback=|| view! { <h1 class="page-title">"Песни"</h1> }>
                    {move || Suspend::new(async move {
                        let sb = songbook.await;
                        view! {
                            <h1 class="page-title">{sb.map(|s| s.name_ru).unwrap_or_else(|| "Песни".to_string())}</h1>
                        }
                    })}
                </Suspense>
                <A href="/songs/search" attr:class="btn-icon">
                    <SearchIcon/>
                </A>
            </header>

            <div class="px-4">
                <Suspense fallback=|| view! { <LoadingSpinner/> }>
                    {move || Suspend::new(async move {
                        let song_list = songs.await;
                        view! {
                            <div class="space-y-1">
                                <For
                                    each=move || song_list.clone()
                                    key=|s| s.id
                                    children=|song| view! { <SongListItem song=song/> }
                                />
                            </div>
                        }
                    })}
                </Suspense>
            </div>
        </div>
    }
}

/// Song list item
#[component]
fn SongListItem(song: SongSummary) -> impl IntoView {
    let first_line = if song.first_line.is_empty() {
        None
    } else {
        Some(song.first_line.clone())
    };

    view! {
        <A href=format!("/songs/{}", song.id) attr:class="block">
            <div class="py-3 px-4 hover:bg-[var(--color-surface-secondary)] rounded-lg transition-colors">
                <div class="flex items-center gap-3">
                    <span class="text-sm font-medium text-[var(--color-text-muted)] w-8">
                        {song.number.map(|n| format!("#{}", n)).unwrap_or_default()}
                    </span>
                    <div class="flex-1 min-w-0">
                        <h3 class="font-medium text-[var(--color-text-primary)] truncate">
                            {song.title}
                        </h3>
                        {first_line.map(|line| view! {
                            <p class="text-sm text-[var(--color-text-secondary)] truncate">
                                {line}
                            </p>
                        })}
                    </div>
                    {song.original_key.map(|k| view! {
                        <span class="text-xs font-medium px-2 py-1 rounded bg-[var(--color-surface-secondary)] text-[var(--color-text-secondary)]">
                            {k}
                        </span>
                    })}
                </div>
            </div>
        </A>
    }
}

/// Song detail page with chords
#[component]
pub fn SongDetail() -> impl IntoView {
    let params = use_params_map();
    let navigate = use_navigate();

    let song_id = move || {
        params
            .read()
            .get("id")
            .and_then(|s| Uuid::parse_str(&s).ok())
    };

    let transpose = RwSignal::new(0_i32);

    let song = LocalResource::new(move || async move {
        if let Some(id) = song_id() {
            let semitones = transpose.get();
            if semitones == 0 {
                api::get_song(id).await.ok()
            } else {
                api::transpose_song(id, semitones).await.ok()
            }
        } else {
            None
        }
    });

    let go_back = move |_: web_sys::MouseEvent| {
        navigate("/songs", Default::default());
    };

    let transpose_up = move |_: web_sys::MouseEvent| {
        transpose.update(|t| *t = (*t + 1) % 12);
    };

    let transpose_down = move |_: web_sys::MouseEvent| {
        transpose.update(|t| *t = (*t - 1 + 12) % 12);
    };

    view! {
        <div class="page-container">
            <header class="page-header">
                <button class="btn-icon" on:click=go_back>
                    <BackIcon/>
                </button>
                <div class="flex-1"/>
                <div class="flex items-center gap-2">
                    <button class="btn-icon" on:click=transpose_down>
                        <MinusIcon/>
                    </button>
                    <span class="text-sm font-medium min-w-[3rem] text-center">
                        {move || {
                            let t = transpose.get();
                            if t == 0 { "0".to_string() }
                            else if t > 6 { format!("-{}", 12 - t) }
                            else { format!("+{}", t) }
                        }}
                    </span>
                    <button class="btn-icon" on:click=transpose_up>
                        <PlusIcon/>
                    </button>
                </div>
            </header>

            <div class="px-4 pb-safe">
                <Suspense fallback=|| view! { <LoadingSpinner/> }>
                    {move || Suspend::new(async move {
                        match song.await {
                            Some(s) => view! { <SongContent song=s/> }.into_any(),
                            None => view! {
                                <div class="text-center py-8 text-[var(--color-text-muted)]">
                                    "Песня не найдена"
                                </div>
                            }.into_any()
                        }
                    })}
                </Suspense>
            </div>
        </div>
    }
}

/// Song content with parsed chords
#[component]
fn SongContent(song: Song) -> impl IntoView {
    let parsed = ChordProParser::parse(&song.content);

    // Debug: log parsed data
    for section in &parsed.sections {
        for line in &section.lines {
            let debug = format!(
                "Line: '{}', Chords: {:?}",
                line.text,
                line.chords.iter().map(|c| (c.position, c.chord.to_string())).collect::<Vec<_>>()
            );
            web_sys::console::log_1(&debug.into());
        }
    }

    view! {
        <div class="song-content">
            // Title and metadata
            <div class="mb-6">
                <h1 class="text-2xl font-bold text-[var(--color-text-primary)]">
                    {song.number.map(|n| format!("№{}. ", n)).unwrap_or_default()}
                    {song.title}
                </h1>
                <div class="flex items-center gap-4 mt-2 text-sm text-[var(--color-text-secondary)]">
                    {song.author_lyrics.map(|a| view! {
                        <span>{a}</span>
                    })}
                    {parsed.key.map(|k| view! {
                        <span class="font-medium">"Тональность: " {k}</span>
                    })}
                </div>
            </div>

            // Song sections
            <div class="space-y-6 font-mono text-base leading-relaxed">
                {parsed.sections.into_iter().map(|section| {
                    let section_label: Option<&str> = match section.section_type {
                        shared::SongSectionType::Chorus => Some("Припев"),
                        shared::SongSectionType::Verse => section.label.as_ref().map(|_| "Куплет"),
                        shared::SongSectionType::Bridge => Some("Бридж"),
                        shared::SongSectionType::PreChorus => Some("Предприпев"),
                        _ => None
                    };

                    view! {
                        <div class="song-section">
                            // Section label
                            {section_label.map(|label| {
                                let label_with_number = section.label.as_ref()
                                    .map(|n| format!("{} {}", label, n))
                                    .unwrap_or_else(|| label.to_string());
                                view! {
                                    <div class="text-xs font-medium text-[var(--color-text-muted)] uppercase tracking-wide mb-2">
                                        {label_with_number}
                                    </div>
                                }
                            })}

                            // Lines with chords
                            <div class="space-y-1">
                                {section.lines.into_iter().map(|line| {
                                    if line.chords.is_empty() {
                                        // Just text
                                        view! {
                                            <div class="song-line">
                                                <div class="song-text">{line.text}</div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        // Text with chords above
                                        let chord_line = render_chords_line(&line.text, &line.chords);
                                        let text = line.text.clone();
                                        view! {
                                            <div class="song-line" style="font-family: monospace; white-space: pre;">
                                                <div style="color: var(--accent); font-weight: bold;">{chord_line}</div>
                                                <div>{text}</div>
                                            </div>
                                        }.into_any()
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Render chord line positioned above text
fn render_chords_line(text: &str, chords: &[shared::PositionedChord]) -> String {
    let text_len = text.chars().count();
    let mut chord_line: Vec<char> = vec![' '; text_len.max(1)];

    for pc in chords {
        let chord_str = pc.chord.to_string();
        let start = pc.position;

        for (i, c) in chord_str.chars().enumerate() {
            let pos = start + i;
            if pos < chord_line.len() {
                chord_line[pos] = c;
            } else {
                chord_line.push(c);
            }
        }
    }

    chord_line
        .into_iter()
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Song search page
#[component]
pub fn SongSearch() -> impl IntoView {
    let navigate = use_navigate();
    let query = RwSignal::new(String::new());
    let results = RwSignal::new(Vec::new());
    let is_searching = RwSignal::new(false);

    let do_search = move |_: web_sys::MouseEvent| {
        let q = query.get();
        if q.len() >= 2 {
            is_searching.set(true);
            leptos::task::spawn_local(async move {
                match api::search_songs(&q).await {
                    Ok(r) => results.set(r),
                    Err(_) => results.set(vec![])
                }
                is_searching.set(false);
            });
        }
    };

    view! {
        <div class="page-container">
            <header class="page-header">
                <button
                    class="btn-icon"
                    on:click=move |_| { navigate("/songs", Default::default()); }
                >
                    <BackIcon/>
                </button>
                <div class="flex-1">
                    <input
                        type="search"
                        class="input-field w-full"
                        placeholder="Поиск песен..."
                        prop:value=query
                        on:input=move |ev| query.set(event_target_value(&ev))
                        on:keydown=move |ev| {
                            if ev.key() == "Enter" {
                                let q = query.get();
                                if q.len() >= 2 {
                                    is_searching.set(true);
                                    leptos::task::spawn_local(async move {
                                        match api::search_songs(&q).await {
                                            Ok(r) => results.set(r),
                                            Err(_) => results.set(vec![])
                                        }
                                        is_searching.set(false);
                                    });
                                }
                            }
                        }
                    />
                </div>
                <button class="btn-icon" on:click=do_search>
                    <SearchIcon/>
                </button>
            </header>

            <div class="px-4">
                <Show when=move || is_searching.get()>
                    <LoadingSpinner/>
                </Show>

                <Show when=move || !results.get().is_empty()>
                    <div class="space-y-1">
                        <For
                            each=move || results.get()
                            key=|s| s.song.id
                            children=|result| {
                                view! {
                                    <A href=format!("/songs/{}", result.song.id) attr:class="block">
                                        <div class="py-3 px-4 hover:bg-[var(--color-surface-secondary)] rounded-lg transition-colors">
                                            <h3 class="font-medium text-[var(--color-text-primary)]">
                                                {result.song.title}
                                            </h3>
                                            <p class="text-sm text-[var(--color-text-muted)]">
                                                {result.songbook_name.unwrap_or_default()}
                                            </p>
                                        </div>
                                    </A>
                                }
                            }
                        />
                    </div>
                </Show>

                <Show when=move || !is_searching.get() && results.get().is_empty() && (query.get().len() >= 2)>
                    <div class="text-center py-8 text-[var(--color-text-muted)]">
                        "Ничего не найдено"
                    </div>
                </Show>
            </div>
        </div>
    }
}

// ========== Icons ==========

#[component]
fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="flex justify-center py-8">
            <div class="w-8 h-8 border-2 border-[var(--color-gold-500)] border-t-transparent rounded-full animate-spin"/>
        </div>
    }
}

#[component]
fn MusicIcon(#[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
             width="24" height="24">
            <path d="M9 18V5l12-2v13"/>
            <circle cx="6" cy="18" r="3"/>
            <circle cx="18" cy="16" r="3"/>
        </svg>
    }
}

#[component]
fn SearchIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
             width="24" height="24">
            <circle cx="11" cy="11" r="8"/>
            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
    }
}

#[component]
fn BackIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
             width="24" height="24">
            <path d="M19 12H5"/>
            <path d="M12 19l-7-7 7-7"/>
        </svg>
    }
}

#[component]
fn ChevronRightIcon(#[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
             width="24" height="24">
            <path d="M9 18l6-6-6-6"/>
        </svg>
    }
}

#[component]
fn PlusIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
             width="24" height="24">
            <line x1="12" y1="5" x2="12" y2="19"/>
            <line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
    }
}

#[component]
fn MinusIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
             width="24" height="24">
            <line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
    }
}
