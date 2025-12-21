//! Main application

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    path
};
use ui::{THEME_CSS, ThemeProvider, ToastProvider};

use crate::{components::Sidebar, pages::*, state::AppState};

/// Main application component
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
