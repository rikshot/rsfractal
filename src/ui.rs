use seed::{prelude::*, *};

use super::wasm::{Model, Msg};

pub fn view(model: &Model) -> impl View<Msg> {
    div![
        class!["container"],
        h2![class!["title", "is-2"], "Controls"],
        div![
            class!["field", "is-horizontal"],
            div![class!["field-label", "is-normal"], label![class!["label"], "Size"]],
            div![
                class!["field-body"],
                div![
                    class!["field"],
                    p![
                        class!["control", "is-expanded"],
                        input![
                            class!["input"],
                            attrs! {
                                At::Type => "text",
                                At::Disabled => "disabled",
                                At::Value => model.config.width.to_string()
                            }
                        ]
                    ]
                ],
                div![
                    class!["field"],
                    p![
                        class!["control", "is-expanded"],
                        input![
                            class!["input"],
                            attrs! {
                                At::Type => "text",
                                At::Disabled => "disabled",
                                At::Value => model.config.height.to_string()
                            }
                        ]
                    ]
                ]
            ]
        ],
        div![
            class!["field", "is-horizontal"],
            div![class!["field-label", "is-normal"], label![class!["label"], "Position"]],
            div![
                class!["field-body"],
                div![
                    class!["field"],
                    p![
                        class!["control", "is-expanded"],
                        input![
                            class!["input"],
                            attrs! {
                                At::Type => "text",
                                At::Disabled => "disabled",
                                At::Value => model.config.position.x.to_string()
                            }
                        ]
                    ]
                ],
                div![
                    class!["field"],
                    p![
                        class!["control", "is-expanded"],
                        input![
                            class!["input"],
                            attrs! {
                                At::Type => "text",
                                At::Disabled => "disabled",
                                At::Value => model.config.position.y.to_string()
                            }
                        ]
                    ]
                ]
            ]
        ],
        div![
            class!["field", "is-horizontal"],
            div![class!["field-label", "is-normal"], label![class!["label"], "Zoom"]],
            div![
                class!["field-body"],
                div![
                    class!["field"],
                    p![
                        class!["control", "is-expanded"],
                        input![
                            class!["input"],
                            attrs! {
                                At::Type => "text",
                                At::Disabled => "disabled",
                                At::Value => model.config.zoom.x.to_string()
                            }
                        ]
                    ]
                ],
                div![
                    class!["field"],
                    p![
                        class!["control", "is-expanded"],
                        input![
                            class!["input"],
                            attrs! {
                                At::Type => "text",
                                At::Disabled => "disabled",
                                At::Value => model.config.zoom.y.to_string()
                            }
                        ]
                    ]
                ]
            ]
        ],
        div![
            class!["field", "is-horizontal"],
            div![
                class!["field-label", "is-normal"],
                label![class!["label"], "Iterations"]
            ],
            div![
                class!["field-body"],
                div![
                    class!["field"],
                    p![
                        class!["control", "is-expanded"],
                        input![
                            class!["input"],
                            attrs! {
                                At::Type => "text",
                                At::Value => model.config.iterations.to_string()
                            },
                            input_ev(Ev::Input, Msg::ChangeIterations)
                        ]
                    ]
                ]
            ]
        ],
        div![
            class!["field", "is-horizontal"],
            div![class!["field-label", "is-normal"], label![class!["label"], "Colors"]],
            div![
                class!["field-body"],
                div![
                    class!["field"],
                    p![
                        class!["control", "is-expanded"],
                        input![
                            class!["input"],
                            attrs! {
                                At::Type => "text",
                                At::Value => {
                                    let colors: Vec<String> = model.config.palette.iter().map(|color| color.to_hex()).collect();
                                    colors.join(",")
                                }
                            },
                            input_ev(Ev::Input, Msg::ChangeColors)
                        ]
                    ]
                ]
            ]
        ],
        div![
            class!["field", "is-horizontal"],
            div![class!["field-label"]],
            div![
                class!["field-body"],
                div![
                    class!["field", "is-grouped"],
                    div![
                        class!["control"],
                        button![
                            class!["button", "is-primary"],
                            attrs! {
                                At::Disabled => model.rendering.as_at_value()
                            },
                            simple_ev(Ev::Click, Msg::Render),
                            "Render"
                        ]
                    ],
                    div![
                        class!["control"],
                        button![
                            class!["button"],
                            attrs! { At::Disabled => model.rendering.as_at_value() },
                            simple_ev(Ev::Click, Msg::Reset),
                            "Reset"
                        ],
                    ],
                    div![
                        class!["control"],
                        button![class!["button"], simple_ev(Ev::Click, Msg::Export), "Export"],
                    ]
                ]
            ]
        ]
    ]
}
