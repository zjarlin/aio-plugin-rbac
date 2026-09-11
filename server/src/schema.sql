CREATE TABLE IF NOT EXISTS tenant_member_roles (
    tenant_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    PRIMARY KEY (tenant_id, user_id, role_id)
);
CREATE TABLE IF NOT EXISTS role_permissions (
    tenant_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    permission TEXT NOT NULL,
    PRIMARY KEY (tenant_id, role_id, permission)
);
