//! Settings page - unified app settings

use leptos::prelude::*;
use revelation_ui::theme::{FontFamily, Theme, use_theme};
use wasm_bindgen::prelude::*;

use crate::{components::Header, haptic};

stylance::import_crate_style!(styles, "src/styles/settings.module.css");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "isPwaInstalled")]
    fn is_pwa_installed() -> bool;

    #[wasm_bindgen(js_name = "isPwaInstallable")]
    fn is_pwa_installable() -> bool;

    #[wasm_bindgen(js_name = "triggerPwaInstall")]
    fn trigger_pwa_install();

    #[wasm_bindgen(js_name = "isIOSDevice")]
    fn is_ios_device() -> bool;
}

#[must_use]
#[component]
pub fn Settings() -> impl IntoView {
    let ts = use_theme();
    let (pwa_installable, set_pwa_installable) = signal(is_pwa_installable());

    Effect::new(move |_| {
        if let Some(win) = web_sys::window() {
            let closure = Closure::wrap(Box::new(move || {
                set_pwa_installable.set(true);
            }) as Box<dyn Fn()>);
            let _ = win.add_event_listener_with_callback(
                "pwainstallable",
                closure.as_ref().unchecked_ref()
            );
            closure.forget();
        }
    });

    view! {
        <div class=styles::page>
            <Header title="Настройки" back=true/>
            <div class=styles::container>
                <ReadingSection ts=ts/>
                <ThemeSection theme=ts.theme/>
                <AppSection pwa_installable=pwa_installable set_pwa_installable=set_pwa_installable/>
                <AboutSection/>
            </div>
        </div>
    }
}

#[must_use]
#[component]
fn ReadingSection(ts: revelation_ui::theme::ThemeState) -> impl IntoView {
    let font_family = ts.font_family;
    let font_size = ts.font_size;
    let verse_per_line = ts.verse_per_line;

    view! {
        <div class=styles::section>
            <h3 class=styles::sectionTitle>"Чтение"</h3>
            <div class=styles::sectionContent>
                <FontSizeRow font_size=font_size/>
                <FontFamilyRow font_family=font_family/>
                <VersePerLineRow verse_per_line=verse_per_line/>
            </div>
        </div>
    }
}

#[must_use]
#[component]
fn FontSizeRow(font_size: RwSignal<u8>) -> impl IntoView {
    view! {
        <div class=styles::row>
            <span class=styles::rowLabel>"Размер текста"</span>
            <div class=styles::sizeControls>
                <button class=styles::sizeBtn on:click=move |_| {
                    haptic::tap();
                    font_size.update(|s| *s = (*s).saturating_sub(2).max(12));
                }>"A-"</button>
                <span class=styles::sizeValue>{move || font_size.get()}</span>
                <button class=styles::sizeBtn on:click=move |_| {
                    haptic::tap();
                    font_size.update(|s| *s = (*s + 2).min(32));
                }>"A+"</button>
            </div>
        </div>
    }
}

#[must_use]
#[component]
fn FontFamilyRow(font_family: RwSignal<FontFamily>) -> impl IntoView {
    view! {
        <div class=styles::row>
            <span class=styles::rowLabel>"Шрифт"</span>
            <div class=styles::fontBtns>
                <button
                    class=move || {
                        let base = format!("{} {}", styles::fontBtn, styles::fontBtnSerif);
                        if font_family.get() == FontFamily::Serif {
                            format!("{base} {}", styles::fontBtnActive)
                        } else { base }
                    }
                    on:click=move |_| { haptic::tap(); font_family.set(FontFamily::Serif); }
                >"Serif"</button>
                <button
                    class=move || {
                        let base = format!("{} {}", styles::fontBtn, styles::fontBtnSans);
                        if font_family.get() == FontFamily::Sans {
                            format!("{base} {}", styles::fontBtnActive)
                        } else { base }
                    }
                    on:click=move |_| { haptic::tap(); font_family.set(FontFamily::Sans); }
                >"Sans"</button>
            </div>
        </div>
    }
}

#[must_use]
#[component]
fn VersePerLineRow(verse_per_line: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class=styles::row>
            <span class=styles::rowLabel>"Стих с новой строки"</span>
            <button
                class=move || if verse_per_line.get() {
                    format!("{} {}", styles::toggle, styles::toggleOn)
                } else { styles::toggle.to_string() }
                on:click=move |_| { haptic::tap(); verse_per_line.update(|v| *v = !*v); }
            ><span class=styles::toggleThumb/></button>
        </div>
    }
}

#[must_use]
#[component]
fn ThemeSection(theme: RwSignal<Theme>) -> impl IntoView {
    view! {
        <div class=styles::section>
            <h3 class=styles::sectionTitle>"Оформление"</h3>
            <div class=styles::sectionContent>
                <div class=styles::row>
                    <span class=styles::rowLabel>"Тема"</span>
                    <div class=styles::themeBtns>
                        <ThemeButton theme=theme target=Theme::Light style_class=styles::themeBtnLight/>
                        <ThemeButton theme=theme target=Theme::Sepia style_class=styles::themeBtnSepia/>
                        <ThemeButton theme=theme target=Theme::Dark style_class=styles::themeBtnDark/>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[must_use]
#[component]
fn ThemeButton(theme: RwSignal<Theme>, target: Theme, style_class: &'static str) -> impl IntoView {
    view! {
        <button
            class=move || {
                let base = format!("{} {style_class}", styles::themeBtn);
                if theme.get() == target { format!("{base} {}", styles::themeBtnActive) } else { base }
            }
            on:click=move |_| { haptic::tap(); theme.set(target); }
        />
    }
}

#[must_use]
#[component]
fn AppSection(
    pwa_installable: ReadSignal<bool>,
    set_pwa_installable: WriteSignal<bool>
) -> impl IntoView {
    view! {
        <div class=styles::section>
            <h3 class=styles::sectionTitle>"Приложение"</h3>
            <div class=styles::sectionContent>
                {move || {
                    let installed = is_pwa_installed();
                    let ios = is_ios_device();
                    let can_install = pwa_installable.get();
                    view! { <PwaStatus installed=installed ios=ios can_install=can_install set_pwa_installable=set_pwa_installable/> }
                }}
            </div>
        </div>
    }
}

#[must_use]
#[component]
fn PwaStatus(
    installed: bool,
    ios: bool,
    can_install: bool,
    set_pwa_installable: WriteSignal<bool>
) -> impl IntoView {
    if installed {
        view! {
            <div class=styles::row>
                <span class=styles::rowLabel>"Статус"</span>
                <span class=styles::installed><CheckIcon/>"Установлено"</span>
            </div>
        }
        .into_any()
    } else if can_install {
        view! {
            <div class=styles::row>
                <span class=styles::rowLabel>"Установка"</span>
                <button class=styles::installBtn on:click=move |_| {
                    trigger_pwa_install();
                    set_pwa_installable.set(false);
                }><InstallIcon/>"Установить"</button>
            </div>
        }
        .into_any()
    } else if ios {
        view! {
            <div class=styles::row>
                <span class=styles::rowLabel>"Установка"</span>
                <span class=styles::hint>"Нажмите " <ShareIcon/> " → На экран Домой"</span>
            </div>
        }
        .into_any()
    } else {
        view! {
            <div class=styles::row>
                <span class=styles::rowLabel>"Установка"</span>
                <span class=styles::hint>"Меню браузера → Установить"</span>
            </div>
        }
        .into_any()
    }
}

#[must_use]
#[component]
fn AboutSection() -> impl IntoView {
    view! {
        <div class=styles::section>
            <h3 class=styles::sectionTitle>"О приложении"</h3>
            <div class=styles::sectionContent>
                <div class=styles::row>
                    <span class=styles::rowLabel>"Версия"</span>
                    <span class=styles::rowValue>{env!("CARGO_PKG_VERSION")}</span>
                </div>
            </div>
        </div>
    }
}

#[must_use]
#[component]
fn CheckIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="16" height="16">
            <polyline points="20 6 9 17 4 12"/>
        </svg>
    }
}

#[must_use]
#[component]
fn InstallIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="16" height="16">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
        </svg>
    }
}

#[must_use]
#[component]
fn ShareIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round"
             stroke-linejoin="round" width="14" height="14">
            <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"/>
            <polyline points="16 6 12 2 8 6"/>
            <line x1="12" y1="2" x2="12" y2="15"/>
        </svg>
    }
}
