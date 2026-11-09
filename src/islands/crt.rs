//! Chinese Remainder Theorem solver that shows its work.

use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Solution {
    remainders: Vec<i128>,
    moduli: Vec<i128>,
    product: i128,
    /// N_i: the product of every modulus except the i-th.
    others: Vec<i128>,
    /// N_i mod m_i, the value that gets inverted.
    reduced: Vec<i128>,
    /// x_i: the inverse of N_i modulo m_i.
    inverses: Vec<i128>,
    terms: Vec<i128>,
    total: i128,
    answer: i128,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SolveError {
    NotCoprime,
    TooLarge,
}

impl SolveError {
    fn message(self) -> &'static str {
        match self {
            Self::NotCoprime => "NOTE: ALL MODS MUST BE CO-PRIME",
            Self::TooLarge => "Those numbers are too large to solve.",
        }
    }
}

/// Modular inverse of `value` mod `modulus`, if one exists.
fn inverse(value: i128, modulus: i128) -> Option<i128> {
    let (mut r0, mut r1) = (modulus, value.rem_euclid(modulus));
    let (mut t0, mut t1) = (0_i128, 1_i128);
    while r1 != 0 {
        let quotient = r0 / r1;
        (r0, r1) = (r1, r0 - quotient * r1);
        (t0, t1) = (t1, t0 - quotient * t1);
    }
    (r0 == 1).then(|| t0.rem_euclid(modulus))
}

fn solve(equations: &[(i128, i128)]) -> Result<Solution, SolveError> {
    let remainders: Vec<i128> = equations.iter().map(|(b, _)| *b).collect();
    let moduli: Vec<i128> = equations.iter().map(|(_, m)| *m).collect();
    let product = moduli
        .iter()
        .try_fold(1_i128, |acc, m| acc.checked_mul(*m))
        .ok_or(SolveError::TooLarge)?;

    let others: Vec<i128> = moduli.iter().map(|m| product / m).collect();
    let reduced: Vec<i128> = others.iter().zip(&moduli).map(|(n, m)| n % m).collect();
    let inverses = reduced
        .iter()
        .zip(&moduli)
        .map(|(value, m)| inverse(*value, *m).ok_or(SolveError::NotCoprime))
        .collect::<Result<Vec<i128>, SolveError>>()?;

    let terms = remainders
        .iter()
        .zip(&others)
        .zip(&inverses)
        .map(|((b, n), x)| b.checked_mul(*n)?.checked_mul(*x))
        .collect::<Option<Vec<i128>>>()
        .ok_or(SolveError::TooLarge)?;
    let total = terms
        .iter()
        .try_fold(0_i128, |acc, term| acc.checked_add(*term))
        .ok_or(SolveError::TooLarge)?;
    let answer = total.rem_euclid(product);

    Ok(Solution {
        remainders,
        moduli,
        product,
        others,
        reduced,
        inverses,
        terms,
        total,
        answer,
    })
}

fn cells(values: Vec<String>) -> impl IntoView {
    values
        .into_iter()
        .map(|value| view! { <td>{value}</td> })
        .collect_view()
}

fn body_rows(rows: Vec<Vec<String>>) -> impl IntoView {
    rows.into_iter()
        .map(|row| view! { <tr>{cells(row)}</tr> })
        .collect_view()
}

fn total_row(total: i128) -> impl IntoView {
    view! { <tr><td></td><td></td><td>"Total: "</td><td>{total.to_string()}</td></tr> }
}

fn header_b() -> impl IntoView {
    view! { <th>"b"<sub>"i"</sub></th> }
}

fn header_n() -> impl IntoView {
    view! { <th>"N"<sub>"i"</sub></th> }
}

fn header_x() -> impl IntoView {
    view! { <th>"x"<sub>"i"</sub></th> }
}

fn header_term() -> impl IntoView {
    view! { <th>"b"<sub>"i"</sub>"N"<sub>"i"</sub>"x"<sub>"i"</sub></th> }
}

fn inverse_work(
    number: usize,
    n: i128,
    modulus: i128,
    reduced: i128,
    inverse: i128,
) -> impl IntoView {
    let label = number.to_string();
    let x = move || view! { "x"<sub>{label.clone()}</sub> };
    view! {
        <div class="crt-work">
            <p>{x()}":"</p>
            <p>{n.to_string()}{x()}" ≡ 1 (mod "{modulus.to_string()}") ⇒"</p>
            <p>{format!("{n} mod {modulus} = {reduced} ⇒")}</p>
            <p>{reduced.to_string()}{x()}" ≡ 1 (mod "{modulus.to_string()}") ⇒"</p>
            <p>"("{reduced.to_string()}" * "<b>{inverse.to_string()}</b>") mod "{modulus.to_string()}" = 1 ⇒"</p>
            <p>{x()}" ≡ "<b>{inverse.to_string()}</b>" (mod "{modulus.to_string()}")"</p>
        </div>
    }
}

fn worked_solution(solution: Solution) -> impl IntoView {
    let Solution {
        remainders,
        moduli,
        product,
        others,
        reduced,
        inverses,
        terms,
        total,
        answer,
    } = solution;
    let row_count = remainders.len();
    let text = |values: &[i128], index: usize| values[index].to_string();

    let term_rows: Vec<Vec<String>> = (0..row_count)
        .map(|i| {
            vec![
                text(&remainders, i),
                text(&others, i),
                text(&inverses, i),
                text(&terms, i),
            ]
        })
        .collect();
    let remainder_rows: Vec<Vec<String>> =
        (0..row_count).map(|i| vec![text(&remainders, i)]).collect();
    let all_moduli = moduli
        .iter()
        .map(i128::to_string)
        .collect::<Vec<_>>()
        .join(" * ");
    let others_rows: Vec<Vec<String>> = (0..row_count)
        .map(|i| {
            vec![
                text(&remainders, i),
                format!("{all_moduli} / {} = {}", moduli[i], others[i]),
            ]
        })
        .collect();
    let inverse_rows: Vec<Vec<String>> = (0..row_count)
        .map(|i| vec![text(&remainders, i), text(&others, i), text(&inverses, i)])
        .collect();
    let work = (0..row_count)
        .map(|i| inverse_work(i + 1, others[i], moduli[i], reduced[i], inverses[i]))
        .collect_view();

    view! {
        <div class="table-scroll">
            <table class="tool-table">
                <thead><tr>{header_b()}{header_n()}{header_x()}{header_term()}</tr></thead>
                <tbody>{body_rows(term_rows.clone())}{total_row(total)}</tbody>
            </table>
        </div>
        <p class="crt-answer"><b>{format!("Final Answer: x ≡ {answer} (mod {product})")}</b></p>

        <h3>"Step 1"</h3>
        <p>"Create b"<sub>"i"</sub>" column from remainders:"</p>
        <table class="tool-table">
            <thead><tr>{header_b()}</tr></thead>
            <tbody>{body_rows(remainder_rows)}</tbody>
        </table>

        <h3>"Step 2"</h3>
        <p>"Create N"<sub>"i"</sub>" column by multiplying all moduli together except the i"<sup>"th"</sup>" row:"</p>
        <table class="tool-table">
            <thead><tr>{header_b()}{header_n()}</tr></thead>
            <tbody>{body_rows(others_rows)}</tbody>
        </table>

        <h3>"Step 3"</h3>
        <p>"Create x"<sub>"i"</sub>" column by finding the inverse of N"<sub>"i"</sub>":"</p>
        {work}
        <table class="tool-table">
            <thead><tr>{header_b()}{header_n()}{header_x()}</tr></thead>
            <tbody>{body_rows(inverse_rows)}</tbody>
        </table>

        <h3>"Step 4"</h3>
        <p>"Multiply b"<sub>"i"</sub>", N"<sub>"i"</sub>", and x"<sub>"i"</sub>" together:"</p>
        <table class="tool-table">
            <thead><tr>{header_b()}{header_n()}{header_x()}{header_term()}</tr></thead>
            <tbody>{body_rows(term_rows.clone())}</tbody>
        </table>

        <h3>"Step 5"</h3>
        <p>"Add all elements of the b"<sub>"i"</sub>"N"<sub>"i"</sub>"x"<sub>"i"</sub>" column together:"</p>
        <table class="tool-table">
            <thead><tr>{header_b()}{header_n()}{header_x()}{header_term()}</tr></thead>
            <tbody>{body_rows(term_rows)}{total_row(total)}</tbody>
        </table>

        <h3>"Step 6"</h3>
        <p>"Mod total by N:"</p>
        <table class="tool-table">
            <tbody>
                <tr><td>{format!("N = {all_moduli} ⇒")}</td></tr>
                <tr><td>{format!("x ≡ {total} (mod {product}) ⇒")}</td></tr>
                <tr><td>{format!("x ≡ {answer} (mod {product})")}</td></tr>
            </tbody>
        </table>
    }
}

type Equation = (ArcRwSignal<String>, ArcRwSignal<String>);

fn digits_only(value: &str) -> String {
    value.chars().filter(char::is_ascii_digit).collect()
}

fn parse_equations(equations: &[Equation]) -> Option<Vec<(i128, i128)>> {
    equations
        .iter()
        .map(|(remainder, modulus)| {
            let b = remainder.get_untracked().parse::<i128>().ok()?;
            let m = modulus.get_untracked().parse::<i128>().ok()?;
            (m > 0).then_some((b, m))
        })
        .collect()
}

fn number_input(value: ArcRwSignal<String>, label: &'static str) -> impl IntoView {
    let writer = value.clone();
    view! {
        <input
            class="tool-input"
            type="text"
            inputmode="numeric"
            aria-label=label
            prop:value=move || value.get()
            on:input=move |ev| writer.set(digits_only(&event_target_value(&ev)))
        />
    }
}

#[derive(Clone)]
enum Outcome {
    Incomplete,
    Failed(SolveError),
    Solved(Solution),
}

#[island]
pub fn ChineseRemainderSolver() -> impl IntoView {
    let new_equation = || -> Equation {
        (
            ArcRwSignal::new(String::new()),
            ArcRwSignal::new(String::new()),
        )
    };
    let equations = RwSignal::new(vec![new_equation(), new_equation()]);
    let outcome = RwSignal::new(None::<Outcome>);

    let calculate = move |_| {
        let result = match parse_equations(&equations.get_untracked()) {
            None => Outcome::Incomplete,
            Some(parsed) => match solve(&parsed) {
                Ok(solution) => Outcome::Solved(solution),
                Err(error) => Outcome::Failed(error),
            },
        };
        outcome.set(Some(result));
    };

    let inputs = move || {
        equations
            .get()
            .into_iter()
            .map(|(remainder, modulus)| {
                view! {
                    <p class="crt-equation">
                        "x ≡ "{number_input(remainder, "Remainder")}" (mod "{number_input(modulus, "Modulus")}")"
                    </p>
                }
            })
            .collect_view()
    };

    let result = move || {
        outcome.get().map(|result| match result {
            Outcome::Solved(solution) => worked_solution(solution).into_any(),
            Outcome::Failed(error) => view! { <p class="crt-answer"><b>{error.message()}</b></p> }.into_any(),
            Outcome::Incomplete => {
                view! { <p class="crt-answer"><b>"Enter a whole number in every box, and moduli above zero."</b></p> }
                    .into_any()
            }
        })
    };

    view! {
        <div class="crt-inputs">{inputs}</div>
        <div class="button-row">
            <button type="button" class="btn btn-secondary" on:click=move |_| equations.update(|list| list.push(new_equation()))>
                "Add Equation"
            </button>
            <button type="button" class="btn btn-primary" on:click=calculate>"Calculate"</button>
        </div>
        <div class="crt-result">{result}</div>
    }
}

#[cfg(test)]
mod tests {
    use super::{inverse, solve, SolveError};

    #[test]
    fn classic_three_equation_example() {
        // x = 2 mod 3, 3 mod 5, 2 mod 7 has the least solution 23. The old page crashed here.
        let solution = solve(&[(2, 3), (3, 5), (2, 7)]).unwrap();
        assert_eq!((solution.answer, solution.product), (23, 105));
        assert_eq!(solution.others, vec![35, 21, 15]);
        assert_eq!(solution.inverses, vec![2, 1, 1]);
        assert_eq!(solution.total, 233);
    }

    #[test]
    fn two_equations() {
        let solution = solve(&[(1, 3), (4, 5)]).unwrap();
        assert_eq!(solution.answer, 4);
    }

    #[test]
    fn shared_factors_are_reported() {
        assert_eq!(solve(&[(1, 4), (3, 6)]), Err(SolveError::NotCoprime));
    }

    #[test]
    fn inverse_matches_definition() {
        assert_eq!(inverse(3, 7), Some(5));
        assert_eq!(inverse(2, 4), None);
    }

    #[test]
    fn huge_moduli_do_not_overflow() {
        let big = 170_141_183_460_469_231_731_687_303_715_884_105_727_i128;
        assert_eq!(
            solve(&[(1, big), (1, 3), (1, 5)]),
            Err(SolveError::TooLarge)
        );
    }
}
