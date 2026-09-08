use aio_plugin_rbac_model::{CreateRoleRequest, CreateUserRequest};
use az_ui_components::{
    button::{Button, ButtonVariant},
    dialog::{Dialog, DialogDescription, DialogTitle},
    input::Input,
};
use dioxus::prelude::*;

use crate::http;

#[allow(non_snake_case)]
#[component]
pub fn CreateUserDialog(on_close: EventHandler<()>) -> Element {
    let mut account = use_signal(String::new);
    let mut display_name = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    rsx! {
        Dialog {
            open: true,
            on_open_change: move |open: bool| if !open { on_close.call(()) },
            form {
                class: "grid gap-3",
                onsubmit: move |event| {
                    event.prevent_default();
                    let request = CreateUserRequest {
                        account: account(),
                        display_name: display_name(),
                        password: password(),
                    };
                    spawn(async move {
                        match http::create_user(request).await {
                            Ok(()) => http::reload(),
                            Err(message) => error.set(Some(message)),
                        }
                    });
                },
                DialogTitle { "新建用户" }
                DialogDescription { "用户仅加入当前租户，初始密码至少 12 个字符。" }
                label { r#for: "new-account", "账号" }
                Input {
                    id: "new-account",
                    aria_label: "新用户账号",
                    value: account(),
                    oninput: move |event: FormEvent| account.set(event.value()),
                }
                label { r#for: "new-display-name", "显示名称" }
                Input {
                    id: "new-display-name",
                    aria_label: "显示名称",
                    value: display_name(),
                    oninput: move |event: FormEvent| display_name.set(event.value()),
                }
                label { r#for: "new-user-password", "初始密码" }
                Input {
                    id: "new-user-password",
                    r#type: "password",
                    minlength: 12,
                    aria_label: "初始密码",
                    value: password(),
                    oninput: move |event: FormEvent| password.set(event.value()),
                }
                if let Some(message) = error() {
                    p { role: "alert", "{message}" }
                }
                footer { class: "flex justify-end gap-2",
                    Button {
                        r#type: "button",
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| on_close.call(()),
                        "取消"
                    }
                    Button { r#type: "submit", "创建" }
                }
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
pub fn CreateRoleDialog(on_close: EventHandler<()>) -> Element {
    let mut role_id = use_signal(String::new);
    let mut permissions = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    rsx! {
        Dialog {
            open: true,
            on_open_change: move |open: bool| if !open { on_close.call(()) },
            form {
                class: "grid gap-3",
                onsubmit: move |event| {
                    event.prevent_default();
                    let request = CreateRoleRequest {
                        role_id: role_id(),
                        permissions: permissions()
                            .split(',')
                            .map(str::trim)
                            .filter(|value| !value.is_empty())
                            .map(str::to_owned)
                            .collect(),
                    };
                    spawn(async move {
                        match http::create_role(request).await {
                            Ok(()) => http::reload(),
                            Err(message) => error.set(Some(message)),
                        }
                    });
                },
                DialogTitle { "新建角色" }
                DialogDescription { "权限使用逗号分隔，例如 plugin:manage。" }
                label { r#for: "new-role-id", "角色 ID" }
                Input {
                    id: "new-role-id",
                    aria_label: "角色 ID",
                    value: role_id(),
                    oninput: move |event: FormEvent| role_id.set(event.value()),
                }
                label { r#for: "new-role-permissions", "权限" }
                Input {
                    id: "new-role-permissions",
                    aria_label: "角色权限",
                    value: permissions(),
                    oninput: move |event: FormEvent| permissions.set(event.value()),
                }
                if let Some(message) = error() {
                    p { role: "alert", "{message}" }
                }
                footer { class: "flex justify-end gap-2",
                    Button {
                        r#type: "button",
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| on_close.call(()),
                        "取消"
                    }
                    Button { r#type: "submit", "创建" }
                }
            }
        }
    }
}
