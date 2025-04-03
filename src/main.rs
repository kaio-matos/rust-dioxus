use std::time::Duration;

use async_std::task;
use dioxus::{ logger::tracing, prelude::*};
use structs::User;
use components::{Table, TableHead, CellContent};
mod structs;
mod components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");


fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let users_resource = use_resource(|| async move {
        task::sleep(Duration::from_millis(3000)).await; // add delay
        reqwest::get("https://jsonplaceholder.typicode.com/users")
            .await
            ?.json::<Vec<User>>()
            .await
    });

    rsx! {
        match &*users_resource.read_unchecked() {
            Some(Ok(response)) => rsx! {
                div {
                    h1 { "Users Table" }
                    UsersTable { users: response.clone() }
                    div {
                        img { src: "{HEADER_SVG}", alt: "Header Image" }
                    }
                }
            },
            Some(Err(_)) => rsx! {
                div { "Loading users failed" }
            },
            None => rsx! {
                div { "Loading users..." }
            },
        }
    }
}

#[component]
fn UsersTable(users: Vec<User>) -> Element {
    rsx! {
        Table {
            heads: vec![
                TableHead {
                    title: "Name".to_string(),
                    content: CellContent::Accessor(|user: User| user.name),
                },
                TableHead {
                    title: "SSN".to_string(),
                    content: CellContent::Accessor(|user| user.username),
                },
                TableHead {
                    title: "Email".to_string(),
                    content: CellContent::Accessor(|user| user.email),
                },
                TableHead {
                    title: "Address".to_string(),
                    content: CellContent::Renderer(|user| {
                        rsx! {
                            AddressCell { user }
                        }
                    }),
                },
                TableHead {
                    title: "Email".to_string(),
                    content: CellContent::Accessor(|user| user.phone),
                },
                TableHead {
                    title: "Company".to_string(),
                    content: CellContent::Renderer(|user| {
                        rsx! {
                            CompanyCell { user }
                        }
                    }),
                },
            ],
            get_key: |user| user.id.to_string(),
            data: users,
        }
    }
}



#[component]
fn AddressCell(user: User) -> Element {
    rsx! {
        b { {user.address.street} }
    }
}

#[component]
fn CompanyCell(user: User) -> Element {
    rsx! {
        b { {user.company.name} }
    }
}

