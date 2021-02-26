use seed::{prelude::*, *};

use super::wasm::{Model, Msg};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["container"],
        h2![C!["title", "is-2"], "Controls"],
        div![
            C!["field", "is-horizontal"],
            div![C!["field-label", "is-normal"], label![C!["label"], "Size"]],
            div![
                C!["field-body"],
                div![
                    C!["field"],
                    p![
                        C!["control", "is-expanded"],
                        input![
                            C!["input"],
                            attrs! {
                                At::Type => "text",
                                At::Disabled => "disabled",
                                At::Value => model.config.width.to_string()
                            }
                        ]
                    ]
                ],
                div![
                    C!["field"],
                    p![
                        C!["control", "is-expanded"],
                        input![
                            C!["input"],
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
            C!["field", "is-horizontal"],
            div![C!["field-label", "is-normal"], label![C!["label"], "Position"]],
            div![
                C!["field-body"],
                div![
                    C!["field"],
                    p![
                        C!["control", "is-expanded"],
                        input![
                            C!["input"],
                            attrs! {
                                At::Type => "text",
                                At::Disabled => "disabled",
                                At::Value => model.config.position.x.to_string()
                            }
                        ]
                    ]
                ],
                div![
                    C!["field"],
                    p![
                        C!["control", "is-expanded"],
                        input![
                            C!["input"],
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
            C!["field", "is-horizontal"],
            div![C!["field-label", "is-normal"], label![C!["label"], "Zoom"]],
            div![
                C!["field-body"],
                div![
                    C!["field"],
                    p![
                        C!["control", "is-expanded"],
                        input![
                            C!["input"],
                            attrs! {
                                At::Type => "text",
                                At::Disabled => "disabled",
                                At::Value => model.config.zoom.x.to_string()
                            }
                        ]
                    ]
                ],
                div![
                    C!["field"],
                    p![
                        C!["control", "is-expanded"],
                        input![
                            C!["input"],
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
            C!["field", "is-horizontal"],
            div![
                C!["field-label", "is-normal"],
                label![C!["label"], "Iterations"]
            ],
            div![
                C!["field-body"],
                div![
                    C!["field"],
                    p![
                        C!["control", "is-expanded"],
                        input![
                            C!["input"],
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
            C!["field", "is-horizontal"],
            div![C!["field-label", "is-normal"], label![C!["label"], "Colors"]],
            div![
                C!["field-body"],
                div![
                    C!["field"],
                    p![
                        C!["control", "is-expanded"],
                        input![
                            C!["input"],
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
            C!["field", "is-horizontal"],
            div![C!["field-label"]],
            div![
                C!["field-body"],
                div![
                    C!["field", "is-grouped"],
                    div![
                        C!["control"],
                        button![
                            C!["button", "is-primary"],
                            attrs! {
                                At::Disabled => model.rendering.as_at_value()
                            },
                            ev(Ev::Click, |_| Msg::Render),
                            "Render"
                        ]
                    ],
                    div![
                        C!["control"],
                        button![
                            C!["button"],
                            attrs! { At::Disabled => model.rendering.as_at_value() },
                            ev(Ev::Click, |_| Msg::Reset),
                            "Reset"
                        ],
                    ],
                    div![
                        C!["control"],
                        button![C!["button"], ev(Ev::Click, |_| Msg::Export), "Export"],
                    ]
                ]
            ]
        ]
    ]
}
