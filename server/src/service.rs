use std::env;

use aio_plugin_rbac_model::{AccessControlView, RoleItem, UserItem};
use anyhow::{Context as _, Result, ensure};
use sqlx::{PgPool, postgres::PgPoolOptions};

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS tenant_member_roles (
    tenant_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    PRIMARY KEY(tenant_id, user_id, role_id)
);
CREATE TABLE IF NOT EXISTS role_permissions (
    tenant_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    permission TEXT NOT NULL,
    PRIMARY KEY(tenant_id, role_id, permission)
);
"#;

#[derive(Debug)]
pub struct AccessControlService {
    pool: PgPool,
}

impl AccessControlService {
    pub fn from_env() -> Result<Self> {
        let database_url = env::var("AIO_DATABASE_URL")
            .or_else(|_| env::var("AZ_AIO_DATABASE_URL"))
            .context("权限插件缺少 AIO_DATABASE_URL")?;
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(8)
                .connect_lazy(&database_url)
                .context("创建权限数据库连接池失败")?,
        })
    }

    pub async fn initialize(&self) -> Result<()> {
        sqlx::raw_sql(SCHEMA)
            .execute(&self.pool)
            .await
            .context("创建权限插件数据表失败")?;
        sqlx::query("INSERT INTO role_permissions (tenant_id, role_id, permission) SELECT id, 'member', 'workspace:view' FROM tenants ON CONFLICT DO NOTHING")
            .execute(&self.pool)
            .await
            .context("初始化租户成员角色失败")?;
        for permission in [
            "plugin:manage",
            "tenant:manage",
            "rbac:manage",
            "dictionary:manage",
            "file:manage",
        ] {
            sqlx::query(
                "INSERT INTO role_permissions (tenant_id, role_id, permission) SELECT DISTINCT tenant_id, role_id, $1 FROM tenant_member_roles WHERE role_id IN ('platform-admin', 'tenant-admin') ON CONFLICT DO NOTHING",
            )
            .bind(permission)
            .execute(&self.pool)
            .await
            .context("补齐系统管理员权限失败")?;
        }
        Ok(())
    }

    pub async fn view(&self, tenant_id: &str) -> Result<AccessControlView> {
        let user_rows = sqlx::query_as::<_, (String, String, String)>(
            "SELECT users.id, users.account, users.display_name FROM tenant_memberships memberships JOIN identity_users users ON users.id = memberships.user_id WHERE memberships.tenant_id = $1 ORDER BY users.account",
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;
        let mut users = Vec::new();
        for (id, account, display_name) in user_rows {
            let roles = sqlx::query_scalar::<_, String>(
                "SELECT role_id FROM tenant_member_roles WHERE tenant_id = $1 AND user_id = $2 ORDER BY role_id",
            )
            .bind(tenant_id)
            .bind(&id)
            .fetch_all(&self.pool)
            .await?;
            users.push(UserItem {
                id,
                account,
                display_name,
                roles,
            });
        }
        let role_ids = sqlx::query_scalar::<_, String>(
            "SELECT DISTINCT role_id FROM role_permissions WHERE tenant_id = $1 ORDER BY role_id",
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;
        let mut roles = Vec::new();
        for id in role_ids {
            let permissions = sqlx::query_scalar::<_, String>(
                "SELECT permission FROM role_permissions WHERE tenant_id = $1 AND role_id = $2 ORDER BY permission",
            )
            .bind(tenant_id)
            .bind(&id)
            .fetch_all(&self.pool)
            .await?;
            roles.push(RoleItem { id, permissions });
        }
        Ok(AccessControlView { users, roles })
    }

    pub async fn create_role(
        &self,
        tenant_id: &str,
        role_id: &str,
        permissions: &[String],
    ) -> Result<()> {
        let role_id = role_id.trim();
        ensure!(!role_id.is_empty(), "角色 ID 不能为空");
        ensure!(
            role_id.chars().all(
                |character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_')
            ),
            "角色 ID 只能包含字母、数字、连字符和下划线"
        );
        ensure!(!permissions.is_empty(), "角色至少需要一项权限");
        let mut transaction = self.pool.begin().await?;
        for permission in permissions {
            let permission = permission.trim();
            ensure!(!permission.is_empty(), "权限不能为空");
            ensure!(permission.contains(':'), "权限必须使用 domain:action 格式");
            sqlx::query("INSERT INTO role_permissions (tenant_id, role_id, permission) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
                .bind(tenant_id).bind(role_id).bind(permission)
                .execute(&mut *transaction).await?;
        }
        transaction.commit().await?;
        Ok(())
    }

    pub async fn assign(&self, tenant_id: &str, user_id: &str, role_id: &str) -> Result<()> {
        let valid = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM tenant_memberships WHERE tenant_id = $1 AND user_id = $2) AND EXISTS(SELECT 1 FROM role_permissions WHERE tenant_id = $1 AND role_id = $3)",
        )
        .bind(tenant_id)
        .bind(user_id)
        .bind(role_id)
        .fetch_one(&self.pool)
        .await?;
        ensure!(valid, "用户或角色不属于当前租户");
        sqlx::query("INSERT INTO tenant_member_roles (tenant_id, user_id, role_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
            .bind(tenant_id).bind(user_id).bind(role_id).execute(&self.pool).await?;
        Ok(())
    }
}
