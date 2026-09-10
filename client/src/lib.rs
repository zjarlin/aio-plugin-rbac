mod dialogs;
mod http;

use aio_plugin_rbac_model::AssignRoleRequest;
use az_dioxus_admin_shell::{
    ApplicationMenuGroup, ApplicationPage, ApplicationPlugin, ApplicationScene,
};
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
        vec![
            ApplicationPage {
                id: "users",
                label: "用户管理",
                icon: Some("user"),
                scene: system_scene(),
                menu_path: system_management_path(),
                required_permission: Some("rbac:manage"),
                render: UsersPage,
            },
            ApplicationPage {
                id: "roles",
                label: "角色管理",
                icon: Some("shield"),
                scene: system_scene(),
                menu_path: system_management_path(),
                required_permission: Some("rbac:manage"),
                render: RolesPage,
            },
        ]
    }
}

fn system_scene() -> ApplicationScene {
    ApplicationScene {
        id: "system",
        label: "系统",
    }
}

fn system_management_path() -> Vec<ApplicationMenuGroup> {
    vec![ApplicationMenuGroup {
        id: "system-management".to_owned(),
        label: "系统管理".to_owned(),
        icon: Some("settings".to_owned()),
    }]
}

pub fn register(builder: &mut CatalogBuilder) {
    builder
        .add_value(AccessControlPlugin)
        .bind::<dyn ApplicationPlugin, AccessControlPlugin>();
}

#[allow(non_snake_case)]
fn UsersPage() -> Element {
    let access = use_resource(http::load);
    let mut creating_user = use_signal(|| false);
    let Some(result) = access.read().as_ref().cloned() else {
        return rsx! { p { "正在读取用户与角色" } };
    };
    let access = match result {
        Ok(value) => value,
        Err(error) => return rsx! { p { role: "alert", "加载权限数据失败：{error}" } },
    };
    let roles = access.roles;
    rsx! {
        section {
            div { class: "flex items-center justify-between gap-3",
                div {
                    h2 { "用户管理" }
                    p { "用户及其角色按当前租户隔离。" }
                }
                Button {
                    r#type: "button",
                    variant: ButtonVariant::Outline,
                    onclick: move |_| creating_user.set(true),
                    "新建用户"
                }
            }
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
        }
        if creating_user() {
            CreateUserDialog { on_close: move |_| creating_user.set(false) }
        }
    }
}

#[allow(non_snake_case)]
fn RolesPage() -> Element {
    let access = use_resource(http::load);
    let mut creating_role = use_signal(|| false);
    let Some(result) = access.read().as_ref().cloned() else {
        return rsx! { p { "正在读取角色" } };
    };
    let access = match result {
        Ok(value) => value,
        Err(error) => return rsx! { p { role: "alert", "加载角色数据失败：{error}" } },
    };
    rsx! {
        section {
            div { class: "flex items-center justify-between gap-3",
                div {
                    h2 { "角色管理" }
                    p { "角色及权限按当前租户隔离。" }
                }
                Button {
                    r#type: "button",
                    variant: ButtonVariant::Outline,
                    onclick: move |_| creating_role.set(true),
                    "新建角色"
                }
            }
            div { class: "grid gap-3 md:grid-cols-2",
                for role in access.roles {
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
        if creating_role() {
            CreateRoleDialog { on_close: move |_| creating_role.set(false) }
        }
    }
}
