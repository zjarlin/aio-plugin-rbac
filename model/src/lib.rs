use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccessControlView {
    pub users: Vec<UserItem>,
    pub roles: Vec<RoleItem>,
    pub current_user_id: String,
    pub grantable_permissions: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserItem {
    pub id: String,
    pub account: String,
    pub display_name: String,
    pub roles: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoleItem {
    pub id: String,
    pub permissions: Vec<String>,
    pub member_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub account: String,
    pub display_name: String,
    pub password: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub role_id: String,
    pub permissions: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssignRoleRequest {
    pub user_id: String,
    pub role_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpdateMemberRequest {
    pub display_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpdateMemberRolesRequest {
    pub roles: Vec<String>,
}

pub fn is_system_role(id: &str) -> bool {
    matches!(id, "platform-admin" | "tenant-admin" | "member")
}

pub fn permission_label(permission: &str) -> &str {
    match permission {
        "workspace:view" => "访问工作区",
        "rbac:manage" => "管理用户与角色",
        "tenant:manage" => "管理租户",
        "plugin:manage" => "管理插件",
        "dictionary:manage" => "管理字典",
        "file:manage" => "管理文件",
        _ => permission,
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccessControlResponse<T> {
    pub data: T,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccessControlErrorResponse {
    pub error: String,
}
