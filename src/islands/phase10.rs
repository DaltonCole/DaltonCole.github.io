//! Phase 10 score keeper. Games are saved in the browser under the same `players` key the
//! old JavaScript used, so a game in progress carries over.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "players";
/// A player who scores fewer points than this in a round completed their phase.
const PHASE_COMPLETE_BELOW: i64 = 50;
const COLORS: [&str; 8] = [
    "#f43f9e", "#8b5cf6", "#22d3ee", "#f5a524", "#4ade80", "#fb7185", "#60a5fa", "#facc15",
];

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Player {
    name: String,
    point_history: Vec<i64>,
    phase_history: Vec<i64>,
    color: String,
}

impl Player {
    fn total(&self) -> i64 {
        self.point_history.iter().sum()
    }

    fn phase(&self) -> i64 {
        self.phase_history.last().copied().unwrap_or(1)
    }

    fn record_round(&mut self, points: i64) {
        let phase = self.phase() + i64::from(points < PHASE_COMPLETE_BELOW);
        self.point_history.push(points);
        self.phase_history.push(phase);
    }
}

fn storage() -> Option<web_sys::Storage> {
    window().local_storage().ok().flatten()
}

fn load_players() -> Vec<Player> {
    storage()
        .and_then(|store| store.get_item(STORAGE_KEY).ok().flatten())
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

fn save_players(players: &[Player]) {
    if let (Some(store), Ok(json)) = (storage(), serde_json::to_string(players)) {
        let _ = store.set_item(STORAGE_KEY, &json);
    }
}

fn digits_only(value: &str) -> String {
    value.chars().filter(char::is_ascii_digit).collect()
}

type Series = (String, String, Vec<i64>);

fn line_chart(title: &'static str, series: Vec<Series>) -> impl IntoView {
    const WIDTH: f64 = 640.0;
    const HEIGHT: f64 = 260.0;
    const PAD: f64 = 36.0;

    let rounds = series
        .iter()
        .map(|(_, _, values)| values.len())
        .max()
        .unwrap_or(0);
    let top = series
        .iter()
        .flat_map(|(_, _, values)| values.iter().copied())
        .max()
        .unwrap_or(1)
        .max(1);
    let x = move |round: usize| {
        if rounds <= 1 {
            WIDTH / 2.0
        } else {
            PAD + round as f64 * (WIDTH - 2.0 * PAD) / (rounds - 1) as f64
        }
    };
    let y = move |value: i64| HEIGHT - PAD - value as f64 * (HEIGHT - 2.0 * PAD) / top as f64;

    let legend = series
        .iter()
        .map(|(name, color, _)| {
            view! {
                <span class="legend-item">
                    <span class="legend-dot" style=format!("background:{color}")></span>
                    {name.clone()}
                </span>
            }
        })
        .collect_view();

    let lines = series
        .into_iter()
        .map(|(_, color, values)| {
            let points = values
                .iter()
                .enumerate()
                .map(|(round, value)| format!("{:.1},{:.1}", x(round), y(*value)))
                .collect::<Vec<_>>()
                .join(" ");
            let dots = values
                .iter()
                .enumerate()
                .map(|(round, value)| {
                    view! { <circle cx=x(round) cy=y(*value) r="3.5" fill=color.clone()></circle> }
                })
                .collect_view();
            view! {
                <polyline points=points fill="none" stroke=color.clone() stroke-width="2"></polyline>
                {dots}
            }
        })
        .collect_view();

    let round_labels = (0..rounds)
        .map(|round| {
            view! {
                <text x=x(round) y=HEIGHT - 12.0 text-anchor="middle" class="chart-label">
                    {(round + 1).to_string()}
                </text>
            }
        })
        .collect_view();

    view! {
        <figure class="chart">
            <figcaption>{title}</figcaption>
            <svg viewBox=format!("0 0 {WIDTH} {HEIGHT}") role="img" aria-label=title>
                <line x1=PAD y1=y(0) x2=WIDTH - PAD y2=y(0) class="chart-axis"></line>
                <line x1=PAD y1=y(top) x2=WIDTH - PAD y2=y(top) class="chart-grid"></line>
                <text x=PAD - 8.0 y=y(0) + 4.0 text-anchor="end" class="chart-label">"0"</text>
                <text x=PAD - 8.0 y=y(top) + 4.0 text-anchor="end" class="chart-label">{top.to_string()}</text>
                {round_labels}
                {lines}
            </svg>
            <div class="chart-legend">{legend}</div>
        </figure>
    }
}

#[island]
pub fn PhaseTenScoreKeeper() -> impl IntoView {
    let players = RwSignal::new(Vec::<Player>::new());
    // Reference-counted, so a row's input outlives the "add player" row that created it.
    let round_inputs = RwSignal::new(Vec::<ArcRwSignal<String>>::new());
    let adding = RwSignal::new(false);
    let new_name = RwSignal::new(String::new());

    // Runs once in the browser; the prerendered page starts with an empty table.
    Effect::new(move |_| {
        let saved = load_players();
        round_inputs.set(
            saved
                .iter()
                .map(|_| ArcRwSignal::new(String::new()))
                .collect(),
        );
        players.set(saved);
    });

    let finish_adding = move |_| {
        let name = new_name.get_untracked().trim().to_string();
        if name.is_empty() {
            return;
        }
        players.update(|list| {
            let color = COLORS[list.len() % COLORS.len()].to_string();
            list.push(Player {
                name,
                point_history: Vec::new(),
                phase_history: Vec::new(),
                color,
            });
        });
        round_inputs.update(|inputs| inputs.push(ArcRwSignal::new(String::new())));
        new_name.set(String::new());
        adding.set(false);
        players.with_untracked(|list| save_players(list));
    };

    let complete_round = move |_| {
        let inputs = round_inputs.get_untracked();
        players.update(|list| {
            for (player, input) in list.iter_mut().zip(&inputs) {
                if let Ok(points) = input.get_untracked().parse::<i64>() {
                    player.record_round(points);
                    input.set(String::new());
                }
            }
        });
        players.with_untracked(|list| save_players(list));
    };

    let restart = move |_| {
        if window()
            .confirm_with_message("Are you sure?")
            .unwrap_or(false)
        {
            players.set(Vec::new());
            round_inputs.set(Vec::new());
            save_players(&[]);
        }
    };

    let rows = move || {
        players
            .get()
            .into_iter()
            .zip(round_inputs.get())
            .map(|(player, input)| {
                let writer = input.clone();
                view! {
                    <tr>
                        <th scope="row">{player.name.clone()}</th>
                        <td>{player.phase()}</td>
                        <td>{player.total()}</td>
                        <td>
                            <input
                                class="tool-input"
                                type="text"
                                inputmode="numeric"
                                maxlength="3"
                                aria-label=format!("Points for {}", player.name)
                                prop:value=move || input.get()
                                on:input=move |ev| writer.set(digits_only(&event_target_value(&ev)))
                            />
                        </td>
                    </tr>
                }
            })
            .collect_view()
    };

    let charts = move || {
        let list = players.get();
        list.iter().any(|p| !p.point_history.is_empty()).then(|| {
            let series = |pick: fn(&Player) -> Vec<i64>| {
                list.iter()
                    .map(|p| (p.name.clone(), p.color.clone(), pick(p)))
                    .collect::<Vec<Series>>()
            };
            view! {
                {line_chart("Points Over Time", series(|p| p.point_history.clone()))}
                {line_chart("Phases Over Time", series(|p| p.phase_history.clone()))}
            }
        })
    };

    view! {
        <div class="table-scroll">
            <table class="tool-table">
                <thead>
                    <tr>
                        <th scope="col">"Name"</th>
                        <th scope="col">"Phase"</th>
                        <th scope="col">"Points"</th>
                        <th scope="col">"Add"</th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                    <Show when=move || adding.get()>
                        <tr>
                            <th>
                                <input
                                    class="tool-input tool-input-wide"
                                    type="text"
                                    aria-label="New player name"
                                    prop:value=move || new_name.get()
                                    on:input=move |ev| new_name.set(event_target_value(&ev))
                                />
                            </th>
                            <td colspan="3">
                                <button type="button" class="btn btn-primary" on:click=finish_adding>"Add Name"</button>
                            </td>
                        </tr>
                    </Show>
                </tbody>
            </table>
        </div>
        <Show when=move || !adding.get()>
            <div class="button-row">
                <button type="button" class="btn btn-primary" on:click=move |_| adding.set(true)>"Add User"</button>
                <button type="button" class="btn btn-secondary" on:click=complete_round>"Complete Round"</button>
            </div>
        </Show>
        <div class="charts">{charts}</div>
        <div class="button-row">
            <button type="button" class="btn btn-danger" on:click=restart>"Restart"</button>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{digits_only, Player};

    fn player() -> Player {
        Player {
            name: "A".into(),
            point_history: vec![],
            phase_history: vec![],
            color: "red".into(),
        }
    }

    #[test]
    fn under_fifty_points_advances_the_phase() {
        let mut p = player();
        assert_eq!(p.phase(), 1);
        p.record_round(49);
        assert_eq!((p.phase(), p.total()), (2, 49));
        p.record_round(50);
        assert_eq!((p.phase(), p.total()), (2, 99));
    }

    #[test]
    fn games_saved_by_the_old_page_still_load() {
        let saved = r#"[{"name":"Sam","nameID":"SamName","phaseID":"SamPhase","pointsID":"SamPoints",
            "addPointsID":"SamAddPoints","pointHistory":[10,60],"phaseHistory":[2,2],"color":"rgb(1, 2, 3)"}]"#;
        let players: Vec<Player> = serde_json::from_str(saved).unwrap();
        assert_eq!(players[0].total(), 70);
        assert_eq!(players[0].phase(), 2);
    }

    #[test]
    fn point_entry_keeps_only_digits() {
        assert_eq!(digits_only("1a2.3"), "123");
    }
}
