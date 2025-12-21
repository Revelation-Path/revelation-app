//! Skeleton loader component

use leptos::prelude::*;

/// Skeleton animation type
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SkeletonAnimation {
    #[default]
    Shimmer,
    Pulse,
    Wave,
    None
}

impl SkeletonAnimation {
    fn class(&self) -> &'static str {
        match self {
            Self::Shimmer => "ui-skeleton--shimmer",
            Self::Pulse => "ui-skeleton--pulse",
            Self::Wave => "ui-skeleton--wave",
            Self::None => ""
        }
    }
}

/// Skeleton shape
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SkeletonShape {
    #[default]
    Rectangle,
    Circle,
    Rounded,
    Text
}

impl SkeletonShape {
    fn class(&self) -> &'static str {
        match self {
            Self::Rectangle => "ui-skeleton--rect",
            Self::Circle => "ui-skeleton--circle",
            Self::Rounded => "ui-skeleton--rounded",
            Self::Text => "ui-skeleton--text"
        }
    }
}

/// Skeleton loader component
#[component]
pub fn Skeleton(
    /// Width (CSS value)
    #[prop(optional)]
    width: Option<&'static str>,
    /// Height (CSS value)
    #[prop(optional)]
    height: Option<&'static str>,
    /// Shape variant
    #[prop(default = SkeletonShape::Rectangle)]
    shape: SkeletonShape,
    /// Animation type
    #[prop(default = SkeletonAnimation::Shimmer)]
    animation: SkeletonAnimation,
    /// Border radius (CSS value)
    #[prop(optional)]
    radius: Option<&'static str>,
    /// Additional CSS classes
    #[prop(optional)]
    class: &'static str,
    /// Number of repetitions
    #[prop(default = 1)]
    count: usize
) -> impl IntoView {
    let style = move || {
        let mut parts = Vec::new();
        if let Some(w) = width {
            parts.push(format!("width: {}", w));
        }
        if let Some(h) = height {
            parts.push(format!("height: {}", h));
        }
        if let Some(r) = radius {
            parts.push(format!("border-radius: {}", r));
        }
        parts.join("; ")
    };

    let classes = move || {
        let mut classes = vec!["ui-skeleton", shape.class(), animation.class()];
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    if count > 1 {
        view! {
            <div class="ui-skeleton-group">
                {(0..count).map(|_| view! {
                    <div class=classes style=style></div>
                }).collect_view()}
            </div>
        }
        .into_any()
    } else {
        view! {
            <div class=classes style=style></div>
        }
        .into_any()
    }
}

/// Skeleton text lines
#[component]
pub fn SkeletonText(
    /// Number of lines
    #[prop(default = 3)]
    lines: usize,
    /// Animation type
    #[prop(default = SkeletonAnimation::Shimmer)]
    animation: SkeletonAnimation
) -> impl IntoView {
    view! {
        <div class="ui-skeleton-text">
            {(0..lines).map(|i| {
                let width = if i == lines - 1 { "60%" } else { "100%" };
                view! {
                    <Skeleton
                        shape=SkeletonShape::Text
                        animation=animation
                        width=width
                    />
                }
            }).collect_view()}
        </div>
    }
}

/// Skeleton avatar
#[component]
pub fn SkeletonAvatar(
    /// Size (CSS value)
    #[prop(default = "48px")]
    size: &'static str,
    /// Animation type
    #[prop(default = SkeletonAnimation::Shimmer)]
    animation: SkeletonAnimation
) -> impl IntoView {
    view! {
        <Skeleton
            shape=SkeletonShape::Circle
            animation=animation
            width=size
            height=size
        />
    }
}

/// Skeleton card - common loading pattern
#[component]
pub fn SkeletonCard(
    /// Animation type
    #[prop(default = SkeletonAnimation::Shimmer)]
    animation: SkeletonAnimation
) -> impl IntoView {
    view! {
        <div class="ui-skeleton-card">
            <div class="ui-skeleton-card__header">
                <SkeletonAvatar animation=animation />
                <div class="ui-skeleton-card__meta">
                    <Skeleton shape=SkeletonShape::Text animation=animation width="120px" />
                    <Skeleton shape=SkeletonShape::Text animation=animation width="80px" />
                </div>
            </div>
            <SkeletonText lines=3 animation=animation />
        </div>
    }
}
