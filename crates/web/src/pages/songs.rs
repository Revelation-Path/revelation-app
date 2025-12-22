//! Songs/Songbook pages

use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};
use shared::{Song, SongSummary, Songbook};
use songbook::ChordProParser;
use ui::{Icon, IconSize, IconType, Toggle};
use uuid::Uuid;

use crate::{
    api,
    components::{BottomNav, Header},
    state::AppState
};

stylance::import_crate_style!(styles, "src/styles/songs.module.css");

/// Main songs page - shows songbooks
#[component]
pub fn Songs() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let only_with_chords = app_state.only_with_chords;

    let songbooks =
        LocalResource::new(|| async { api::get_songbooks().await.unwrap_or_default() });

    let title = Signal::derive(move || {
        if only_with_chords.get() {
            "Песни с аккордами"
        } else {
            "Песни"
        }
    });

    view! {
        <div class=styles::container>
            <Header title=title>
                <A href="/songs/search" attr:class=styles::iconBtn>
                    <Icon icon=IconType::Search size=IconSize::Medium/>
                </A>
            </Header>

            <div class=styles::filterBar>
                <Toggle
                    active=only_with_chords
                    label_off="С аккордами и без"
                    label_on="Только с аккордами"
                    icon=IconType::Music
                    on_toggle=Callback::new(move |v| {
                        only_with_chords.set(v);
                    })
                />
            </div>

            <div class=styles::content>
                <Suspense fallback=|| view! { <LoadingSpinner/> }>
                    {move || Suspend::new(async move {
                        let books = songbooks.await;
                        let filter_active = only_with_chords.get();
                        let filtered_books: Vec<_> = if filter_active {
                            books.into_iter().filter(|b| b.songs_with_chords_count > 0).collect()
                        } else {
                            books
                        };
                        view! {
                            <div class=styles::grid>
                                <For
                                    each=move || filtered_books.clone()
                                    key=|s| s.id
                                    children=move |songbook| view! {
                                        <SongbookCard songbook=songbook filter_chords=filter_active/>
                                    }
                                />
                            </div>
                        }
                    })}
                </Suspense>
            </div>

            <BottomNav/>
        </div>
    }
}

/// Songbook card component
#[component]
fn SongbookCard(
    songbook: Songbook,
    #[prop(default = false)] filter_chords: bool
) -> impl IntoView {
    let year_info = match (songbook.year_first_published, songbook.edition_name.clone()) {
        (Some(year), Some(edition)) => Some(format!("с {} г. • {}", year, edition)),
        (Some(year), None) => Some(format!("с {} г.", year)),
        (None, Some(edition)) => Some(edition),
        (None, None) => None
    };

    let publisher = songbook.publisher.clone();
    let count = if filter_chords {
        songbook.songs_with_chords_count
    } else {
        songbook.songs_count
    };

    view! {
        <A href=format!("/songs/book/{}", songbook.id) attr:class=styles::songbookCard>
            <div class=styles::songbookIcon>
                <Icon icon=IconType::Music size=IconSize::Large/>
            </div>
            <div class=styles::songbookInfo>
                <h3 class=styles::songbookName>
                    {songbook.name_ru.clone()}
                </h3>
                {year_info.map(|info| view! {
                    <p class=styles::songbookDesc>{info}</p>
                })}
                <span class=styles::songbookCount>
                    {format!("{} песен", count)}
                </span>
                {publisher.map(|p| view! {
                    <p class=styles::songbookCount>{p}</p>
                })}
            </div>
            <span class=styles::chevron>
                <Icon icon=IconType::ChevronRight size=IconSize::Medium/>
            </span>
        </A>
    }
}

/// Songbook detail page - shows info and editions
#[component]
pub fn SongbookDetail() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let only_with_chords = app_state.only_with_chords;
    let params = use_params_map();

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

    let songbook_name = LocalResource::new(move || async move {
        if let Some(id) = songbook_id() {
            api::get_songbook(id).await.ok().map(|s| s.name_ru)
        } else {
            None
        }
    });

    let title = Signal::derive(move || {
        let name = songbook_name
            .get()
            .flatten()
            .unwrap_or_else(|| "Сборник".to_string());
        if only_with_chords.get() {
            format!("{} (с аккордами)", name)
        } else {
            name
        }
    });

    view! {
        <div class=styles::container>
            <Header title=title back=true>
                <A href="/songs/search" attr:class=styles::iconBtn>
                    <Icon icon=IconType::Search size=IconSize::Medium/>
                </A>
            </Header>

            <div class=styles::content>
                <Suspense fallback=|| view! { <LoadingSpinner/> }>
                    {move || Suspend::new(async move {
                        let sb = songbook.await;
                        let eds = editions.await;

                        match sb {
                            Some(sb) => {
                                let sb_id = sb.id;
                                let has_editions = !eds.is_empty();
                                let filter = only_with_chords.get();
                                let count = if filter { sb.songs_with_chords_count } else { sb.songs_count };
                                let label = if filter { "Песни с аккордами" } else { "Все песни" };

                                view! {
                                    <div class=styles::grid>
                                        <SongbookHeader
                                            name=sb.name_ru.clone()
                                            year=sb.year_first_published
                                            songs_count=count
                                        />

                                        {sb.history.clone().map(|h| view! {
                                            <SongbookHistory history=h/>
                                        })}

                                        {has_editions.then(|| view! {
                                            <EditionsList editions=eds.clone() songbook_id=sb_id/>
                                        })}

                                        <A href=format!("/songs/book/{}/songs", sb_id) attr:class=styles::songbookCard>
                                            <div class=styles::songbookIcon>
                                                <Icon icon=IconType::Music size=IconSize::Large/>
                                            </div>
                                            <div class=styles::songbookInfo>
                                                <span class=styles::songbookName>
                                                    {format!("{} ({})", label, count)}
                                                </span>
                                            </div>
                                            <span class=styles::chevron>
                                                <Icon icon=IconType::ChevronRight size=IconSize::Medium/>
                                            </span>
                                        </A>
                                    </div>
                                }.into_any()
                            },
                            None => view! {
                                <div class=styles::empty>
                                    <span class=styles::emptyText>"Сборник не найден"</span>
                                </div>
                            }.into_any()
                        }
                    })}
                </Suspense>
            </div>

            <BottomNav/>
        </div>
    }
}

/// Songbook header with icon
#[component]
fn SongbookHeader(name: String, year: Option<i16>, songs_count: i32) -> impl IntoView {
    view! {
        <div class=styles::songHeader>
            <div class=styles::songbookIcon style="width: 5rem; height: 5rem; margin: 0 auto var(--space-md);">
                <Icon icon=IconType::Music size=IconSize::Large/>
            </div>
            <h1 class=styles::songDetailTitle style="text-align: center;">{name}</h1>
            {year.map(|y| view! {
                <p class=styles::songbookDesc style="text-align: center;">
                    {format!("с {} года", y)}
                </p>
            })}
            <p class=styles::songbookCount style="text-align: center;">
                {format!("{} песен в базе", songs_count)}
            </p>
        </div>
    }
}

/// Songbook history section with expand/collapse
#[component]
fn SongbookHistory(history: String) -> impl IntoView {
    let expanded = RwSignal::new(false);
    let history_clone = history.clone();

    view! {
        <div class=styles::songSection>
            <div class=styles::sectionLabel>"История"</div>
            <p class={move || if expanded.get() { styles::historyExpanded } else { styles::historyCollapsed }}>
                {history_clone.clone()}
            </p>
            <button
                class=styles::expandBtn
                on:click=move |_| expanded.update(|v| *v = !*v)
            >
                {move || if expanded.get() { "Свернуть" } else { "Читать полностью" }}
            </button>
        </div>
    }
}

/// Editions list
#[component]
fn EditionsList(editions: Vec<shared::SongbookEdition>, songbook_id: Uuid) -> impl IntoView {
    view! {
        <div class=styles::songSection>
            <div class=styles::sectionLabel>"Издания"</div>
            <div class=styles::songList>
                <For
                    each=move || editions.clone()
                    key=|e| e.id
                    children=move |edition| {
                        view! {
                            <A href=format!("/songs/book/{}/songs", songbook_id) attr:class=styles::songItem>
                                <div class=styles::songInfo>
                                    <span class=styles::songTitle>{edition.edition_name.clone()}</span>
                                    <span class=styles::songFirstLine>
                                        {format!("{} год • {} песен", edition.year_published, edition.songs_count)}
                                    </span>
                                </div>
                                <span class=styles::chevron>
                                    <Icon icon=IconType::ChevronRight size=IconSize::Small/>
                                </span>
                            </A>
                        }
                    }
                />
            </div>
        </div>
    }
}

/// Songbook songs list page
#[component]
pub fn SongbookSongs() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let only_with_chords = app_state.only_with_chords;
    let params = use_params_map();

    let songbook_id = move || {
        params
            .read()
            .get("id")
            .and_then(|s| Uuid::parse_str(&s).ok())
    };

    let songs = LocalResource::new(move || async move {
        if let Some(id) = songbook_id() {
            api::get_songs_by_songbook(id, None, Some(1000))
                .await
                .unwrap_or_default()
        } else {
            vec![]
        }
    });

    let songbook_name = LocalResource::new(move || async move {
        if let Some(id) = songbook_id() {
            api::get_songbook(id).await.ok().map(|s| s.name_ru)
        } else {
            None
        }
    });

    let title = Signal::derive(move || {
        let name = songbook_name
            .get()
            .flatten()
            .unwrap_or_else(|| "Песни".to_string());
        if only_with_chords.get() {
            format!("{} (с аккордами)", name)
        } else {
            name
        }
    });

    view! {
        <div class=styles::container>
            <Header title=title back=true>
                <A href="/songs/search" attr:class=styles::iconBtn>
                    <Icon icon=IconType::Search size=IconSize::Medium/>
                </A>
            </Header>

            <div class=styles::content>
                <Suspense fallback=|| view! { <LoadingSpinner/> }>
                    {move || Suspend::new(async move {
                        let song_list = songs.await;
                        let filter = only_with_chords.get();
                        let filtered: Vec<_> = if filter {
                            song_list.into_iter().filter(|s| s.has_chords).collect()
                        } else {
                            song_list
                        };
                        view! {
                            <div class=styles::songList>
                                <For
                                    each=move || filtered.clone()
                                    key=|s| s.id
                                    children=|song| view! { <SongListItem song=song/> }
                                />
                            </div>
                        }
                    })}
                </Suspense>
            </div>

            <BottomNav/>
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
        <A href=format!("/songs/{}", song.id) attr:class=styles::songItem>
            <span class=styles::songNumber>
                {song.number.map(|n| format!("#{}", n)).unwrap_or_default()}
            </span>
            <div class=styles::songInfo>
                <span class=styles::songTitle>{song.title}</span>
                {first_line.map(|line| view! {
                    <span class=styles::songFirstLine>{line}</span>
                })}
            </div>
            {song.original_key.map(|k| view! {
                <span class=styles::songKey>{k}</span>
            })}
        </A>
    }
}

/// Song detail page with chords
#[component]
pub fn SongDetail() -> impl IntoView {
    let params = use_params_map();

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

    let song_title = LocalResource::new(move || async move {
        if let Some(id) = song_id() {
            api::get_song(id).await.ok().map(|s| s.title)
        } else {
            None
        }
    });

    view! {
        <div class=styles::container>
            <Header
                title=Signal::derive(move || {
                    song_title.get().flatten().unwrap_or_else(|| "Песня".to_string())
                })
                back=true
            >
                <div class=styles::transposeControls>
                    <button
                        class=styles::transposeBtn
                        on:click=move |_| transpose.update(|t| *t = (*t - 1 + 12) % 12)
                    >
                        <Icon icon=IconType::Minus size=IconSize::Small/>
                    </button>
                    <span class=styles::transposeValue>
                        {move || {
                            let t = transpose.get();
                            if t == 0 { "0".to_string() }
                            else if t > 6 { format!("-{}", 12 - t) }
                            else { format!("+{}", t) }
                        }}
                    </span>
                    <button
                        class=styles::transposeBtn
                        on:click=move |_| transpose.update(|t| *t = (*t + 1) % 12)
                    >
                        <Icon icon=IconType::Plus size=IconSize::Small/>
                    </button>
                </div>
            </Header>

            <div class=styles::songDetail>
                <Suspense fallback=|| view! { <LoadingSpinner/> }>
                    {move || Suspend::new(async move {
                        match song.await {
                            Some(s) => view! { <SongContent song=s/> }.into_any(),
                            None => view! {
                                <div class=styles::empty>
                                    <span class=styles::emptyText>"Песня не найдена"</span>
                                </div>
                            }.into_any()
                        }
                    })}
                </Suspense>
            </div>

            <BottomNav/>
        </div>
    }
}

/// Song content with parsed chords
#[component]
fn SongContent(song: Song) -> impl IntoView {
    let parsed = ChordProParser::parse(&song.content);

    view! {
        <div>
            <div class=styles::songHeader>
                <h1 class=styles::songDetailTitle>
                    {song.number.map(|n| format!("№{}. ", n)).unwrap_or_default()}
                    {song.title}
                </h1>
                <div class=styles::songMeta>
                    {song.author_lyrics.map(|a| view! { <span>{a}</span> })}
                    {parsed.key.map(|k| view! {
                        <span class=styles::songKey>"Тональность: " {k}</span>
                    })}
                </div>
            </div>

            {parsed.sections.into_iter().map(|section| {
                let section_label: Option<&str> = match section.section_type {
                    shared::SongSectionType::Chorus => Some("Припев"),
                    shared::SongSectionType::Verse => section.label.as_ref().map(|_| "Куплет"),
                    shared::SongSectionType::Bridge => Some("Бридж"),
                    shared::SongSectionType::PreChorus => Some("Предприпев"),
                    _ => None
                };

                view! {
                    <div class=styles::songSection>
                        {section_label.map(|label| {
                            let label_with_number = section.label.as_ref()
                                .map(|n| format!("{} {}", label, n))
                                .unwrap_or_else(|| label.to_string());
                            view! {
                                <div class=styles::sectionLabel>{label_with_number}</div>
                            }
                        })}

                        {section.lines.into_iter().map(|line| {
                            if line.chords.is_empty() {
                                view! {
                                    <div class=styles::songLine>
                                        <div class=styles::textLine>{line.text}</div>
                                    </div>
                                }.into_any()
                            } else {
                                let chord_line = render_chords_line(&line.text, &line.chords);
                                let text = line.text.clone();
                                view! {
                                    <div class=styles::songLine>
                                        <div class=styles::chordLine>{chord_line}</div>
                                        <div class=styles::textLine>{text}</div>
                                    </div>
                                }.into_any()
                            }
                        }).collect_view()}
                    </div>
                }
            }).collect_view()}
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
    let app_state = expect_context::<AppState>();
    let only_with_chords = app_state.only_with_chords;
    let query = RwSignal::new(String::new());
    let results = RwSignal::new(Vec::new());
    let is_searching = RwSignal::new(false);

    let do_search = move || {
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

    let filtered_results = move || {
        let all = results.get();
        if only_with_chords.get() {
            all.into_iter()
                .filter(|r| r.song.has_chords)
                .collect::<Vec<_>>()
        } else {
            all
        }
    };

    let title = Signal::derive(move || {
        if only_with_chords.get() {
            "Поиск (с аккордами)"
        } else {
            "Поиск"
        }
    });

    view! {
        <div class=styles::container>
            <Header title=title back=true/>

            <div class=styles::header style="position: static; padding: var(--space-md);">
                <input
                    type="search"
                    class=styles::searchInput
                    placeholder="Поиск песен..."
                    prop:value=query
                    on:input=move |ev| query.set(event_target_value(&ev))
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" {
                            do_search();
                        }
                    }
                />
                <button class=styles::iconBtn on:click=move |_| do_search()>
                    <Icon icon=IconType::Search size=IconSize::Medium/>
                </button>
            </div>

            <div class=styles::content>
                <Show when=move || is_searching.get()>
                    <LoadingSpinner/>
                </Show>

                <Show when=move || !filtered_results().is_empty()>
                    <div class=styles::songList>
                        <For
                            each=filtered_results
                            key=|s| s.song.id
                            children=|result| {
                                view! {
                                    <A href=format!("/songs/{}", result.song.id) attr:class=styles::songItem>
                                        <div class=styles::songInfo>
                                            <span class=styles::songTitle>{result.song.title}</span>
                                            <span class=styles::songFirstLine>
                                                {result.songbook_name.unwrap_or_default()}
                                            </span>
                                        </div>
                                    </A>
                                }
                            }
                        />
                    </div>
                </Show>

                <Show when=move || !is_searching.get() && filtered_results().is_empty() && (query.get().len() >= 2)>
                    <div class=styles::empty>
                        <span class=styles::emptyText>"Ничего не найдено"</span>
                    </div>
                </Show>
            </div>

            <BottomNav/>
        </div>
    }
}

/// Loading spinner component
#[component]
fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class=styles::empty>
            <div class=styles::skeleton style="width: 2rem; height: 2rem; border-radius: 50%;"></div>
        </div>
    }
}
