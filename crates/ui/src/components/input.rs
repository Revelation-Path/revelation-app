//! Input field component with label and suffix

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

/// Input type
#[derive(Clone, Copy, PartialEq, Default)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Password,
    Number,
    Tel,
    Search,
    Url
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Number => "number",
            Self::Tel => "tel",
            Self::Search => "search",
            Self::Url => "url"
        }
    }
}

/// Input size
#[derive(Clone, Copy, PartialEq, Default)]
pub enum InputSize {
    Small,
    #[default]
    Medium,
    Large
}

impl InputSize {
    fn class(&self) -> &'static str {
        match self {
            Self::Small => "ui-input--sm",
            Self::Medium => "ui-input--md",
            Self::Large => "ui-input--lg"
        }
    }
}

/// Universal input field component
#[component]
pub fn Input(
    /// Input value (controlled)
    #[prop(into)]
    value: Signal<String>,
    /// Change callback
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Input type
    #[prop(default = InputType::Text)]
    input_type: InputType,
    /// Input size
    #[prop(default = InputSize::Medium)]
    size: InputSize,
    /// Label text
    #[prop(optional)]
    label: Option<&'static str>,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<&'static str>,
    /// Suffix text (e.g., currency)
    #[prop(optional)]
    suffix: Option<&'static str>,
    /// Prefix text
    #[prop(optional)]
    prefix: Option<&'static str>,
    /// Helper text below input
    #[prop(optional)]
    helper: Option<&'static str>,
    /// Error message
    #[prop(optional)]
    error: Option<&'static str>,
    /// Disabled state
    #[prop(default = false)]
    disabled: bool,
    /// Required field
    #[prop(default = false)]
    required: bool,
    /// Readonly field
    #[prop(default = false)]
    readonly: bool,
    /// Additional CSS classes
    #[prop(optional)]
    class: &'static str,
    /// Input mode for mobile keyboards
    #[prop(optional)]
    inputmode: Option<&'static str>,
    /// Autocomplete attribute
    #[prop(optional)]
    autocomplete: Option<&'static str>
) -> impl IntoView {
    let has_error = error.is_some();

    let handle_input = move |ev: web_sys::Event| {
        if let Some(cb) = on_change {
            let target = ev.target().unwrap();
            let input: HtmlInputElement = target.dyn_into().unwrap();
            cb.run(input.value());
        }
    };

    let wrapper_classes = move || {
        let mut classes = vec!["ui-input-wrapper", size.class()];
        if has_error {
            classes.push("ui-input-wrapper--error");
        }
        if disabled {
            classes.push("ui-input-wrapper--disabled");
        }
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    let input_classes = move || {
        let mut classes = vec!["ui-input"];
        if prefix.is_some() {
            classes.push("ui-input--has-prefix");
        }
        if suffix.is_some() {
            classes.push("ui-input--has-suffix");
        }
        classes.join(" ")
    };

    view! {
        <div class=wrapper_classes>
            {label.map(|l| view! {
                <label class="ui-input__label">
                    {l}
                    {required.then(|| view! { <span class="ui-input__required">*</span> })}
                </label>
            })}

            <div class="ui-input__container">
                {prefix.map(|p| view! {
                    <span class="ui-input__prefix">{p}</span>
                })}

                <input
                    class=input_classes
                    type=input_type.as_str()
                    prop:value=value
                    placeholder=placeholder.unwrap_or("")
                    disabled=disabled
                    readonly=readonly
                    required=required
                    aria-invalid=has_error
                    inputmode=inputmode.unwrap_or("")
                    autocomplete=autocomplete.unwrap_or("off")
                    on:input=handle_input
                />

                {suffix.map(|s| view! {
                    <span class="ui-input__suffix">{s}</span>
                })}
            </div>

            {error.map(|e| view! {
                <span class="ui-input__error">{e}</span>
            })}

            {helper.filter(|_| error.is_none()).map(|h| view! {
                <span class="ui-input__helper">{h}</span>
            })}
        </div>
    }
}

/// Textarea component
#[component]
pub fn Textarea(
    /// Textarea value (controlled)
    #[prop(into)]
    value: Signal<String>,
    /// Change callback
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Label text
    #[prop(optional)]
    label: Option<&'static str>,
    /// Placeholder text
    #[prop(optional)]
    placeholder: Option<&'static str>,
    /// Number of rows
    #[prop(default = 4)]
    rows: u32,
    /// Helper text below textarea
    #[prop(optional)]
    helper: Option<&'static str>,
    /// Error message
    #[prop(optional)]
    error: Option<&'static str>,
    /// Disabled state
    #[prop(default = false)]
    disabled: bool,
    /// Required field
    #[prop(default = false)]
    required: bool,
    /// Additional CSS classes
    #[prop(optional)]
    class: &'static str
) -> impl IntoView {
    let has_error = error.is_some();

    let handle_input = move |ev: web_sys::Event| {
        if let Some(cb) = on_change {
            let target = ev.target().unwrap();
            let input: web_sys::HtmlTextAreaElement = target.dyn_into().unwrap();
            cb.run(input.value());
        }
    };

    let wrapper_classes = move || {
        let mut classes = vec!["ui-input-wrapper"];
        if has_error {
            classes.push("ui-input-wrapper--error");
        }
        if disabled {
            classes.push("ui-input-wrapper--disabled");
        }
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    view! {
        <div class=wrapper_classes>
            {label.map(|l| view! {
                <label class="ui-input__label">
                    {l}
                    {required.then(|| view! { <span class="ui-input__required">*</span> })}
                </label>
            })}

            <textarea
                class="ui-textarea"
                prop:value=value
                placeholder=placeholder.unwrap_or("")
                rows=rows
                disabled=disabled
                required=required
                aria-invalid=has_error
                on:input=handle_input
            ></textarea>

            {error.map(|e| view! {
                <span class="ui-input__error">{e}</span>
            })}

            {helper.filter(|_| error.is_none()).map(|h| view! {
                <span class="ui-input__helper">{h}</span>
            })}
        </div>
    }
}

/// Select component
#[component]
pub fn Select<T>(
    /// Selected value
    #[prop(into)]
    value: Signal<T>,
    /// Change callback
    on_change: Callback<T>,
    /// Options list
    options: Vec<(T, &'static str)>,
    /// Label text
    #[prop(optional)]
    label: Option<&'static str>,
    /// Disabled state
    #[prop(default = false)]
    disabled: bool,
    /// Additional CSS classes
    #[prop(optional)]
    class: &'static str
) -> impl IntoView
where
    T: Clone + PartialEq + Send + Sync + 'static
{
    let wrapper_classes = move || {
        let mut classes = vec!["ui-input-wrapper"];
        if disabled {
            classes.push("ui-input-wrapper--disabled");
        }
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    view! {
        <div class=wrapper_classes>
            {label.map(|l| view! {
                <label class="ui-input__label">{l}</label>
            })}

            <div class="ui-select__container">
                <select
                    class="ui-select"
                    disabled=disabled
                    on:change=move |ev| {
                        let target = event_target::<web_sys::HtmlSelectElement>(&ev);
                        let idx = target.selected_index() as usize;
                        if let Some((val, _)) = options.get(idx) {
                            on_change.run(val.clone());
                        }
                    }
                >
                    {options.iter().map(|(val, label)| {
                        let is_selected = val.clone() == value.get();
                        view! {
                            <option selected=is_selected>{*label}</option>
                        }
                    }).collect_view()}
                </select>
                <span class="ui-select__chevron"></span>
            </div>
        </div>
    }
}
