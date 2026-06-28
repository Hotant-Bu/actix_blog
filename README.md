# 公司官网后台管理系统

基于 Rust + Actix-web + SQLx + MySQL 构建的企业官网后台管理系统。

## 功能特性

- ✅ **用户认证与授权**
  - JWT Token 认证
  - 用户注册、登录
  - 密码加密存储（bcrypt）

- ✅ **RBAC 权限管理**
  - 角色管理（创建、编辑、删除、查询）
  - 权限管理（创建、编辑、删除、查询）
  - 角色权限分配
  - 用户角色分配
  - 基于权限的访问控制

- ✅ **组织架构管理**
  - 部门管理（支持层级结构）
  - 员工管理
  - 员工与用户关联
  - 员工与部门关联

- ✅ **项目案例展示**
  - 项目创建、编辑、删除、查询
  - 项目状态管理（草稿/发布）
  - 项目封面图片
  - 项目视频链接
  - 项目详细描述
  - 项目图片集（多图展示）
  - 显示顺序控制

- ✅ **文件上传**
  - 支持图片、视频等文件上传
  - 文件大小限制
  - UUID 文件命名
  - 静态文件访问

## 技术栈

- **Web 框架**: Actix-web 4.5
- **数据库**: MySQL + SQLx
- **认证**: JWT (jsonwebtoken)
- **密码加密**: bcrypt
- **序列化**: serde + serde_json
- **验证**: validator
- **异步运行时**: Tokio
- **文件上传**: actix-multipart

## 项目结构

```
.
├── src/
│   ├── main.rs              # 应用入口
│   ├── config.rs            # 配置管理
│   ├── models/              # 数据模型
│   │   ├── user.rs
│   │   ├── role.rs
│   │   ├── permission.rs
│   │   ├── department.rs
│   │   ├── employee.rs
│   │   ├── project.rs
│   │   └── ...
│   ├── db/                  # 数据库操作
│   │   ├── users.rs
│   │   ├── roles.rs
│   │   ├── permissions.rs
│   │   ├── departments.rs
│   │   ├── employees.rs
│   │   ├── projects.rs
│   │   └── ...
│   ├── handlers/            # API 处理器
│   │   ├── auth.rs
│   │   ├── users.rs
│   │   ├── roles.rs
│   │   ├── permissions.rs
│   │   ├── departments.rs
│   │   ├── employees.rs
│   │   ├── projects.rs
│   │   └── upload.rs
│   ├── middleware/          # 中间件
│   │   └── auth.rs
│   └── utils/               # 工具函数
│       ├── jwt.rs
│       ├── password.rs
│       └── response.rs
├── migrations/              # 数据库迁移
│   └── 001_init.sql
├── uploads/                 # 上传文件目录
├── Cargo.toml
├── .env.example
└── README.md
```

## 快速开始

### 1. 环境要求

- Rust 1.70+
- MySQL 8.0+

### 2. 数据库配置

创建数据库：

```sql
CREATE DATABASE company_backend CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

执行迁移脚本：

```bash
mysql -u root -p company_backend < migrations/001_init.sql
```

### 3. 环境配置

复制 `.env.example` 为 `.env` 并修改配置：

```bash
cp .env.example .env
```

编辑 `.env` 文件：

```env
DATABASE_URL=mysql://root:your_password@localhost:3306/company_backend
JWT_SECRET=your-secret-key-change-this-in-production
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
UPLOAD_DIR=./uploads
MAX_FILE_SIZE=52428800
RUST_LOG=info
```

### 4. 运行项目

```bash
# 开发模式
cargo run

# 生产模式
cargo build --release
./target/release/company-backend
```

服务将在 `http://127.0.0.1:8080` 启动。

## API 文档

### 认证接口

#### 用户注册
```http
POST /api/auth/register
Content-Type: application/json

{
  "username": "admin",
  "email": "admin@example.com",
  "password": "password123"
}
```

#### 用户登录
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "password123"
}
```

响应：
```json
{
  "success": true,
  "message": "Success",
  "data": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
    "user_id": 1,
    "username": "admin"
  }
}
```

### 用户管理

所有用户管理接口需要在请求头中携带 JWT Token：
```
Authorization: Bearer <token>
```

#### 获取当前用户信息
```http
GET /api/users/me
```

#### 获取所有用户
```http
GET /api/users
```

#### 获取指定用户
```http
GET /api/users/{id}
```

#### 更新用户
```http
PUT /api/users/{id}
Content-Type: application/json

{
  "username": "newname",
  "email": "newemail@example.com",
  "is_active": true
}
```

#### 删除用户
```http
DELETE /api/users/{id}
```

#### 为用户分配角色
```http
POST /api/users/{id}/roles
Content-Type: application/json

{
  "role_ids": [1, 2]
}
```

#### 获取用户权限
```http
GET /api/users/{id}/permissions
```

### 角色管理

#### 创建角色
```http
POST /api/roles
Content-Type: application/json

{
  "name": "编辑",
  "description": "内容编辑人员"
}
```

#### 获取所有角色
```http
GET /api/roles
```

#### 获取指定角色
```http
GET /api/roles/{id}
```

#### 更新角色
```http
PUT /api/roles/{id}
Content-Type: application/json

{
  "name": "高级编辑",
  "description": "高级内容编辑人员"
}
```

#### 删除角色
```http
DELETE /api/roles/{id}
```

#### 为角色分配权限
```http
POST /api/roles/{id}/permissions
Content-Type: application/json

{
  "permission_ids": [1, 2, 3]
}
```

#### 获取角色权限
```http
GET /api/roles/{id}/permissions
```

### 权限管理

#### 创建权限
```http
POST /api/permissions
Content-Type: application/json

{
  "name": "查看报表",
  "resource": "reports",
  "action": "read",
  "description": "查看系统报表"
}
```

#### 获取所有权限
```http
GET /api/permissions
```

#### 获取指定权限
```http
GET /api/permissions/{id}
```

#### 更新权限
```http
PUT /api/permissions/{id}
```

#### 删除权限
```http
DELETE /api/permissions/{id}
```

### 部门管理

#### 创建部门
```http
POST /api/departments
Content-Type: application/json

{
  "name": "技术部",
  "parent_id": null,
  "description": "技术研发部门"
}
```

#### 获取所有部门
```http
GET /api/departments
```

#### 获取指定部门
```http
GET /api/departments/{id}
```

#### 更新部门
```http
PUT /api/departments/{id}
```

#### 删除部门
```http
DELETE /api/departments/{id}
```

### 员工管理

#### 创建员工
```http
POST /api/employees
Content-Type: application/json

{
  "user_id": 1,
  "department_id": 1,
  "employee_number": "EMP001",
  "full_name": "张三",
  "position": "高级工程师",
  "phone": "13800138000",
  "hire_date": "2024-01-01T00:00:00Z"
}
```

#### 获取所有员工
```http
GET /api/employees
```

#### 获取指定员工
```http
GET /api/employees/{id}
```

#### 更新员工
```http
PUT /api/employees/{id}
```

#### 删除员工
```http
DELETE /api/employees/{id}
```

### 项目案例管理

#### 创建项目
```http
POST /api/projects
Content-Type: application/json

{
  "title": "某大型电商平台",
  "description": "为客户打造的大型电商平台...",
  "cover_image": "/uploads/xxx.jpg",
  "video_url": "https://example.com/video.mp4",
  "status": "published",
  "display_order": 1
}
```

#### 获取所有项目（需认证）
```http
GET /api/projects
```

#### 获取已发布项目（公开接口）
```http
GET /api/projects/published
```

#### 获取指定项目
```http
GET /api/projects/{id}
```

#### 更新项目
```http
PUT /api/projects/{id}
```

#### 删除项目
```http
DELETE /api/projects/{id}
```

#### 添加项目图片
```http
POST /api/projects/images
Content-Type: application/json

{
  "project_id": 1,
  "image_url": "/uploads/xxx.jpg",
  "caption": "项目截图",
  "display_order": 1
}
```

#### 获取项目图片
```http
GET /api/projects/{id}/images
```

#### 删除项目图片
```http
DELETE /api/projects/images/{id}
```

### 文件上传

#### 上传文件
```http
POST /api/upload
Content-Type: multipart/form-data

file: <binary>
```

响应：
```json
{
  "success": true,
  "message": "File uploaded successfully",
  "data": {
    "url": "/uploads/uuid.jpg",
    "filename": "uuid.jpg"
  }
}
```

## 默认数据

系统初始化后会创建以下默认角色：

1. **超级管理员** - 拥有所有权限
2. **管理员** - 拥有大部分管理权限
3. **编辑** - 可以管理内容
4. **查看者** - 只能查看内容

以及相应的权限数据。

## 安全建议

1. 修改 `.env` 中的 `JWT_SECRET` 为强密码
2. 使用 HTTPS 部署生产环境
3. 定期更新依赖包
4. 限制文件上传大小和类型
5. 配置 CORS 策略
6. 使用环境变量管理敏感信息

## 开发建议

1. 使用 `cargo watch` 进行热重载开发：
   ```bash
   cargo install cargo-watch
   cargo watch -x run
   ```

2. 代码格式化：
   ```bash
   cargo fmt
   ```

3. 代码检查：
   ```bash
   cargo clippy
   ```

## 许可证

MIT License
