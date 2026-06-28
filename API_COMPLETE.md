# 完整 API 接口文档

## 📋 目录

- [基础信息](#基础信息)
- [认证说明](#认证说明)
- [响应格式](#响应格式)
- [公开 API（无需认证）](#公开-api无需认证)
- [认证 API](#认证-api)
- [用户管理 API](#用户管理-api)
- [角色管理 API](#角色管理-api)
- [权限管理 API](#权限管理-api)
- [部门管理 API](#部门管理-api)
- [员工管理 API](#员工管理-api)
- [项目管理 API](#项目管理-api)
- [文件上传 API](#文件上传-api)
- [管理员 API](#管理员-api)

---

## 基础信息

- **基础 URL**: `http://localhost:8080/api`
- **认证方式**: JWT Bearer Token
- **内容类型**: `application/json`
- **字符编码**: `UTF-8`

---

## 认证说明

### 认证方式

大部分 API 需要在请求头中携带 JWT Token：

```http
Authorization: Bearer <your_jwt_token>
```

### 权限级别

1. **公开访问**：无需认证
2. **已认证用户**：需要 JWT Token
3. **管理员**：需要 JWT Token + 超级管理员权限 (`*:*`)

---

## 响应格式

### 成功响应

```json
{
  "success": true,
  "message": "Success",
  "data": { ... }
}
```

### 错误响应

```json
{
  "success": false,
  "message": "错误信息描述"
}
```

### HTTP 状态码

- `200 OK` - 请求成功
- `201 Created` - 资源创建成功
- `400 Bad Request` - 请求参数错误
- `401 Unauthorized` - 未认证或 Token 无效
- `403 Forbidden` - 无权限访问
- `404 Not Found` - 资源不存在
- `500 Internal Server Error` - 服务器内部错误

---

## 公开 API（无需认证）

### 1. 获取已发布项目列表

获取所有已发布的项目（前台展示）

**请求**
```http
GET /api/public/projects
```

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    {
      "id": 1,
      "title": "项目标题",
      "description": "项目描述",
      "status": "published",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### 2. 获取单个已发布项目

获取指定已发布项目的详细信息

**请求**
```http
GET /api/public/projects/{id}
```

**路径参数**
- `id` (integer) - 项目 ID

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": {
    "id": 1,
    "title": "项目标题",
    "description": "项目描述",
    "content": "项目详细内容",
    "status": "published",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 3. 获取项目图片

获取指定已发布项目的所有图片

**请求**
```http
GET /api/public/projects/{id}/images
```

**路径参数**
- `id` (integer) - 项目 ID

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    {
      "id": 1,
      "project_id": 1,
      "image_url": "/uploads/image.jpg",
      "description": "图片描述",
      "sort_order": 1,
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### 4. 获取公司信息

获取公司基本信息

**请求**
```http
GET /api/public/company
```

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": {
    "name": "公司名称",
    "description": "公司简介",
    "contact": {
      "email": "contact@company.com",
      "phone": "123-456-7890",
      "address": "公司地址"
    }
  }
}
```

---

## 认证 API

### 1. 用户注册

注册新用户账号

**请求**
```http
POST /api/auth/register
Content-Type: application/json

{
  "username": "admin",
  "email": "admin@example.com",
  "password": "password123"
}
```

**请求参数**
- `username` (string, required) - 用户名，3-50 字符
- `email` (string, required) - 邮箱地址
- `password` (string, required) - 密码，至少 6 字符

**响应**
```json
{
  "success": true,
  "message": "User registered successfully",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user_id": 1,
    "username": "admin"
  }
}
```

---

### 2. 用户登录

用户登录获取 JWT Token

**请求**
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "password123"
}
```

**请求参数**
- `username` (string, required) - 用户名
- `password` (string, required) - 密码

**响应**
```json
{
  "success": true,
  "message": "Login successful",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user_id": 1,
    "username": "admin"
  }
}
```

---

### 3. 用户登出

登出当前用户（删除 Redis 中的 Token）

**请求**
```http
POST /api/auth/logout
Authorization: Bearer <token>
```

**响应**
```json
{
  "success": true,
  "message": "Logged out successfully",
  "data": null
}
```

---

## 用户管理 API

> **认证要求**: 需要 JWT Token

### 1. 获取当前用户信息

获取当前登录用户的详细信息

**请求**
```http
GET /api/users/me
Authorization: Bearer <token>
```

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": {
    "id": 1,
    "username": "admin",
    "email": "admin@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 2. 获取所有用户

获取系统中所有用户列表

**请求**
```http
GET /api/users
Authorization: Bearer <token>
```

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    {
      "id": 1,
      "username": "admin",
      "email": "admin@example.com",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### 3. 获取指定用户

获取指定用户的详细信息

**请求**
```http
GET /api/users/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 用户 ID

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": {
    "id": 1,
    "username": "admin",
    "email": "admin@example.com",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 4. 更新用户信息

更新指定用户的信息

**请求**
```http
PUT /api/users/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "username": "newname",
  "email": "newemail@example.com"
}
```

**路径参数**
- `id` (integer) - 用户 ID

**请求参数**
- `username` (string, optional) - 新用户名
- `email` (string, optional) - 新邮箱

**响应**
```json
{
  "success": true,
  "message": "User updated successfully",
  "data": null
}
```

---

### 5. 删除用户

删除指定用户

**请求**
```http
DELETE /api/users/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 用户 ID

**响应**
```json
{
  "success": true,
  "message": "User deleted successfully",
  "data": null
}
```

---

### 6. 为用户分配角色

为指定用户分配一个或多个角色

**请求**
```http
POST /api/users/{id}/roles
Authorization: Bearer <token>
Content-Type: application/json

{
  "role_ids": [1, 2, 3]
}
```

**路径参数**
- `id` (integer) - 用户 ID

**请求参数**
- `role_ids` (array of integers, required) - 角色 ID 列表

**响应**
```json
{
  "success": true,
  "message": "Roles assigned successfully",
  "data": null
}
```

---

### 7. 获取用户权限

获取指定用户的所有权限

**请求**
```http
GET /api/users/{id}/permissions
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 用户 ID

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    "projects:create",
    "projects:read",
    "projects:update",
    "projects:delete"
  ]
}
```

---

## 角色管理 API

> **认证要求**: 需要 JWT Token

### 1. 创建角色

创建新角色

**请求**
```http
POST /api/roles
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "项目管理员",
  "description": "管理项目的角色"
}
```

**请求参数**
- `name` (string, required) - 角色名称
- `description` (string, optional) - 角色描述

**响应**
```json
{
  "success": true,
  "message": "Role created successfully",
  "data": {
    "id": 1
  }
}
```

---

### 2. 获取所有角色

获取系统中所有角色

**请求**
```http
GET /api/roles
Authorization: Bearer <token>
```

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    {
      "id": 1,
      "name": "超级管理员",
      "description": "拥有所有权限",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### 3. 获取指定角色

获取指定角色的详细信息

**请求**
```http
GET /api/roles/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 角色 ID

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": {
    "id": 1,
    "name": "超级管理员",
    "description": "拥有所有权限",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 4. 更新角色

更新指定角色的信息

**请求**
```http
PUT /api/roles/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "新角色名",
  "description": "新描述"
}
```

**路径参数**
- `id` (integer) - 角色 ID

**请求参数**
- `name` (string, optional) - 新角色名
- `description` (string, optional) - 新描述

**响应**
```json
{
  "success": true,
  "message": "Role updated successfully",
  "data": null
}
```

---

### 5. 删除角色

删除指定角色

**请求**
```http
DELETE /api/roles/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 角色 ID

**响应**
```json
{
  "success": true,
  "message": "Role deleted successfully",
  "data": null
}
```

---

### 6. 为角色分配权限

为指定角色分配一个或多个权限

**请求**
```http
POST /api/roles/{id}/permissions
Authorization: Bearer <token>
Content-Type: application/json

{
  "permission_ids": [1, 2, 3]
}
```

**路径参数**
- `id` (integer) - 角色 ID

**请求参数**
- `permission_ids` (array of integers, required) - 权限 ID 列表

**响应**
```json
{
  "success": true,
  "message": "Permissions assigned successfully",
  "data": null
}
```

---

## 权限管理 API

> **认证要求**: 需要 JWT Token

### 1. 创建权限

创建新权限

**请求**
```http
POST /api/permissions
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "projects:create",
  "description": "创建项目"
}
```

**请求参数**
- `name` (string, required) - 权限名称，格式：`resource:action`
- `description` (string, optional) - 权限描述

**权限命名规范**
- 格式：`资源:操作`
- 示例：`projects:create`, `users:read`, `*:*`
- 通配符：`*` 表示所有

**响应**
```json
{
  "success": true,
  "message": "Permission created successfully",
  "data": {
    "id": 1
  }
}
```

---

### 2. 获取所有权限

获取系统中所有权限

**请求**
```http
GET /api/permissions
Authorization: Bearer <token>
```

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    {
      "id": 1,
      "name": "projects:create",
      "description": "创建项目",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### 3. 获取指定权限

获取指定权限的详细信息

**请求**
```http
GET /api/permissions/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 权限 ID

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": {
    "id": 1,
    "name": "projects:create",
    "description": "创建项目",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 4. 更新权限

更新指定权限的信息

**请求**
```http
PUT /api/permissions/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "projects:update",
  "description": "更新项目"
}
```

**路径参数**
- `id` (integer) - 权限 ID

**请求参数**
- `name` (string, optional) - 新权限名
- `description` (string, optional) - 新描述

**响应**
```json
{
  "success": true,
  "message": "Permission updated successfully",
  "data": null
}
```

---

### 5. 删除权限

删除指定权限

**请求**
```http
DELETE /api/permissions/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 权限 ID

**响应**
```json
{
  "success": true,
  "message": "Permission deleted successfully",
  "data": null
}
```

---

## 部门管理 API

> **认证要求**: 需要 JWT Token

### 1. 创建部门

创建新部门

**请求**
```http
POST /api/departments
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "技术部",
  "description": "负责技术研发"
}
```

**请求参数**
- `name` (string, required) - 部门名称
- `description` (string, optional) - 部门描述

**响应**
```json
{
  "success": true,
  "message": "Department created successfully",
  "data": {
    "id": 1
  }
}
```

---

### 2. 获取所有部门

获取系统中所有部门

**请求**
```http
GET /api/departments
Authorization: Bearer <token>
```

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    {
      "id": 1,
      "name": "技术部",
      "description": "负责技术研发",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### 3. 获取指定部门

获取指定部门的详细信息

**请求**
```http
GET /api/departments/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 部门 ID

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": {
    "id": 1,
    "name": "技术部",
    "description": "负责技术研发",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 4. 更新部门

更新指定部门的信息

**请求**
```http
PUT /api/departments/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "研发部",
  "description": "新描述"
}
```

**路径参数**
- `id` (integer) - 部门 ID

**请求参数**
- `name` (string, optional) - 新部门名
- `description` (string, optional) - 新描述

**响应**
```json
{
  "success": true,
  "message": "Department updated successfully",
  "data": null
}
```

---

### 5. 删除部门

删除指定部门

**请求**
```http
DELETE /api/departments/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 部门 ID

**响应**
```json
{
  "success": true,
  "message": "Department deleted successfully",
  "data": null
}
```

---

## 员工管理 API

> **认证要求**: 需要 JWT Token

### 1. 创建员工

创建新员工

**请求**
```http
POST /api/employees
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "张三",
  "email": "zhangsan@example.com",
  "department_id": 1,
  "position": "高级工程师"
}
```

**请求参数**
- `name` (string, required) - 员工姓名
- `email` (string, required) - 邮箱
- `department_id` (integer, optional) - 部门 ID
- `position` (string, optional) - 职位

**响应**
```json
{
  "success": true,
  "message": "Employee created successfully",
  "data": {
    "id": 1
  }
}
```

---

### 2. 获取所有员工

获取系统中所有员工

**请求**
```http
GET /api/employees
Authorization: Bearer <token>
```

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    {
      "id": 1,
      "name": "张三",
      "email": "zhangsan@example.com",
      "department_id": 1,
      "position": "高级工程师",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### 3. 获取指定员工

获取指定员工的详细信息

**请求**
```http
GET /api/employees/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 员工 ID

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": {
    "id": 1,
    "name": "张三",
    "email": "zhangsan@example.com",
    "department_id": 1,
    "position": "高级工程师",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 4. 更新员工

更新指定员工的信息

**请求**
```http
PUT /api/employees/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "李四",
  "position": "技术总监"
}
```

**路径参数**
- `id` (integer) - 员工 ID

**请求参数**
- `name` (string, optional) - 新姓名
- `email` (string, optional) - 新邮箱
- `department_id` (integer, optional) - 新部门 ID
- `position` (string, optional) - 新职位

**响应**
```json
{
  "success": true,
  "message": "Employee updated successfully",
  "data": null
}
```

---

### 5. 删除员工

删除指定员工

**请求**
```http
DELETE /api/employees/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 员工 ID

**响应**
```json
{
  "success": true,
  "message": "Employee deleted successfully",
  "data": null
}
```

---

## 项目管理 API

> **认证要求**: 部分接口需要 JWT Token

### 1. 获取已发布项目（公开）

获取所有已发布的项目

**请求**
```http
GET /api/projects/published
```

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    {
      "id": 1,
      "title": "项目标题",
      "description": "项目描述",
      "status": "published",
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### 2. 创建项目（需认证）

创建新项目

**请求**
```http
POST /api/projects
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "新项目",
  "description": "项目描述",
  "content": "项目详细内容",
  "status": "draft"
}
```

**请求参数**
- `title` (string, required) - 项目标题
- `description` (string, optional) - 项目描述
- `content` (string, optional) - 项目详细内容
- `status` (string, optional) - 状态：`draft`(草稿) 或 `published`(已发布)

**响应**
```json
{
  "success": true,
  "message": "Project created successfully",
  "data": {
    "id": 1
  }
}
```

---

### 3. 获取所有项目（需认证）

获取所有项目（包括草稿）

**请求**
```http
GET /api/projects
Authorization: Bearer <token>
```

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    {
      "id": 1,
      "title": "项目标题",
      "description": "项目描述",
      "status": "draft",
      "created_by": 1,
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### 4. 获取指定项目（需认证）

获取指定项目的详细信息

**请求**
```http
GET /api/projects/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 项目 ID

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": {
    "id": 1,
    "title": "项目标题",
    "description": "项目描述",
    "content": "项目详细内容",
    "status": "published",
    "created_by": 1,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

---

### 5. 更新项目（需认证）

更新指定项目的信息

**请求**
```http
PUT /api/projects/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "新标题",
  "status": "published"
}
```

**路径参数**
- `id` (integer) - 项目 ID

**请求参数**
- `title` (string, optional) - 新标题
- `description` (string, optional) - 新描述
- `content` (string, optional) - 新内容
- `status` (string, optional) - 新状态

**响应**
```json
{
  "success": true,
  "message": "Project updated successfully",
  "data": null
}
```

---

### 6. 删除项目（需认证）

删除指定项目

**请求**
```http
DELETE /api/projects/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 项目 ID

**响应**
```json
{
  "success": true,
  "message": "Project deleted successfully",
  "data": null
}
```

---

### 7. 添加项目图片（需认证）

为项目添加图片

**请求**
```http
POST /api/projects/images
Authorization: Bearer <token>
Content-Type: application/json

{
  "project_id": 1,
  "image_url": "/uploads/image.jpg",
  "description": "图片描述",
  "sort_order": 1
}
```

**请求参数**
- `project_id` (integer, required) - 项目 ID
- `image_url` (string, required) - 图片 URL
- `description` (string, optional) - 图片描述
- `sort_order` (integer, optional) - 排序顺序

**响应**
```json
{
  "success": true,
  "message": "Image added successfully",
  "data": {
    "id": 1
  }
}
```

---

### 8. 获取项目图片（需认证）

获取指定项目的所有图片

**请求**
```http
GET /api/projects/{id}/images
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 项目 ID

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    {
      "id": 1,
      "project_id": 1,
      "image_url": "/uploads/image.jpg",
      "description": "图片描述",
      "sort_order": 1,
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### 9. 删除项目图片（需认证）

删除指定项目图片

**请求**
```http
DELETE /api/projects/images/{id}
Authorization: Bearer <token>
```

**路径参数**
- `id` (integer) - 图片 ID

**响应**
```json
{
  "success": true,
  "message": "Image deleted successfully",
  "data": null
}
```

---

## 文件上传 API

> **认证要求**: 需要 JWT Token

### 1. 上传文件

上传文件到服务器

**请求**
```http
POST /api/upload
Authorization: Bearer <token>
Content-Type: multipart/form-data

file: <binary_file_data>
```

**请求参数**
- `file` (file, required) - 要上传的文件

**支持的文件类型**
- 图片：jpg, jpeg, png, gif, webp
- 文档：pdf, doc, docx
- 其他：根据配置

**响应**
```json
{
  "success": true,
  "message": "File uploaded successfully",
  "data": {
    "url": "/uploads/2024/01/01/filename.jpg",
    "filename": "filename.jpg",
    "size": 102400
  }
}
```

---

## 管理员 API

> **认证要求**: 需要 JWT Token + 超级管理员权限 (`*:*`)

所有管理员 API 都在 `/api/admin` 路径下，需要超级管理员权限才能访问。

### 项目管理（管理员）

#### 1. 创建项目

```http
POST /api/admin/projects
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "新项目",
  "description": "项目描述",
  "content": "项目内容",
  "status": "draft"
}
```

#### 2. 获取所有项目

```http
GET /api/admin/projects
Authorization: Bearer <token>
```

#### 3. 获取指定项目

```http
GET /api/admin/projects/{id}
Authorization: Bearer <token>
```

#### 4. 更新项目

```http
PUT /api/admin/projects/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "新标题",
  "status": "published"
}
```

#### 5. 删除项目

```http
DELETE /api/admin/projects/{id}
Authorization: Bearer <token>
```

#### 6. 添加项目图片

```http
POST /api/admin/projects/images
Authorization: Bearer <token>
Content-Type: application/json

{
  "project_id": 1,
  "image_url": "/uploads/image.jpg",
  "description": "图片描述"
}
```

#### 7. 获取项目图片

```http
GET /api/admin/projects/{id}/images
Authorization: Bearer <token>
```

#### 8. 删除项目图片

```http
DELETE /api/admin/projects/images/{id}
Authorization: Bearer <token>
```

---

### 用户管理（管理员）

所有用户管理接口与上述用户管理 API 相同，但在 `/api/admin/users` 路径下。

---

### 角色管理（管理员）

所有角色管理接口与上述角色管理 API 相同，但在 `/api/admin/roles` 路径下。

---

### 权限管理（管理员）

所有权限管理接口与上述权限管理 API 相同，但在 `/api/admin/permissions` 路径下。

---

### 部门管理（管理员）

所有部门管理接口与上述部门管理 API 相同，但在 `/api/admin/departments` 路径下。

---

### 员工管理（管理员）

所有员工管理接口与上述员工管理 API 相同，但在 `/api/admin/employees` 路径下。

---

## 错误代码说明

### 常见错误

| 错误信息 | 说明 | 解决方法 |
|---------|------|---------|
| `Invalid token` | Token 无效或已过期 | 重新登录获取新 Token |
| `Authentication required` | 未提供 Token | 在请求头中添加 Authorization |
| `Super admin access required` | 需要超级管理员权限 | 使用具有管理员权限的账号 |
| `Permission required: resource:action` | 缺少特定权限 | 联系管理员分配权限 |
| `Database error` | 数据库错误 | 检查数据库连接和数据 |
| `User not found` | 用户不存在 | 检查用户 ID 是否正确 |
| `Duplicate entry` | 数据重复 | 检查唯一字段（如用户名、邮箱） |

---

## 使用示例

### JavaScript (Fetch API)

```javascript
// 登录
const login = async () => {
  const response = await fetch('http://localhost:8080/api/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      username: 'admin',
      password: 'password123'
    })
  });
  
  const data = await response.json();
  if (data.success) {
    localStorage.setItem('token', data.data.token);
  }
};

// 获取用户信息
const getUserInfo = async () => {
  const token = localStorage.getItem('token');
  const response = await fetch('http://localhost:8080/api/users/me', {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  
  const data = await response.json();
  console.log(data);
};
```

### cURL

```bash
# 登录
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password123"}'

# 获取用户信息
curl -X GET http://localhost:8080/api/users/me \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"

# 创建项目
curl -X POST http://localhost:8080/api/projects \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -H "Content-Type: application/json" \
  -d '{"title":"新项目","description":"项目描述","status":"draft"}'
```

---

## 附录

### 权限列表

系统预定义的权限：

| 权限名称 | 描述 |
|---------|------|
| `*:*` | 超级管理员，所有权限 |
| `projects:create` | 创建项目 |
| `projects:read` | 读取项目 |
| `projects:update` | 更新项目 |
| `projects:delete` | 删除项目 |
| `users:create` | 创建用户 |
| `users:read` | 读取用户 |
| `users:update` | 更新用户 |
| `users:delete` | 删除用户 |
| `roles:manage` | 管理角色 |
| `permissions:manage` | 管理权限 |

### 项目状态

| 状态 | 描述 |
|-----|------|
| `draft` | 草稿，未发布 |
| `published` | 已发布，公开可见 |
| `archived` | 已归档 |

---

**文档版本**: 1.0  
**最后更新**: 2024-06-15  
**联系方式**: 如有问题，请联系开发团队
