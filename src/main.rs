use axum::Extension;
use axum::response::Html;
use axum::routing::{get};
use maud::Markup;
use maud::html;
use serde::{Deserialize, Serialize};
use serde_qs::axum::{QsQuery, QsQueryConfig};
use std::cmp::Reverse;
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Swimmer {
    name: String,
    age: u32,
    skill: u32,
    duration: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Swimmers {
    swimmers: Vec<Swimmer>,
}

fn get_instruction(swimmer: &Swimmer) -> String {
    let instruction = match swimmer.skill {
        8 | 9 => {
            if swimmer.duration < 60 {
                "сам"
            } else {
                "до дна и обратно"
            }
        }
        7 => {
            if swimmer.duration < 55 {
                "до дна"
            } else {
                "до дна и обратно"
            }
        }
        6 => {
            if swimmer.duration < 55 {
                "до глубины и обратно"
            } else {
                "3 нырка, до глубины и обратно"
            }
        }
        5 => {
            if swimmer.duration < 55 {
                "3 нырка, до глубины и обратно"
            } else {
                "2 нырка, до глубины и обратно"
            }
        }
        _ => unreachable!(),
    };

    format!("{} ({})", swimmer.name, instruction)
}

fn make_distr(swimmers: Vec<(Swimmer, Option<Swimmer>, Option<Swimmer>)>) -> Markup {
    html! {
        @for (escort, first, second) in swimmers {
            (escort.name) ": "

            @if let Some(ref first) = first {
                (get_instruction(first))
            }

            @if let Some(ref second) = second {
                ", " (get_instruction(second))
            }

            br;
        }
    }
}

fn create_distribution(swimmers: &[Swimmer]) -> Result<Markup, Markup> {
    let mut escorts = Vec::<Swimmer>::new();
    let mut wards = Vec::<Swimmer>::new();

    for swimmer in swimmers {
        if swimmer.skill > 7 && swimmer.duration < 60 {
            escorts.push(swimmer.clone());
        } else {
            wards.push(swimmer.clone());
        }
    }

    if escorts.len() * 2 < wards.len() {
        return Err(html! {
            "Невозможно распределить: Найдите больше споровождюащих"
        });
    }

    if escorts.len() + wards.len() < 3 {
        return Err(html! {
            "Невозможно распределить: Это не заплыв, а помощь с нырком"
        });
    }

    escorts.sort_by_key(|s| Reverse((s.age, s.skill, s.duration)));
    wards.sort_by_key(|s| Reverse((s.age, s.skill, s.duration)));

    let mut escorts_counter = 0;

    let mut distr = Vec::<(Swimmer, Option<Swimmer>, Option<Swimmer>)>::new();

    for escort in &escorts {
        distr.push((escort.clone(), None, None));
    }

    let wards_amount = wards.len();
    let escorts_amount = escorts.len();

    if wards_amount != 0 {
        for ref mut ward_counter in 0..wards_amount {
            if escorts[escorts_counter].age > wards[*ward_counter].age {
                if distr[escorts_counter].1.is_none() {
                    distr[escorts_counter].1 = Some(wards[*ward_counter].clone());
                } else if distr[escorts_counter].2.is_none() {
                    distr[escorts_counter].2 = Some(wards[*ward_counter].clone());
                } else {
                    if escorts_counter == 0 {
                        return Err(html! {
                            (format!("Невозможно распределить! Найдите кого-то старше {:?} лун", wards[*ward_counter].age))
                        });
                    } else {
                        escorts_counter = escorts_counter - 1;
                        *ward_counter = *ward_counter - 1;
                        continue;
                    }
                }
            } else {
                if escorts_counter == 0 {
                    return Err(html! {
                        (format!("Невозможно распределить! Найдите кого-то старше {:?} лун", wards[*ward_counter].age))
                    });
                } else {
                    escorts_counter = escorts_counter - 1;
                    *ward_counter = *ward_counter - 1;
                    continue;
                }
            }

            escorts_counter = (escorts_counter + 1) % escorts_amount;
        }
    }

    let mut free_escorts = 0;

    for escort in &distr {
        if escort.1.is_none() {
            free_escorts += 1;
        }
    }

    if free_escorts == 1 {
        if distr[escorts_amount - 2].2.is_none() {
            distr[escorts_amount - 2].2 = Some(escorts[escorts_amount - 1].clone());

            distr.truncate(escorts_amount - 1);
            let beautiful_distribution = make_distr(distr);

            return Ok(beautiful_distribution);
        } else {
            return Err(html! {
                (format!("Невозможно распределелить: не хватает сопроводждающего/подопечного для игрока по имени {}", escorts[escorts_amount - 1].name))
            });
        }
    }

    let mut small_escorts_amount = free_escorts / 3;
    if free_escorts % 3 != 0 {
        small_escorts_amount += 1;
    }

    let escorts_to_wards_amount = free_escorts - small_escorts_amount;

    let new_escorts_amount = escorts_amount - escorts_to_wards_amount;
    let mut new_escorts_counter = escorts_amount - free_escorts;

    let start = escorts_amount - escorts_to_wards_amount;
    let end = escorts_amount - 1;

    for ref mut escorts_to_wards_counter in start..=end {
        if escorts[new_escorts_counter].age > escorts[*escorts_to_wards_counter].age {
            if distr[new_escorts_counter].1.is_none() {
                distr[new_escorts_counter].1 = Some(escorts[*escorts_to_wards_counter].clone());
            } else if distr[new_escorts_counter].2.is_none() {
                distr[new_escorts_counter].2 = Some(escorts[*escorts_to_wards_counter].clone());
            } else {
                if new_escorts_counter == escorts_amount - free_escorts {
                    return Err(html! {
                        (format!("Невозможно распределить! Найдите кого-то старше {} лун", escorts[*escorts_to_wards_counter].age))
                    });
                } else {
                    new_escorts_counter -= 1;
                    *escorts_to_wards_counter -= 1;
                    continue;
                }
            }
        } else {
            if new_escorts_counter == escorts_amount - free_escorts {
                return Err(html! {
                    (format!("Невозможно распределить! Найдите кого-то старше {} лун", escorts[*escorts_to_wards_counter].age))
                });
            } else {
                new_escorts_counter -= 1;
                *escorts_to_wards_counter -= 1;
                continue;
            }
        }

        new_escorts_counter = (new_escorts_counter + 1) % (new_escorts_amount);
        if new_escorts_counter == 0 {
            new_escorts_counter = escorts_amount - free_escorts;
        }
    }

    distr.truncate(new_escorts_amount);

    let beautiful_distribution = make_distr(distr);

    Ok(beautiful_distribution)
}

async fn index_handler() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

async fn submit_handler(QsQuery(swimmers): QsQuery<Swimmers>) -> Markup {
    match create_distribution(&swimmers.swimmers) {
        Ok(distribution) => html! {
            div id="response-message" class="success" { (distribution) }
        },

        Err(err) => html! {
            div id="response-message" class="error" { (err) }
        },
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    let router = axum::Router::new()
        .route("/", get(index_handler))
        .route("/submit", get(submit_handler))
        .nest_service("/js", ServeDir::new("./js"))
        .nest_service("/css", ServeDir::new("./css"))
        .layer(Extension(QsQueryConfig::new(5, false)))
        .into_make_service();

    axum::serve(listener, router).await?;
    Ok(())
}
