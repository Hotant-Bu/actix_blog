use futures_util::StreamExt;
use redis::aio::ConnectionManager;
use sqlx::MySqlPool;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

/// 权限结构体
/// 权限格式： resource:action
/// ## 示例：
/// - "projects:create" - 创建项目
/// - "projects:read" - 读取项目
/// - "projects:*" - 项目的所有操作
/// - "*:*" - 超级管理员（所有资源的所有操作）
///
/// ## 字段说明：
/// - `resource`: 资源名称，如 "projects"、"users"、"*"（所有资源）
/// - `action`: 操作名称，如 "create"、"read"、"*"（所有操作）
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Permission {
    /// 资源名称（如：projects, users, departments）
    pub resource: String,
    /// 操作名称（如：create, read, update, delete）
    pub action: String,
}

impl Permission {
    /// 创建新的权限对象
    pub fn new(resource: &str, action: &str) -> Self {
        Self {
            resource: resource.to_string(),
            action: action.to_string(),
        }
    }

    /// 从字符串解析权限
    /// 将 "resource:action" 格式的字符串解析为 Permission 对象
    pub fn from_string(perm_str: &str) -> Option<Self> {
        // 使用':'分割字符串
        let parts: Vec<&str> = perm_str.split(":").collect();
        // 必须恰好有两部分：resource和action
        if parts.len() == 2 {
            Some(Self {
                resource: parts[0].to_string(),
                action: parts[1].to_string(),
            })
        } else {
            None
        }
    }

    /// 权限转为字符串
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.resource, self.action)
    }

    /// 检查权限是否匹配
    pub fn matches(&self, required: &Permission) -> bool {
        // 检查资源是否匹配：
        // 1. 当前权限的资源是 "*"（所有资源）
        // 2. 或者当前权限的资源与所需权限的资源相同
        let resource_match = self.resource == "*" || self.resource == required.resource;

        // 检查操作是否匹配：
        // 1. 当前权限的操作是 "*"（所有操作）
        // 2. 或者当前权限的操作与所需权限的操作相同
        let action_match = self.action == "*" || self.action == required.action;

        // 只有资源和操作都匹配时，权限才匹配
        resource_match && action_match
    }
}

/// 从数据库中获取用户权限并缓存到Redis
/// 优先从Redis缓存获取，缓存未命中时从数据库查询
/// ## 工作流程：
/// 1. 尝试从 Redis 缓存获取用户权限
/// 2. 如果缓存命中，直接返回权限列表
/// 3. 如果缓存未命中，从数据库查询用户权限
/// 4. 将查询结果缓存到 Redis（5分钟过期）
/// 5. 返回权限列表
/// ## 参数：
/// - `pool`: MySQL 数据库连接池
/// - `redis`: Redis 连接管理器
/// - `user_id`: 用户 ID
///
/// ## 返回值：
/// - `Ok(Vec<Permission>)`: 用户的权限列表
/// - `Err`: 数据库或 Redis 错误
///
/// ## 性能优化：
/// - 使用 Redis 缓存减少数据库查询
/// - 缓存过期时间为 5 分钟（300 秒）
/// - 权限变更时需要手动清除缓存
pub async fn get_user_permissions_cached(
    pool: &MySqlPool,
    redis: &ConnectionManager,
    user_id: i64,
) -> Result<Vec<Permission>, Box<dyn std::error::Error>> {
    // 构造Redis缓存键：user_permissions:{user_id}
    let cache_key = format!("user_permissions:{user_id}");
    let mut conn = redis.clone();
    // 尝试从Redis获取缓存
    // let cached = redis::AsyncCommands::get(&mut conn, &cache_key).await.ok();
    let cached:Option<String> = conn.get(&cache_key).await.ok();

    if let Some(cached_json) = cached {
        // 缓存命中，解析JSON字符串
        if let Ok(perms) = serde_json::from_str::<Vec<String>>(&cached_json) {
            // 将字符串列表转换未Permission对象列表
            let permissions: Vec<Permission> = perms
                .iter()
                .filter_map(|perm| Permission::from_string(perm))
                .collect();
            return Ok(permissions);
        }
    }

    // 如果缓存未命令，则从数据库中获取权限
    let db_permissions = crate::db::get_user_permissions(pool, user_id).await?;

    // 将数据库中返回的字符串列表转换为Permission对象列表
    let permissions: Vec<Permission> = db_permissions
        .iter()
        .filter_map(|perm| Permission::from_string(perm))
        .collect();

    // 将权限缓存到Redis（设置过期时间，5分钟）
    let perms_json = serde_json::to_string(&permissions)?;
    // 设置键值，并指定过期时间（单位：秒）
    let _: () = redis::AsyncCommands::set_ex(&mut conn, &cache_key, perms_json, 300).await?;
    Ok(permissions)
}

/// # 检查用户是否有指定权限
///
/// 这是权限检查的核心函数，用于判断用户是否有执行某个操作的权限
///
/// ## 工作流程：
/// 1. 从缓存或数据库获取用户的所有权限
/// 2. 创建所需权限对象
/// 3. 遍历用户权限，检查是否有匹配的权限（支持通配符）
/// 4. 只要有一个权限匹配，就返回 true
///
/// ## 参数：
/// - `pool`: MySQL 数据库连接池
/// - `redis`: Redis 连接管理器
/// - `user_id`: 用户 ID
/// - `required_resource`: 所需的资源名称（如 "projects"）
/// - `required_action`: 所需的操作名称（如 "create"）
///
/// ## 返回值：
/// - `Ok(true)`: 用户有该权限
/// - `Ok(false)`: 用户没有该权限
/// - `Err`: 查询权限时发生错误
///
/// ## 示例：
/// ```rust
/// // 检查用户是否可以创建项目
/// let has_perm = check_permission(&pool, &redis, user_id, "projects", "create").await?;
/// if has_perm {
///     // 执行创建项目的逻辑
/// } else {
///     // 返回 403 Forbidden
/// }
/// ```
///
/// ## 权限匹配逻辑：
/// - 如果用户有 `*:*` 权限，可以执行任何操作
/// - 如果用户有 `projects:*` 权限，可以对项目执行任何操作
/// - 如果用户有 `projects:create` 权限，只能创建项目
pub async fn check_permission(
    pool: &MySqlPool,
    redis: &ConnectionManager,
    user_id: i64,
    required_resource: &str,
    required_action: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // 步骤1: 获取用户的所有权限（带缓存）
    let user_permissions = get_user_permissions_cached(pool, redis, user_id).await?;

    // 步骤2: 创建所需权限对象
    let required = Permission::new(required_resource, required_action);

    // 步骤3: 遍历用户权限，检查是否有匹配的权限
    for perm in user_permissions {
        // 使用 matches 方法检查权限是否匹配（支持通配符）
        if perm.matches(&required) {
            // 只要有一个权限匹配，就返回 true
            return Ok(true);
        }
    }

    // 步骤4: 没有匹配的权限，返回 false
    Ok(false)
}

/// # 清除用户权限缓存
///
/// 当用户的权限发生变更时（如分配新角色、修改角色权限等），需要调用此函数清除缓存
///
/// ## 使用场景：
/// 1. 为用户分配新角色后
/// 2. 从用户移除角色后
/// 3. 修改角色的权限后
/// 4. 修改权限定义后
///
/// ## 参数：
/// - `redis`: Redis 连接管理器
/// - `user_id`: 用户 ID
///
/// ## 返回值：
/// - `Ok(())`: 缓存清除成功
/// - `Err`: Redis 操作失败
///
/// ## 示例：
/// ```rust
/// // 为用户分配角色后，清除权限缓存
/// assign_role_to_user(&pool, user_id, role_id).await?;
/// clear_user_permissions_cache(&redis, user_id).await?;
/// ```
///
/// ## 注意事项：
/// - 清除缓存后，下次权限检查会重新从数据库查询
/// - 如果忘记清除缓存，用户可能看到旧的权限（最多5分钟）
pub async fn clear_user_permissions_cache(
    redis: &ConnectionManager,
    user_id: i64,
) -> Result<(), redis::RedisError> {
    // 构造缓存键
    let cache_key = format!("user_permissions:{}", user_id);
    let mut conn = redis.clone();

    // 从 Redis 删除该键
    let _: () = redis::AsyncCommands::del(&mut conn, &cache_key).await?;

    Ok(())
}

/// 预定义的权限常量
pub mod permissions {
    // 超级管理员
    pub const SUPER_ADMIN: &str = "*:*";

    // 项目权限
    pub const PROJECTS_CREATE: &str = "projects:create";
    pub const PROJECTS_READ: &str = "projects:read";
    pub const PROJECTS_UPDATE: &str = "projects:update";
    pub const PROJECTS_DELETE: &str = "projects:delete";
    pub const PROJECTS_ALL: &str = "projects:*";

    // 用户权限
    pub const USERS_CREATE: &str = "users:create";
    pub const USERS_READ: &str = "users:read";
    pub const USERS_UPDATE: &str = "users:update";
    pub const USERS_DELETE: &str = "users:delete";
    pub const USERS_ALL: &str = "users:*";

    // 角色权限
    pub const ROLES_CREATE: &str = "roles:create";
    pub const ROLES_READ: &str = "roles:read";
    pub const ROLES_UPDATE: &str = "roles:update";
    pub const ROLES_DELETE: &str = "roles:delete";
    pub const ROLES_ALL: &str = "roles:*";

    // 权限管理
    pub const PERMISSIONS_CREATE: &str = "permissions:create";
    pub const PERMISSIONS_READ: &str = "permissions:read";
    pub const PERMISSIONS_UPDATE: &str = "permissions:update";
    pub const PERMISSIONS_DELETE: &str = "permissions:delete";
    pub const PERMISSIONS_ALL: &str = "permissions:*";

    // 部门权限
    pub const DEPARTMENTS_CREATE: &str = "departments:create";
    pub const DEPARTMENTS_READ: &str = "departments:read";
    pub const DEPARTMENTS_UPDATE: &str = "departments:update";
    pub const DEPARTMENTS_DELETE: &str = "departments:delete";
    pub const DEPARTMENTS_ALL: &str = "departments:*";

    // 员工权限
    pub const EMPLOYEES_CREATE: &str = "employees:create";
    pub const EMPLOYEES_READ: &str = "employees:read";
    pub const EMPLOYEES_UPDATE: &str = "employees:update";
    pub const EMPLOYEES_DELETE: &str = "employees:delete";
    pub const EMPLOYEES_ALL: &str = "employees:*";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_parsing() {
        let perm = Permission::from_string("projects:create").unwrap();
        assert_eq!(perm.resource, "projects");
        assert_eq!(perm.action, "create");
    }

    #[test]
    fn test_permission_matching() {
        let super_admin = Permission::new("*", "*");
        let required = Permission::new("projects", "create");
        assert!(super_admin.matches(&required));

        let projects_all = Permission::new("projects", "*");
        assert!(projects_all.matches(&required));

        let projects_create = Permission::new("projects", "create");
        assert!(projects_create.matches(&required));

        let users_create = Permission::new("users", "create");
        assert!(!users_create.matches(&required));
    }
}
