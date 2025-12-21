//! Navigation components - responsive sidebar/bottom nav

use leptos::{prelude::*, reactive::wrappers::read::Signal};
use leptos_router::{components::A, hooks::use_location};

#[allow(dead_code)]
mod styles {
    stylance::import_crate_style!(pub nav, "src/styles/nav.module.css");
}
use styles::nav;

/// Responsive navigation - bottom on mobile, sidebar on desktop
#[component]
pub fn BottomNav() -> impl IntoView {
    let location = use_location();
    let current_path = move || location.pathname.get();

    view! {
        <nav class=nav::bottomNav>
            <div class=nav::bottomNavInner>
                <NavItem href="/feed" icon=NavIcon::Feed label="Лента" current=current_path/>
                <NavItem href="/bible" icon=NavIcon::Bible label="Библия" current=current_path/>
                <NavItem href="/search" icon=NavIcon::Search label="Поиск" current=current_path/>
                <NavItem href="/churches" icon=NavIcon::Church label="Церкви" current=current_path/>
                <NavItem href="/profile" icon=NavIcon::Profile label="Профиль" current=current_path/>
            </div>
        </nav>
    }
}

/// Desktop sidebar - visible on lg and up
#[component]
pub fn Sidebar() -> impl IntoView {
    let state = expect_context::<crate::state::AppState>();
    let collapsed = state.sidebar_collapsed;
    let current_book = state.current_book;
    let location = use_location();
    let current_path = move || location.pathname.get();

    let book_color = move || get_book_category_var(current_book.get());

    view! {
        <aside
            class=move || format!("{} {}", nav::sidebar, if collapsed.get() { nav::sidebarCollapsed } else { nav::sidebarExpanded })
            style:background=book_color
        >
            <div class=nav::sidebarHeader>
                <button class=nav::sidebarLogo on:click=move |_| collapsed.update(|v| *v = !*v)>
                    <CrossIcon/>
                </button>
                <Show when=move || !collapsed.get()>
                    <span class=nav::sidebarTitle>"Откровение"</span>
                </Show>
            </div>
            <nav class=nav::sidebarNav>
                <SidebarItem href="/bible" icon=NavIcon::Bible label="Библия" current=Signal::derive(current_path) collapsed=collapsed/>
                <SidebarItem href="/search" icon=NavIcon::Search label="Поиск" current=Signal::derive(current_path) collapsed=collapsed/>
                <SidebarItem href="/feed" icon=NavIcon::Feed label="Лента" current=Signal::derive(current_path) collapsed=collapsed/>
                <SidebarItem href="/churches" icon=NavIcon::Church label="Церкви" current=Signal::derive(current_path) collapsed=collapsed/>
            </nav>
            <div class=nav::sidebarBottom>
                <SidebarItem href="/profile" icon=NavIcon::Profile label="Профиль" current=Signal::derive(current_path) collapsed=collapsed/>
                <Show when=move || !collapsed.get()>
                    <div class=nav::version>
                        "v" {env!("CARGO_PKG_VERSION")}
                    </div>
                </Show>
            </div>
        </aside>
    }
}

#[derive(Clone, Copy)]
enum NavIcon {
    Feed,
    Bible,
    Search,
    Church,
    Profile
}

#[component]
fn NavItem(
    href: &'static str,
    icon: NavIcon,
    label: &'static str,
    current: impl Fn() -> String + 'static + Copy + Send + Sync
) -> impl IntoView {
    let is_active = move || current().starts_with(href);

    view! {
        <A href=href attr:class=move || format!("{} {}", nav::navItem, if is_active() { "active" } else { "" })>
            {match icon {
                NavIcon::Feed => view! { <FeedIcon/> }.into_any(),
                NavIcon::Bible => view! { <BibleIcon/> }.into_any(),
                NavIcon::Search => view! { <SearchIcon/> }.into_any(),
                NavIcon::Church => view! { <ChurchIcon/> }.into_any(),
                NavIcon::Profile => view! { <ProfileIcon/> }.into_any(),
            }}
            <span>{label}</span>
        </A>
    }
}

#[component]
fn SidebarItem(
    href: &'static str,
    icon: NavIcon,
    label: &'static str,
    #[prop(into)] current: Signal<String>,
    collapsed: RwSignal<bool>
) -> impl IntoView {
    let is_active = move || current.get().starts_with(href);

    view! {
        <A
            href=href
            attr:class=move || format!(
                "{} {}",
                nav::sidebarItem,
                if is_active() { nav::sidebarItemActive } else { "" }
            )
        >
            {match icon {
                NavIcon::Feed => view! { <FeedIcon/> }.into_any(),
                NavIcon::Bible => view! { <BibleIcon/> }.into_any(),
                NavIcon::Search => view! { <SearchIcon/> }.into_any(),
                NavIcon::Church => view! { <ChurchIcon/> }.into_any(),
                NavIcon::Profile => view! { <ProfileIcon/> }.into_any(),
            }}
            <Show when=move || !collapsed.get()>
                <span>{label}</span>
            </Show>
        </A>
    }
}

/// Header component - responsive
#[component]
pub fn Header(#[prop(into)] title: String, #[prop(optional)] back: bool) -> impl IntoView {
    view! {
        <header class="header safe-top">
            <div class="flex items-center gap-4 px-4 py-3 max-w-4xl mx-auto">
                {back.then(|| view! {
                    <button
                        onclick="history.back()"
                        class="btn-ghost p-2 -ml-2"
                    >
                        <BackIcon/>
                    </button>
                })}
                <h1 class="header-title">{title}</h1>
            </div>
        </header>
    }
}

// Icons
#[component]
fn CrossIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white" width="20" height="20">
            <path d="M11 2v7H4v4h7v9h2v-9h7V9h-7V2z"/>
        </svg>
    }
}

#[component]
fn BackIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="20" height="20">
            <polyline points="15 18 9 12 15 6"/>
        </svg>
    }
}

#[component]
fn FeedIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="24" height="24">
            <path d="M4 11a9 9 0 0 1 9 9"/>
            <path d="M4 4a16 16 0 0 1 16 16"/>
            <circle cx="5" cy="19" r="1"/>
        </svg>
    }
}

#[component]
fn BibleIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="24" height="24">
            <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
            <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
            <path d="M12 6v8"/>
            <path d="M8 10h8"/>
        </svg>
    }
}

#[component]
fn SearchIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="24" height="24">
            <circle cx="11" cy="11" r="8"/>
            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
    }
}

#[component]
fn ChurchIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="24" height="24">
            <path d="M18 21V9l-6-4-6 4v12"/>
            <path d="M12 1v4"/>
            <path d="M9 5h6"/>
            <path d="M9 21v-6h6v6"/>
        </svg>
    }
}

#[component]
fn ProfileIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="24" height="24">
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
            <circle cx="12" cy="7" r="4"/>
        </svg>
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
