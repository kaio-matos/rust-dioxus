use dioxus::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub enum CellContent<T> {
    Accessor(fn(T) -> String),
    Renderer(fn(T) -> Element),
}

#[derive(PartialEq, Clone, Debug)]
pub struct TableHead<T> {
    pub title: String,
    pub content: CellContent<T>
}

#[derive(Props, PartialEq, Clone, Debug)]
pub struct TableProps<T> where T: PartialEq + 'static {
    pub heads: Vec<TableHead<T>>,
    pub get_key: fn(T)-> String,
    pub data: Vec<T>
}

#[component]
pub fn Table<T: Clone + PartialEq + 'static>(props: TableProps<T>) -> Element {
    rsx! {
        table {
            thead {
                tr {
                    for head in props.heads.iter() {
                        th { {head.title.clone()} }
                    }
                }
            }
            tbody {
                for data in props.data.iter() {
                    tr { key: "{(props.get_key)(data.clone())}",
                        for head in props.heads.iter() {
                            match &head.content {
                                CellContent::Accessor(access) => rsx! {
                                    td { {access(data.clone())} }
                                },
                                CellContent::Renderer(render) => rsx! {
                                    td { {render(data.clone())} }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
