//! Tag filter for the research notes list.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteSummary {
    pub href: String,
    pub title: String,
    pub citations: u32,
    /// `YYYY-MM`
    pub month: String,
    pub tags: Vec<String>,
}

/// Selecting tags narrows the list: a note must have every selected tag to stay visible.
#[island]
pub fn NoteFilter(notes: Vec<NoteSummary>, tags: Vec<String>) -> impl IntoView {
    let selected = RwSignal::new(Vec::<String>::new());

    let toggle = move |tag: String| {
        selected.update(|chosen| match chosen.iter().position(|t| *t == tag) {
            Some(index) => {
                chosen.remove(index);
            }
            None => chosen.push(tag),
        });
    };

    let buttons = tags
        .into_iter()
        .map(|tag| {
            let label = tag.clone();
            let on_click = tag.clone();
            let is_active = move || selected.with(|chosen| chosen.contains(&tag));
            view! {
                <button type="button" class="tag-btn" class:active=is_active on:click=move |_| toggle(on_click.clone())>
                    {label}
                </button>
            }
        })
        .collect_view();

    let items = notes
        .into_iter()
        .map(|note| {
            let tag_text = note.tags.join(", ");
            let note_tags = note.tags;
            let hidden =
                move || selected.with(|chosen| !chosen.iter().all(|t| note_tags.contains(t)));
            view! {
                <li class="note-item" hidden=hidden>
                    <a href=note.href>{note.title}</a>
                    <span class="note-meta">
                        {format!("({}) {}", note.citations, note.month)}
                        {(!tag_text.is_empty()).then(|| format!(" · Tags: {tag_text}"))}
                    </span>
                </li>
            }
        })
        .collect_view();

    view! {
        <div class="tag-list">
            <button
                type="button"
                class="tag-btn"
                class:active=move || selected.with(Vec::is_empty)
                on:click=move |_| selected.set(Vec::new())
            >
                "All"
            </button>
            {buttons}
        </div>
        <ul class="note-list">{items}</ul>
    }
}
