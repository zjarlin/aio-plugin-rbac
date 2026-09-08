mod dialogs;
mod http;

use aio_plugin_rbac_model::AssignRoleRequest;
use az_dioxus_admin_shell::{ApplicationPage, ApplicationPlugin, ApplicationScene};
use az_ui_components::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonVariant},
};
use dill::CatalogBuilder;
use dioxus::prelude::*;

use dialogs::{CreateRoleDialog, CreateUserDialog};

#[derive(Debug)]
pub struct AccessControlPlugin;

impl ApplicationPlugin for AccessControlPlugin {
    fn pages(&self) -> Vec<ApplicationPage> {
        vec![ApplicationPage {
            id: "access-control",
            label: "用户与权限",
            icon: Some("shield"),
            scene: ApplicationScene {
                id: "system",
                label: "系统",
            },
            required_permission: Some("rbac:manage"),
            render: AccessControlPage,
        }]
    }
}

pub fn register(builder: &mut CatalogBuilder) {
    builder
        .add_value(AccessControlPlugin)
        .bind::<dyn ApplicationPlugin, AccessControlPlugin>();
}

#[allow(non_snake_case)]
fn AccessControlPage() -> Element {
    let access = use_resource(http::load);
    let mut creating_user = use_signal(|| false);
    let mut creating_role = use_signal(|| false);
    let Some(result) = access.read().as_ref().cloned() else {
        return rsx! { p { "正在读取用户与角色" } };
    };
    let access = match result {
        Ok(value) => value,
        Err(error) => return rsx! { p { role: "alert", "加载权限数据失败：{error}" } },
    };
    let roles = access.roles.clone();
    rsx! {
        section {
            div { class: "flex items-center justify-between gap-3",
                div {
                    h2 { "用户与权限" }
                    p { "用户、角色和权限按当前租户隔离。" }
                }
                div { class: "flex gap-2",
                    Button {
                        r#type: "button",
                        variant: ButtonVariant::Outline,
                        onclick: move |_| creating_user.set(true),
                        "新建用户"
                    }
                    Button {
                        r#type: "button",
                        variant: ButtonVariant::Outline,
                        onclick: move |_| creating_role.set(true),
                        "新建角色"
                    }
                }
            }
            h3 { "用户" }
            div { class: "grid gap-3 md:grid-cols-2",
                for user in access.users {
                    article { class: "border p-4",
                        h4 { "{user.display_name}" }
                        p { "@{user.account}" }
                        div { class: "flex flex-wrap gap-2",
                            for role in user.roles.iter() {
                                Badge { variant: BadgeVariant::Outline, "{role}" }
                            }
                        }
                        div { class: "flex flex-wrap gap-2",
                            for role in roles.iter().filter(|role| !user.roles.contains(&role.id)) {
                                Button {
                                    r#type: "button",
                                    variant: ButtonVariant::Ghost,
                                    onclick: {
                                        let request = AssignRoleRequest {
                                            user_id: user.id.clone(),
                                            role_id: role.id.clone(),
                                        };
                                        move |_| {
                                            let request = request.clone();
                                            spawn(async move {
                                                if http::assign(request).await.is_ok() {
                                                    http::reload();
                                                }
                                            });
                                        }
                                    },
                                    "授予 {role.id}"
                                }
                            }
                        }
                    }
                }
            }
            h3 { "角色" }
            div { class: "grid gap-3 md:grid-cols-2",
                for role in roles {
                    article { class: "border p-4",
                        h4 { "{role.id}" }
                        div { class: "flex flex-wrap gap-2",
                            for permission in role.permissions {
                                Badge { variant: BadgeVariant::Outline, "{permission}" }
                            }
                        }
                    }
                }
            }
        }
        if creating_user() {
            CreateUserDialog { on_close: move |_| creating_user.set(false) }
        }
        if creating_role() {
            CreateRoleDialog { on_close: move |_| creating_role.set(false) }
        }
    }
}
