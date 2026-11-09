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
    /// Two moduli and the factor they share.
    NotCoprime(i128, i128, i128),
    TooLarge,
}

impl SolveError {
    fn message(self) -> String {
        match self {
            Self::NotCoprime(first, second, shared) => format!(
                "This system can't be solved this way: {first} and {second} share a factor of \
                 {shared}. Every modulus must be coprime with the others."
            ),
            Self::TooLarge => "Those numbers are too large to solve.".to_string(),
        }
    }
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn shared_factor(moduli: &[i128]) -> Option<SolveError> {
    moduli.iter().enumerate().find_map(|(index, first)| {
        moduli[index + 1..].iter().find_map(|second| {
            let shared = gcd(*first, *second);
            (shared > 1).then_some(SolveError::NotCoprime(*first, *second, shared))
        })
    })
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
    if let Some(error) = shared_factor(&moduli) {
        return Err(error);
    }
    let product = moduli
        .iter()
        .try_fold(1_i128, |acc, m| acc.checked_mul(*m))
        .ok_or(SolveError::TooLarge)?;

    let others: Vec<i128> = moduli.iter().map(|m| product / m).collect();
    let reduced: Vec<i128> = others.iter().zip(&moduli).map(|(n, m)| n % m).collect();
    let inverses: Vec<i128> = reduced
        .iter()
        .zip(&moduli)
        .map(|(value, m)| inverse(*value, *m).expect("coprime moduli always have inverses"))
        .collect();

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

fn body_rows(rows: Vec<Vec<String>>) -> impl IntoView {
    rows.into_iter()
        .map(|row| {
            let cells = row
                .into_iter()
                .map(|value| view! { <td>{value}</td> })
                .collect_view();
            view! { <tr>{cells}</tr> }
        })
        .collect_view()
}

fn subscripted(symbol: &'static str, index: impl ToString) -> impl IntoView {
    view! { {symbol}<sub>{index.to_string()}</sub> }
}

fn inverse_work(
    number: usize,
    n: i128,
    modulus: i128,
    reduced: i128,
    inverse: i128,
) -> impl IntoView {
    view! {
        <li>
            {subscripted("x", number)}": "
            {format!("{n} mod {modulus} = {reduced}, and ({reduced} × ")}
            <b>{inverse.to_string()}</b>
            {format!(") mod {modulus} = {}, so ", 1 % modulus)}
            {subscripted("x", number)}" = "<b>{inverse.to_string()}</b>
        </li>
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
    let count = remainders.len();
    let text = |values: &[i128], index: usize| values[index].to_string();
    let all_moduli = moduli
        .iter()
        .map(i128::to_string)
        .collect::<Vec<_>>()
        .join(" × ");

    let checks = (0..count)
        .map(|i| {
            let shown = format!(
                "{answer} mod {} = {}",
                moduli[i],
                remainders[i].rem_euclid(moduli[i])
            );
            view! { <li class="pill">{shown}</li> }
        })
        .collect_view();
    let others_rows: Vec<Vec<String>> = (0..count)
        .map(|i| {
            vec![
                (i + 1).to_string(),
                text(&moduli, i),
                format!("{product} / {} = {}", moduli[i], others[i]),
            ]
        })
        .collect();
    let work = (0..count)
        .map(|i| inverse_work(i + 1, others[i], moduli[i], reduced[i], inverses[i]))
        .collect_view();
    let term_rows: Vec<Vec<String>> = (0..count)
        .map(|i| {
            vec![
                text(&remainders, i),
                text(&others, i),
                text(&inverses, i),
                text(&terms, i),
            ]
        })
        .collect();

    view! {
        <div class="crt-answer-card">
            <div class="stat-label">"Answer"</div>
            <p class="crt-answer">{format!("x ≡ {answer} (mod {product})")}</p>
            <p>{format!("Every solution has the form {answer} + {product}k.")}</p>
            <ul class="pill-list" aria-label="Checks">{checks}</ul>
        </div>

        <h2>"How it's solved"</h2>
        <ol class="crt-steps">
            <li>
                <h3>"Multiply the moduli"</h3>
                <p>{format!("N = {all_moduli} = {product}")}</p>
            </li>
            <li>
                <h3>"Divide N by each modulus"</h3>
                <p>"N"<sub>"i"</sub>" is the product of every modulus except m"<sub>"i"</sub>"."</p>
                <div class="table-scroll">
                    <table class="tool-table">
                        <thead><tr><th>"i"</th><th>"m"<sub>"i"</sub></th><th>"N"<sub>"i"</sub></th></tr></thead>
                        <tbody>{body_rows(others_rows)}</tbody>
                    </table>
                </div>
            </li>
            <li>
                <h3>"Invert each N"<sub>"i"</sub></h3>
                <p>"x"<sub>"i"</sub>" is the number that makes N"<sub>"i"</sub>"x"<sub>"i"</sub>" ≡ 1 (mod m"<sub>"i"</sub>")."</p>
                <ul class="crt-work">{work}</ul>
            </li>
            <li>
                <h3>"Combine"</h3>
                <p>"Multiply b"<sub>"i"</sub>", the remainder, by N"<sub>"i"</sub>" and x"<sub>"i"</sub>", then add the results."</p>
                <div class="table-scroll">
                    <table class="tool-table">
                        <thead>
                            <tr>
                                <th>"b"<sub>"i"</sub></th>
                                <th>"N"<sub>"i"</sub></th>
                                <th>"x"<sub>"i"</sub></th>
                                <th>"b"<sub>"i"</sub>"N"<sub>"i"</sub>"x"<sub>"i"</sub></th>
                            </tr>
                        </thead>
                        <tbody>
                            {body_rows(term_rows)}
                            <tr class="total-row">
                                <td colspan="3">"Total"</td>
                                <td>{total.to_string()}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </li>
            <li>
                <h3>"Reduce by N"</h3>
                <p>{format!("{total} mod {product} = {answer}")}</p>
            </li>
        </ol>
    }
}

type Equation = (ArcRwSignal<String>, ArcRwSignal<String>);

fn digits_only(value: &str) -> String {
    value.chars().filter(char::is_ascii_digit).collect()
}

fn equation(remainder: &str, modulus: &str) -> Equation {
    (
        ArcRwSignal::new(remainder.to_string()),
        ArcRwSignal::new(modulus.to_string()),
    )
}

fn parse_equations(equations: &[Equation]) -> Option<Vec<(i128, i128)>> {
    equations
        .iter()
        .map(|(remainder, modulus)| {
            let b = remainder.get().parse::<i128>().ok()?;
            let m = modulus.get().parse::<i128>().ok()?;
            (m > 0).then_some((b, m))
        })
        .collect()
}

fn number_input(value: ArcRwSignal<String>, label: String) -> impl IntoView {
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

#[island]
pub fn ChineseRemainderSolver() -> impl IntoView {
    const MINIMUM_EQUATIONS: usize = 2;

    let equations = RwSignal::new(vec![equation("", ""), equation("", "")]);

    let load_example = move |_| {
        equations.set(vec![
            equation("2", "3"),
            equation("3", "5"),
            equation("2", "7"),
        ]);
    };

    let inputs = move || {
        let list = equations.get();
        let removable = list.len() > MINIMUM_EQUATIONS;
        list.into_iter()
            .enumerate()
            .map(|(index, (remainder, modulus))| {
                let number = index + 1;
                let remove = removable.then(|| {
                    view! {
                        <button
                            type="button"
                            class="icon-btn"
                            aria-label=format!("Remove equation {number}")
                            title="Remove equation"
                            on:click=move |_| equations.update(|list| { list.remove(index); })
                        >
                            "×"
                        </button>
                    }
                });
                view! {
                    <div class="crt-equation">
                        <span>"x ≡"</span>
                        {number_input(remainder, format!("Remainder {number}"))}
                        <span>"(mod"</span>
                        {number_input(modulus, format!("Modulus {number}"))}
                        <span>")"</span>
                        {remove}
                    </div>
                }
            })
            .collect_view()
    };

    let result = move || {
        match parse_equations(&equations.get()).map(|parsed| solve(&parsed)) {
        None => view! { <p class="tool-empty">"Fill in every box to see the answer. Moduli must be at least 1."</p> }
            .into_any(),
        Some(Ok(solution)) => worked_solution(solution).into_any(),
        Some(Err(error)) => view! { <p class="tool-hint" role="alert">{error.message()}</p> }.into_any(),
    }
    };

    view! {
        <div class="crt-inputs">{inputs}</div>
        <div class="button-row">
            <button
                type="button"
                class="btn btn-secondary"
                on:click=move |_| equations.update(|list| list.push(equation("", "")))
            >
                "Add equation"
            </button>
            <button type="button" class="btn btn-secondary" on:click=load_example>"Try an example"</button>
        </div>
        <div class="crt-result" aria-live="polite">{result}</div>
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
        assert_eq!(
            solve(&[(1, 4), (3, 6)]),
            Err(SolveError::NotCoprime(4, 6, 2))
        );
    }

    #[test]
    fn the_error_names_the_moduli_at_fault() {
        let error = solve(&[(1, 5), (1, 9), (1, 12)]).unwrap_err();
        assert_eq!(error, SolveError::NotCoprime(9, 12, 3));
        assert!(error.message().contains("9 and 12 share a factor of 3"));
    }

    #[test]
    fn a_modulus_of_one_is_allowed() {
        assert_eq!(solve(&[(0, 1), (3, 5)]).unwrap().answer, 3);
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
