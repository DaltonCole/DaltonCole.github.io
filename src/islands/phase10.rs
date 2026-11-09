//! Phase 10 score keeper. Games are saved in the browser under the same `players` key the
//! old JavaScript used, so a game in progress carries over.

use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "players";
/// A player who scores fewer points than this has probably completed their phase, so the
/// round form ticks that box for them. It can always be changed by hand.
const PHASE_COMPLETE_BELOW: i64 = 50;
const COLORS: [&str; 8] = [
    "#f43f9e", "#8b5cf6", "#22d3ee", "#f5a524", "#4ade80", "#fb7185", "#60a5fa", "#facc15",
];
const PHASES: [&str; 10] = [
    "2 sets of 3",
    "1 set of 3 + 1 run of 4",
    "1 set of 4 + 1 run of 4",
    "1 run of 7",
    "1 run of 8",
    "1 run of 9",
    "2 sets of 4",
    "7 cards of one color",
    "1 set of 5 + 1 set of 2",
    "1 set of 5 + 1 set of 3",
];
/// What `Player::phase` reports once the last phase is complete.
const FINISHED: i64 = PHASES.len() as i64 + 1;

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

    fn phases_done(&self) -> i64 {
        (self.phase() - 1).clamp(0, PHASES.len() as i64)
    }

    fn goal(&self) -> &'static str {
        usize::try_from(self.phase() - 1)
            .ok()
            .and_then(|index| PHASES.get(index))
            .copied()
            .unwrap_or("Every phase complete")
    }

    fn record_round(&mut self, points: i64, made_phase: bool) {
        let phase = (self.phase() + i64::from(made_phase)).min(FINISHED);
        self.point_history.push(points);
        self.phase_history.push(phase);
    }

    fn running_totals(&self) -> Vec<i64> {
        self.point_history
            .iter()
            .scan(0, |total, points| {
                *total += points;
                Some(*total)
            })
            .collect()
    }
}

fn rounds_played(players: &[Player]) -> usize {
    players
        .iter()
        .map(|player| player.point_history.len())
        .max()
        .unwrap_or(0)
}

/// Takes back the latest round. A player who joined late or skipped it keeps their history.
fn undo_round(players: &mut [Player]) {
    let rounds = rounds_played(players);
    if rounds == 0 {
        return;
    }
    for player in players
        .iter_mut()
        .filter(|player| player.point_history.len() == rounds)
    {
        player.point_history.pop();
        player.phase_history.pop();
    }
}

/// The first player to finish every phase wins, and ties go to the lowest score.
fn winner(players: &[Player]) -> Option<&Player> {
    players
        .iter()
        .filter(|player| player.phase() >= FINISHED)
        .min_by_key(|player| player.total())
}

fn made_phase(points: &str, choice: Option<bool>) -> bool {
    choice.unwrap_or_else(|| {
        points
            .parse::<i64>()
            .is_ok_and(|points| points < PHASE_COMPLETE_BELOW)
    })
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

fn confirmed(message: &str) -> bool {
    window().confirm_with_message(message).unwrap_or(false)
}

/// One player's unsaved entries for the current round. Reference-counted, so they outlive
/// the event handler or list render that created them.
#[derive(Clone)]
struct RoundEntry {
    points: ArcRwSignal<String>,
    /// `None` until the box is changed by hand, so that it follows the points typed.
    choice: ArcRwSignal<Option<bool>>,
}

impl RoundEntry {
    fn new() -> Self {
        Self {
            points: ArcRwSignal::new(String::new()),
            choice: ArcRwSignal::new(None),
        }
    }

    fn score(&self) -> Option<(i64, bool)> {
        let text = self.points.get_untracked();
        let points = text.parse::<i64>().ok()?;
        Some((points, made_phase(&text, self.choice.get_untracked())))
    }

    fn clear(&self) {
        self.points.set(String::new());
        self.choice.set(None);
    }
}

type Series = (String, String, Vec<i64>);

fn line_chart(title: &'static str, series: Vec<Series>) -> impl IntoView {
    const WIDTH: f64 = 640.0;
    const HEIGHT: f64 = 260.0;
    const PAD: f64 = 44.0;

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

    let mut ticks = vec![0, top / 2, top];
    ticks.dedup();
    let grid = ticks
        .into_iter()
        .map(|tick| {
            let class = if tick == 0 {
                "chart-axis"
            } else {
                "chart-grid"
            };
            view! {
                <line x1=PAD y1=y(tick) x2=WIDTH - PAD y2=y(tick) class=class></line>
                <text x=PAD - 8.0 y=y(tick) + 4.0 text-anchor="end" class="chart-label">
                    {tick.to_string()}
                </text>
            }
        })
        .collect_view();

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
                <text x=x(round) y=HEIGHT - 16.0 text-anchor="middle" class="chart-label">
                    {(round + 1).to_string()}
                </text>
            }
        })
        .collect_view();

    view! {
        <figure class="chart">
            <figcaption>{title}</figcaption>
            <svg viewBox=format!("0 0 {WIDTH} {HEIGHT}") role="img" aria-label=title>
                {grid}
                {round_labels}
                {lines}
            </svg>
            <div class="chart-legend">{legend}</div>
        </figure>
    }
}

fn round_fields(name: &str, entry: RoundEntry) -> impl IntoView {
    let RoundEntry { points, choice } = entry;
    let points_writer = points.clone();
    let points_for_box = points.clone();
    let choice_writer = choice.clone();
    view! {
        <label class="player-field">
            <span class="stat-label">"This round"</span>
            <input
                class="tool-input"
                type="text"
                inputmode="numeric"
                maxlength="3"
                aria-label=format!("Round points for {name}")
                prop:value=move || points.get()
                on:input=move |ev| points_writer.set(digits_only(&event_target_value(&ev)))
            />
        </label>
        <label class="player-check">
            <input
                type="checkbox"
                aria-label=format!("{name} made their phase")
                prop:checked=move || made_phase(&points_for_box.get(), choice.get())
                on:change=move |ev| choice_writer.set(Some(event_target_checked(&ev)))
            />
            "Made phase"
        </label>
    }
}

#[island]
pub fn PhaseTenScoreKeeper() -> impl IntoView {
    let players = RwSignal::new(Vec::<Player>::new());
    let entries = RwSignal::new(Vec::<RoundEntry>::new());
    let new_name = RwSignal::new(String::new());
    let missing_points = RwSignal::new(false);

    let has_players = move || players.with(|list| !list.is_empty());
    let has_rounds = move || players.with(|list| rounds_played(list) > 0);
    let game_over = move || players.with(|list| winner(list).is_some());
    let save = move || players.with_untracked(|list| save_players(list));

    // Runs once in the browser; the prerendered page starts with no players.
    Effect::new(move |_| {
        let saved = load_players();
        entries.set(saved.iter().map(|_| RoundEntry::new()).collect());
        players.set(saved);
    });

    let add_player = move |ev: SubmitEvent| {
        ev.prevent_default();
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
        entries.update(|list| list.push(RoundEntry::new()));
        new_name.set(String::new());
        save();
    };

    let remove_player = move |index: usize, name: &str| {
        if confirmed(&format!("Remove {name} from this game?")) {
            players.update(|list| {
                list.remove(index);
            });
            entries.update(|list| {
                list.remove(index);
            });
            save();
        }
    };

    let complete_round = move |_| {
        let current = entries.get_untracked();
        let Some(scores) = current
            .iter()
            .map(RoundEntry::score)
            .collect::<Option<Vec<_>>>()
        else {
            missing_points.set(true);
            return;
        };
        missing_points.set(false);
        players.update(|list| {
            for (player, (points, made)) in list.iter_mut().zip(scores) {
                player.record_round(points, made);
            }
        });
        current.iter().for_each(RoundEntry::clear);
        save();
    };

    let undo = move |_| {
        players.update(|list| undo_round(list));
        save();
    };

    let restart = move |_| {
        if confirmed("Clear every player and score, and start over?") {
            players.set(Vec::new());
            entries.set(Vec::new());
            missing_points.set(false);
            save();
        }
    };

    let player_cards = move || {
        let accepting_scores = !game_over();
        players
            .get()
            .into_iter()
            .zip(entries.get())
            .enumerate()
            .map(|(index, (player, entry))| {
                let name = player.name.clone();
                let done = player.phases_done();
                let phase_label = if player.phase() >= FINISHED {
                    "Finished".to_string()
                } else {
                    format!("Phase {} of {}", player.phase(), PHASES.len())
                };
                let fields = accepting_scores.then(|| round_fields(&name, entry));
                let remove_name = name.clone();
                view! {
                    <li class="player" style=format!("--player-color: {}", player.color)>
                        <div class="player-head">
                            <span class="player-name">{name.clone()}</span>
                            <button
                                type="button"
                                class="icon-btn"
                                aria-label=format!("Remove {name}")
                                title="Remove player"
                                on:click=move |_| remove_player(index, &remove_name)
                            >
                                "×"
                            </button>
                        </div>
                        <div class="player-phase">
                            <strong>{phase_label}</strong>
                            <span>{player.goal()}</span>
                        </div>
                        <div class="phase-bar" aria-hidden="true">
                            <span style=format!("width: {}%", done * 10)></span>
                        </div>
                        <div class="player-round">
                            <div class="player-field">
                                <span class="stat-label">"Total"</span>
                                <span class="stat-value">{player.total()}</span>
                            </div>
                            {fields}
                        </div>
                    </li>
                }
            })
            .collect_view()
    };

    let winner_banner = move || {
        players.with(|list| {
            winner(list).map(|player| {
                let text = format!("{} wins with {} points!", player.name, player.total());
                view! { <p class="phase-winner">{text}</p> }
            })
        })
    };

    let charts = move || {
        let list = players.get();
        (rounds_played(&list) > 0).then(|| {
            let series = |pick: fn(&Player) -> Vec<i64>| {
                list.iter()
                    .map(|p| (p.name.clone(), p.color.clone(), pick(p)))
                    .collect::<Vec<Series>>()
            };
            view! {
                {line_chart("Total points by round", series(Player::running_totals))}
                {line_chart("Phase by round", series(|p| p.phase_history.clone()))}
            }
        })
    };

    let round_number = move || players.with(|list| rounds_played(list) + 1);

    view! {
        {winner_banner}
        <h2 class="round-title" hidden=move || game_over() || !has_players()>"Round " {round_number}</h2>
        <ul class="player-grid">{player_cards}</ul>
        <p class="tool-empty" hidden=has_players>"No players yet. Add the first one below."</p>
        <p class="tool-hint" role="alert" hidden=move || !missing_points.get()>
            "Enter points for every player first. Use 0 for anyone who went out."
        </p>
        <div class="button-row" hidden=move || !has_players()>
            <button
                type="button"
                class="btn btn-primary"
                hidden=game_over
                on:click=complete_round
            >
                "Finish round"
            </button>
            <button
                type="button"
                class="btn btn-secondary"
                prop:disabled=move || !has_rounds()
                on:click=undo
            >
                "Undo last round"
            </button>
        </div>
        <form class="add-player" on:submit=add_player>
            <input
                class="tool-input tool-input-wide"
                type="text"
                maxlength="30"
                placeholder="Player name"
                aria-label="New player name"
                prop:value=move || new_name.get()
                on:input=move |ev| new_name.set(event_target_value(&ev))
            />
            <button type="submit" class="btn btn-secondary">"Add player"</button>
        </form>
        <div class="charts">{charts}</div>
        <div class="button-row">
            <button type="button" class="btn btn-danger" on:click=restart>"Restart game"</button>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::{digits_only, made_phase, rounds_played, undo_round, winner, Player, FINISHED};

    fn player(name: &str) -> Player {
        Player {
            name: name.into(),
            point_history: vec![],
            phase_history: vec![],
            color: "red".into(),
        }
    }

    #[test]
    fn a_made_phase_advances_the_player() {
        let mut p = player("A");
        assert_eq!(p.phase(), 1);
        p.record_round(49, true);
        assert_eq!((p.phase(), p.total()), (2, 49));
        p.record_round(50, false);
        assert_eq!((p.phase(), p.total()), (2, 99));
    }

    #[test]
    fn the_box_follows_the_points_until_changed_by_hand() {
        assert!(made_phase("49", None));
        assert!(!made_phase("50", None));
        assert!(!made_phase("", None));
        assert!(made_phase("75", Some(true)));
        assert!(!made_phase("10", Some(false)));
    }

    #[test]
    fn phases_stop_at_the_last_one() {
        let mut p = player("A");
        for _ in 0..12 {
            p.record_round(0, true);
        }
        assert_eq!(p.phase(), FINISHED);
        assert_eq!(p.phases_done(), 10);
        assert_eq!(p.goal(), "Every phase complete");
    }

    #[test]
    fn each_phase_shows_its_goal() {
        let mut p = player("A");
        assert_eq!(p.goal(), "2 sets of 3");
        p.record_round(0, true);
        assert_eq!(p.goal(), "1 set of 3 + 1 run of 4");
    }

    #[test]
    fn the_lowest_score_among_finishers_wins() {
        let mut high = player("High");
        let mut low = player("Low");
        let mut behind = player("Behind");
        for _ in 0..10 {
            high.record_round(20, true);
            low.record_round(5, true);
            behind.record_round(0, false);
        }
        let players = [high, behind, low];
        assert_eq!(winner(&players).map(|p| p.name.as_str()), Some("Low"));
    }

    #[test]
    fn nobody_wins_before_finishing() {
        let mut p = player("A");
        p.record_round(0, true);
        assert!(winner(&[p]).is_none());
    }

    #[test]
    fn undo_takes_back_only_the_latest_round() {
        let mut veteran = player("Veteran");
        veteran.record_round(10, true);
        veteran.record_round(20, false);
        let mut late = player("Late");
        late.record_round(30, true);
        let mut players = [veteran, late];

        undo_round(&mut players);

        assert_eq!(players[0].point_history, vec![10]);
        assert_eq!(players[1].point_history, vec![30]);
        assert_eq!(rounds_played(&players), 1);
    }

    #[test]
    fn undo_on_a_new_game_does_nothing() {
        let mut players = [player("A")];
        undo_round(&mut players);
        assert_eq!(rounds_played(&players), 0);
    }

    #[test]
    fn running_totals_accumulate() {
        let mut p = player("A");
        p.record_round(10, true);
        p.record_round(25, false);
        assert_eq!(p.running_totals(), vec![10, 35]);
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
