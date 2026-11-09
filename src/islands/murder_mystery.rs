//! Companion for the Medieval Murder Mystery party game. Saved state uses the same cookie
//! names as the original page.

use std::{collections::HashMap, time::Duration};

use leptos::prelude::*;
use wasm_bindgen::JsCast;

const STATS: [&str; 6] = ["str", "dex", "con", "int", "luck", "cha"];
const ANOTHER_PLAYER: &str = "Another Player";
const COOKIE_MAX_AGE_SECONDS: u32 = 7 * 24 * 60 * 60;
const BASE_COOLDOWN_MS: f64 = 3.0 * 60.0 * 1000.0;
const COOLDOWN_MS_PER_CON: f64 = 15_000.0;

const STAT_HELP: &str = "STR: Strength: Determines melee attack power\n\
DEX: Dexterity: Determines ranged attack power and ability to steal\n\
CON: Constitution: Decreases cooldown period\n\
INT: Intelligence: Determines magic attack power\n\
LUCK: Luck: Allows rolling multiple dice\n\
CHA: Charisma: Determines ability to charm NPCs";

/// Stats are in the order of [`STATS`].
const NPCS: [(&str, [i32; 6]); 14] = [
    ("Crow", [0, 5, 0, 7, 9, 0]),
    ("First Princess", [2, 4, 0, 5, 3, 7]),
    ("Fortune Teller", [0, 0, 0, 0, 6, 3]),
    ("Guard 1", [5, 3, 0, 3, 0, 0]),
    ("Guard 2", [2, 2, 0, 2, 0, 2]),
    ("Jester", [0, 0, 0, 6, 3, 5]),
    ("Queen", [7, 7, 0, 7, 0, 7]),
    ("Royal Dog", [3, 2, 0, 3, 3, 3]),
    ("Second Princess", [3, 4, 0, 4, 3, 4]),
    ("Street Cat", [1, 5, 0, 5, 7, 3]),
    ("Town Begger - Sir Saint Duggery", [4, 4, 0, 4, 0, 0]),
    ("Town Folk 1 - Jack", [1, 1, 0, 1, 0, 1]),
    ("Town Folk 2 - Jill", [0, 1, 0, 0, 0, 3]),
    ("Town Folk 3 - James", [2, 1, 0, 0, 9, 1]),
];

const CHARACTERS: [(&str, [i32; 6]); 17] = [
    ("Bella - Bobert Bouzqueeffious", [0, 0, 0, 3, 0, 2]),
    ("Caleb - Aonn Rivers", [0, 0, 0, 3, 2, 0]),
    ("Carly - Bongo", [0, 2, 1, 0, 0, 2]),
    ("Cindy - Noctyra Hexwrn", [0, 0, 0, 2, 0, 3]),
    ("Dylan - Merlin", [0, 0, 0, 5, 0, 0]),
    ("Hannah - Dewalt The Dragon", [3, 1, 0, 1, 0, 0]),
    ("Kenzie - Ingrid Stormcloak", [1, 4, 0, 0, 0, 0]),
    ("Leti - Mistress Belladonna", [0, 0, 0, 1, 0, 4]),
    ("Lorél - Chromie", [3, 0, 0, 1, 0, 1]),
    ("Madeline - Algea", [0, 0, 2, 0, 3, 0]),
    ("Madi - Trinket Brassfield", [0, 3, 0, 3, 0, -1]),
    ("Melissa - Merrick", [2, 0, 0, 0, 0, 3]),
    ("Philip - Holark", [4, 0, 2, 2, 0, -3]),
    ("Robert - Rian Josling", [0, 2, 1, 1, 3, -2]),
    ("Rose - Aster Rhys", [0, 0, 1, 3, 2, -1]),
    ("Tyler - Aldrich Blackthorn", [2, 0, 1, 2, 0, 0]),
    ("I Just Showed Up Here", [1, 1, 1, 1, 0, 1]),
];

type Stats = HashMap<String, i32>;

fn to_stats(values: &[i32; 6]) -> Stats {
    STATS
        .iter()
        .map(|s| (s.to_string(), 0))
        .zip(values)
        .map(|((k, _), v)| (k, *v))
        .collect()
}

fn base_stats(character: &str) -> Stats {
    let values = CHARACTERS
        .iter()
        .find(|(name, _)| *name == character)
        .map_or([0; 6], |(_, values)| *values);
    to_stats(&values)
}

fn npc_stats(npc: &str) -> Option<Stats> {
    NPCS.iter()
        .find(|(name, _)| *name == npc)
        .map(|(_, values)| to_stats(values))
}

fn stat(stats: &Stats, name: &str) -> i32 {
    stats.get(name).copied().unwrap_or(0)
}

/// Each three points of luck adds a die. Always at least one, so negative luck can't skip the roll.
fn dice_count(luck: i32) -> i32 {
    (1 + luck.div_euclid(3)).max(1)
}

fn html_document() -> web_sys::HtmlDocument {
    document().unchecked_into()
}

fn get_cookie(name: &str) -> Option<String> {
    let all = html_document().cookie().ok()?;
    let raw = all.split("; ").find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        (key == name).then_some(value.to_string())
    })?;
    js_sys::decode_uri_component(&raw).ok().map(String::from)
}

fn set_cookie(name: &str, value: &str) {
    let encoded = String::from(js_sys::encode_uri_component(value));
    let cookie = format!("{name}={encoded}; max-age={COOKIE_MAX_AGE_SECONDS}; path=/");
    let _ = html_document().set_cookie(&cookie);
}

fn cooldown_key(character: &str, npc: &str) -> String {
    format!("mm_cd_{character}_{npc}")
}

fn cooldown_until(character: &str, npc: &str) -> f64 {
    get_cookie(&cooldown_key(character, npc))
        .and_then(|value| value.parse().ok())
        .unwrap_or(0.0)
}

fn cooldown_ms(con: i32) -> f64 {
    (BASE_COOLDOWN_MS - f64::from(con) * COOLDOWN_MS_PER_CON).max(0.0)
}

fn roll(stat_name: &str, stats: &Stats) -> i32 {
    let mut best = 0;
    for _ in 0..dice_count(stat(stats, "luck")) {
        best = best.max((js_sys::Math::random() * 20.0).floor() as i32 + 1);
    }
    best + stat(stats, stat_name)
}

#[derive(Clone)]
struct LogEntry {
    text: String,
    /// Colored suffix: `true` for a success, `false` for a failure.
    outcome: Option<(String, bool)>,
}

impl LogEntry {
    fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            outcome: None,
        }
    }

    fn outcome(text: impl Into<String>, label: &str, success: bool) -> Self {
        Self {
            text: text.into(),
            outcome: Some((label.to_string(), success)),
        }
    }
}

fn stat_used(action: &str, attack_type: &str) -> &'static str {
    match (action, attack_type) {
        ("steal", _) | ("attack", "ranged") => "dex",
        ("attack", "melee") => "str",
        ("attack", _) => "int",
        _ => "cha",
    }
}

#[island]
pub fn MurderMysteryGame() -> impl IntoView {
    let first_character = CHARACTERS[0].0.to_string();
    let character = RwSignal::new(first_character.clone());
    let stats = RwSignal::new(base_stats(&first_character));
    let npc = RwSignal::new(NPCS[0].0.to_string());
    let attack_type = RwSignal::new("melee".to_string());
    let log = RwSignal::new(Vec::<LogEntry>::new());
    let cooldown_label = RwSignal::new("Select NPC to see cooldown".to_string());
    let on_cooldown = RwSignal::new(false);

    let save = move || {
        set_cookie("mm_character_name", &character.get_untracked());
        if let Ok(json) = stats.with_untracked(serde_json::to_string) {
            set_cookie("mm_character_stats", &json);
        }
    };

    let refresh_cooldown = move || {
        let target = npc.get_untracked();
        if target == ANOTHER_PLAYER {
            cooldown_label.set(String::new());
            on_cooldown.set(false);
            return;
        }
        let remaining = cooldown_until(&character.get_untracked(), &target) - js_sys::Date::now();
        if remaining > 0.0 {
            cooldown_label.set(format!("Cooldown: {}s", (remaining / 1000.0).ceil()));
            on_cooldown.set(true);
        } else {
            cooldown_label.set("Ready".to_string());
            on_cooldown.set(false);
        }
    };

    // Browser-only start-up: restore the saved character and keep the cooldown label current.
    Effect::new(move |_| {
        if let (Some(name), Some(saved)) = (
            get_cookie("mm_character_name"),
            get_cookie("mm_character_stats"),
        ) {
            let known = CHARACTERS.iter().any(|(candidate, _)| *candidate == name);
            if let (true, Ok(parsed)) = (known, serde_json::from_str::<Stats>(&saved)) {
                character.set(name);
                stats.set(parsed);
                log.update(|entries| entries.insert(0, LogEntry::plain("Loaded saved character")));
            }
        }
        refresh_cooldown();
        set_interval(refresh_cooldown, Duration::from_millis(500));
    });

    let change_character = move |ev| {
        let name = event_target_value(&ev);
        stats.set(base_stats(&name));
        character.set(name);
        save();
        refresh_cooldown();
    };

    let adjust = move |name: &'static str, delta: i32| {
        stats.update(|current| *current.entry(name.to_string()).or_insert(0) += delta);
        save();
        refresh_cooldown();
    };

    let interact = move |action: &'static str| {
        let target = npc.get_untracked();
        let who = character.get_untracked();
        let add = move |entry: LogEntry| log.update(|entries| entries.insert(0, entry));

        if target != ANOTHER_PLAYER && cooldown_until(&who, &target) > js_sys::Date::now() {
            add(LogEntry::outcome(
                "",
                &format!("{target} is on cooldown ⏳"),
                false,
            ));
            return;
        }

        let attack = attack_type.get_untracked();
        let stat_name = stat_used(action, &attack);
        let player_roll = stats.with_untracked(|current| roll(stat_name, current));
        let upper = action.to_uppercase();
        let kind = if action == "attack" {
            attack.to_uppercase()
        } else {
            upper.clone()
        };

        let Some(opponent) = npc_stats(&target) else {
            add(LogEntry::plain(format!(
                "PVP {upper} [{kind}]: Roll = {player_roll} 🎲"
            )));
            return;
        };

        let npc_roll = roll(stat_name, &opponent);
        let con = stats.with_untracked(|current| stat(current, "con"));
        set_cookie(
            &cooldown_key(&who, &target),
            &(js_sys::Date::now() + cooldown_ms(con)).to_string(),
        );
        refresh_cooldown();

        let detail = if action == "attack" {
            format!(" [{kind}]")
        } else {
            String::new()
        };
        let text = format!("{upper} vs {target}{detail}: {player_roll} vs {npc_roll} → ");
        let success = player_roll > npc_roll;
        add(LogEntry::outcome(
            text,
            if success { "SUCCESS 🎉" } else { "FAIL ❌" },
            success,
        ));
    };

    let character_options = CHARACTERS
        .iter()
        .map(|(name, _)| view! { <option value=*name>{*name}</option> })
        .collect_view();
    let npc_options = NPCS
        .iter()
        .map(|(name, _)| view! { <option value=*name>{*name}</option> })
        .collect_view();

    let stat_rows = STATS
        .iter()
        .map(|name| {
            let label = name.to_uppercase();
            let value = move || {
                let current = stats.with(|s| stat(s, name));
                let base = character.with(|c| stat(&base_stats(c), name));
                let bonus = current - base;
                match bonus {
                    0 => base.to_string(),
                    b if b > 0 => format!("{base} + {b}"),
                    b => format!("{base} - {}", b.abs()),
                }
            };
            view! { <div>{label}</div><div>{value}</div> }
        })
        .collect_view();

    let adjust_rows = STATS
        .iter()
        .map(|name| {
            view! {
                <div>
                    <span class="item-label">{name.to_uppercase()}</span>
                    <button type="button" on:click=move |_| adjust(name, -1)>"−"</button>
                    <button type="button" on:click=move |_| adjust(name, 1)>"+"</button>
                </div>
            }
        })
        .collect_view();

    let entries = move || {
        log.get()
            .into_iter()
            .map(|entry| {
                view! {
                    <div>
                        {entry.text}
                        {entry.outcome.map(|(label, success)| {
                            view! { <span class=if success { "success" } else { "fail" }>{label}</span> }
                        })}
                    </div>
                }
            })
            .collect_view()
    };

    view! {
        <h1 class="mm-title">"🗡️ Medieval Murder Mystery"</h1>
        <div class="mm-container">
            <div class="panel">
                <h2>"Character"</h2>
                <select prop:value=move || character.get() on:change=change_character>{character_options}</select>

                <h3>
                    "Stats "
                    <button
                        type="button"
                        class="tooltip-btn"
                        aria-label="What the stats do"
                        on:click=move |_| { let _ = window().alert_with_message(STAT_HELP); }
                    >
                        "?"
                    </button>
                </h3>
                <div class="stats">{stat_rows}</div>

                <h3>"Adjust Stats"</h3>
                <div class="items">{adjust_rows}</div>
            </div>

            <div class="panel">
                <h2>"Interaction"</h2>
                <select
                    prop:value=move || npc.get()
                    on:change=move |ev| {
                        npc.set(event_target_value(&ev));
                        refresh_cooldown();
                    }
                >
                    {npc_options}
                    <option value=ANOTHER_PLAYER>{ANOTHER_PLAYER}</option>
                </select>
                <div class="cooldown">{move || cooldown_label.get()}</div>

                <h3>"Action"</h3>
                <button type="button" disabled=move || on_cooldown.get() on:click=move |_| interact("charm")>"Charm"</button>
                <button type="button" disabled=move || on_cooldown.get() on:click=move |_| interact("steal")>"Steal"</button>
                <button type="button" disabled=move || on_cooldown.get() on:click=move |_| interact("attack")>"Attack"</button>

                <h3>"Attack Type"</h3>
                <select prop:value=move || attack_type.get() on:change=move |ev| attack_type.set(event_target_value(&ev))>
                    <option value="melee">"Melee"</option>
                    <option value="ranged">"Ranged"</option>
                    <option value="magic">"Magic"</option>
                </select>

                <h3>"Event Log"</h3>
                <div class="log">{entries}</div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{base_stats, cooldown_ms, dice_count, stat, stat_used};

    #[test]
    fn luck_sets_the_number_of_dice() {
        assert_eq!(dice_count(0), 1);
        assert_eq!(dice_count(2), 1);
        assert_eq!(dice_count(3), 2);
        assert_eq!(dice_count(9), 4);
    }

    #[test]
    fn negative_luck_still_rolls_once() {
        assert_eq!(dice_count(-1), 1);
        assert_eq!(dice_count(-30), 1);
    }

    #[test]
    fn constitution_shortens_cooldowns_down_to_zero() {
        assert_eq!(cooldown_ms(0), 180_000.0);
        assert_eq!(cooldown_ms(4), 120_000.0);
        assert_eq!(cooldown_ms(12), 0.0);
        assert_eq!(cooldown_ms(40), 0.0);
        assert_eq!(cooldown_ms(-1), 195_000.0);
    }

    #[test]
    fn each_action_uses_the_right_stat() {
        assert_eq!(stat_used("charm", "melee"), "cha");
        assert_eq!(stat_used("steal", "magic"), "dex");
        assert_eq!(stat_used("attack", "melee"), "str");
        assert_eq!(stat_used("attack", "ranged"), "dex");
        assert_eq!(stat_used("attack", "magic"), "int");
    }

    #[test]
    fn characters_keep_their_stats() {
        let philip = base_stats("Philip - Holark");
        assert_eq!(stat(&philip, "str"), 4);
        assert_eq!(stat(&philip, "cha"), -3);
    }
}
