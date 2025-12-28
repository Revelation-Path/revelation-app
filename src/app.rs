//! Main application.

use leptos::prelude::*;
use leptos_meta::{Meta, Style, Title, provide_meta_context};
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    path
};
use revelation_ui::{BrowserChrome, THEME_CSS, ThemeProvider, ToastProvider};

use crate::{
    components::Sidebar,
    pages::{
        Bible, BibleChapter, ChurchDetail, Churches, DailyReading, Feed, NotFound, Onboarding,
        Profile, Search, Settings, SongDetail, SongSearch, SongbookDetail, SongbookSongs, Songs
    },
    state::AppState
};

/// Syncs browser chrome color with current book
#[component]
fn ChromeSync() -> impl IntoView {
    let state = expect_context::<AppState>();

    Effect::new(move |_| {
        let book_id = state.current_book.get();
        BrowserChrome::set_book_category(book_id);
    });
}

/// Main application component.
#[must_use]
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Initialize app state
    let state = AppState::init();
    provide_context(state);

    view! {
        <Meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover"/>
        <Meta name="theme-color" content="#fefdfb"/>
        <Meta name="apple-mobile-web-app-capable" content="yes"/>
        <Meta name="apple-mobile-web-app-status-bar-style" content="default"/>
        <Title text="Revelation - Библия"/>
        <Style>{THEME_CSS}</Style>

        <ThemeProvider>
            <ChromeSync/>
            <ToastProvider>
                <Router>
                    <div class="flex">
                        // Desktop sidebar
                        <Sidebar/>

                        // Main content area
                        <div class="flex-1">
                            <Routes fallback=|| view! { <NotFound/> }>
                                <Route path=path!("/") view=|| view! { <Redirect path="/bible"/> }/>
                                <Route path=path!("/onboarding") view=Onboarding/>
                                <Route path=path!("/feed") view=Feed/>
                                <Route path=path!("/bible") view=Bible/>
                                <Route path=path!("/bible/:book/:chapter") view=BibleChapter/>
                                <Route path=path!("/search") view=Search/>
                                <Route path=path!("/today") view=DailyReading/>
                                <Route path=path!("/churches") view=Churches/>
                                <Route path=path!("/church/:id") view=ChurchDetail/>
                                <Route path=path!("/songs") view=Songs/>
                                <Route path=path!("/songs/search") view=SongSearch/>
                                <Route path=path!("/songs/book/:id") view=SongbookDetail/>
                                <Route path=path!("/songs/book/:id/songs") view=SongbookSongs/>
                                <Route path=path!("/songs/:id") view=SongDetail/>
                                <Route path=path!("/profile") view=Profile/>
                                <Route path=path!("/settings") view=Settings/>
                            </Routes>
                        </div>
                    </div>
                </Router>
            </ToastProvider>
        </ThemeProvider>
    }
}
