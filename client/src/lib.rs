use az_dioxus_admin_shell::{ApplicationPage, ApplicationPlugin, ApplicationScene};
use dill::CatalogBuilder;
use dioxus::prelude::*;

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
    rsx! {
        section {
            h2 { "用户与权限" }
            p { "用户、角色和权限均按当前租户隔离。" }
        }
    }
}
