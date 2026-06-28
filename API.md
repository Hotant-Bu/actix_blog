# API 接口文档

## 基础信息

- 基础URL: `http://localhost:8080/api`
- 认证方式: JWT Bearer Token
- 内容类型: `application/json`

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
  "message": "错误信息"
}
```

## 认证接口

### 1. 用户注册

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

**响应**
```json
{
  "success": true,
  "message": "User registered successfully",
  "data": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
    "user_id": 1,
    "username": "admin"
  }
}
```

### 2. 用户登录

**请求**
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "password123"
}
```

**响应**
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

---

## 用户管理接口

> 所有接口需要认证，请在请求头添加: `Authorization: Bearer <token>`

### 1. 获取当前用户信息

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
    "is_active": true,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}
```

### 2. 获取所有用户

**请求**
```http
GET /api/users
Authorization: Bearer <token>
```

### 3. 获取指定用户

**请求**
```http
GET /api/users/{id}
Authorization: Bearer <token>
```

### 4. 更新用户

**请求**
```http
PUT /api/users/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "username": "newname",
  "email": "newemail@example.com",
  "is_active": true
}
```

### 5. 删除用户

**请求**
```http
DELETE /api/users/{id}
Authorization: Bearer <token>
```

### 6. 为用户分配角色

**请求**
```http
POST /api/users/{id}/roles
Authorization: Bearer <token>
Content-Type: application/json

{
  "role_ids": [1, 2, 3]
}
```

### 7. 获取用户权限

**请求**
```http
GET /api/users/{id}/permissions
Authorization: Bearer <token>
```

**响应**
```json
{
  "success": true,
  "message": "Success",
  "data": [
    "查看用户",
    "创建用户",
    "编辑用户"
  ]
}
```

---

## 角色管理接口

### 1. 创建角色

**请求**
```http
POST /api/roles
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "编辑",
  "description": "内容编辑人员"
}
```

### 2. 获取所有角色

**请求**
```http
GET /api/roles
Authorization: Bearer <token>
```

### 3. 获取指定角色

**请求**
```http
GET /api/roles/{id}
Authorization: Bearer <token>
```

### 4. 更新角色

**请求**
```http
PUT /api/roles/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "高级编辑",
  "description": "高级内容编辑人员"
}
```

### 5. 删除角色

**请求**
```http
DELETE /api/roles/{id}
Authorization: Bearer <token>
```

### 6. 为角色分配权限

**请求**
```http
POST /api/roles/{id}/permissions
Authorization: Bearer <token>
Content-Type: application/json

{
  "permission_ids": [1, 2, 3, 4, 5]
}
```

### 7. 获取角色权限

**请求**
```http
GET /api/roles/{id}/permissions
Authorization: Bearer <token>
```

---

## 权限管理接口

### 1. 创建权限

**请求**
```http
POST /api/permissions
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "查看报表",
  "resource": "reports",
  "action": "read",
  "description": "查看系统报表"
}
```

### 2. 获取所有权限

**请求**
```http
GET /api/permissions
Authorization: Bearer <token>
```

### 3. 获取指定权限

**请求**
```http
GET /api/permissions/{id}
Authorization: Bearer <token>
```

### 4. 更新权限

**请求**
```http
PUT /api/permissions/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "查看高级报表",
  "resource": "reports",
  "action": "read_advanced",
  "description": "查看高级系统报表"
}
```

### 5. 删除权限

**请求**
```http
DELETE /api/permissions/{id}
Authorization: Bearer <token>
```

---

## 部门管理接口

### 1. 创建部门

**请求**
```http
POST /api/departments
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "技术部",
  "parent_id": null,
  "description": "技术研发部门"
}
```

### 2. 获取所有部门

**请求**
```http
GET /api/departments
Authorization: Bearer <token>
```

### 3. 获取指定部门

**请求**
```http
GET /api/departments/{id}
Authorization: Bearer <token>
```

### 4. 更新部门

**请求**
```http
PUT /api/departments/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "技术研发部",
  "parent_id": 1,
  "description": "负责产品研发"
}
```

### 5. 删除部门

**请求**
```http
DELETE /api/departments/{id}
Authorization: Bearer <token>
```

---

## 员工管理接口

### 1. 创建员工

**请求**
```http
POST /api/employees
Authorization: Bearer <token>
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

### 2. 获取所有员工

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
      "user_id": 1,
      "username": "admin",
      "email": "admin@example.com",
      "department_id": 1,
      "department_name": "技术部",
      "employee_number": "EMP001",
      "full_name": "张三",
      "position": "高级工程师",
      "phone": "13800138000",
      "hire_date": "2024-01-01T00:00:00Z",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

### 3. 获取指定员工

**请求**
```http
GET /api/employees/{id}
Authorization: Bearer <token>
```

### 4. 更新员工

**请求**
```http
PUT /api/employees/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "department_id": 2,
  "position": "资深工程师",
  "phone": "13900139000"
}
```

### 5. 删除员工

**请求**
```http
DELETE /api/employees/{id}
Authorization: Bearer <token>
```

---

## 项目案例管理接口

### 1. 创建项目

**请求**
```http
POST /api/projects
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "某大型电商平台",
  "description": "为客户打造的大型电商平台，包含商品管理、订单系统、支付系统等...",
  "cover_image": "/uploads/xxx.jpg",
  "video_url": "https://example.com/video.mp4",
  "status": "published",
  "display_order": 1
}
```

### 2. 获取所有项目（需认证）

**请求**
```http
GET /api/projects
Authorization: Bearer <token>
```

### 3. 获取已发布项目（公开接口）

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
      "title": "某大型电商平台",
      "description": "为客户打造的大型电商平台...",
      "cover_image": "/uploads/xxx.jpg",
      "video_url": "https://example.com/video.mp4",
      "status": "published",
      "display_order": 1,
      "created_by": 1,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

### 4. 获取指定项目

**请求**
```http
GET /api/projects/{id}
Authorization: Bearer <token>
```

### 5. 更新项目

**请求**
```http
PUT /api/projects/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "某大型电商平台（更新）",
  "status": "published",
  "display_order": 2
}
```

### 6. 删除项目

**请求**
```http
DELETE /api/projects/{id}
Authorization: Bearer <token>
```

### 7. 添加项目图片

**请求**
```http
POST /api/projects/images
Authorization: Bearer <token>
Content-Type: application/json

{
  "project_id": 1,
  "image_url": "/uploads/xxx.jpg",
  "caption": "项目首页截图",
  "display_order": 1
}
```

### 8. 获取项目图片

**请求**
```http
GET /api/projects/{id}/images
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
      "project_id": 1,
      "image_url": "/uploads/xxx.jpg",
      "caption": "项目首页截图",
      "display_order": 1,
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

### 9. 删除项目图片

**请求**
```http
DELETE /api/projects/images/{id}
Authorization: Bearer <token>
```

---

## 文件上传接口

### 1. 上传文件

**请求**
```http
POST /api/upload
Authorization: Bearer <token>
Content-Type: multipart/form-data

file: <binary>
```

**响应**
```json
{
  "success": true,
  "message": "File uploaded successfully",
  "data": {
    "url": "/uploads/550e8400-e29b-41d4-a716-446655440000.jpg",
    "filename": "550e8400-e29b-41d4-a716-446655440000.jpg"
  }
}
```

**说明**
- 支持的文件类型：图片、视频等
- 最大文件大小：50MB（可在 .env 中配置）
- 文件会自动重命名为 UUID 格式
- 上传后的文件可通过 `/uploads/{filename}` 访问

---

## 错误码说明

| HTTP状态码 | 说明 |
|-----------|------|
| 200 | 请求成功 |
| 400 | 请求参数错误 |
| 401 | 未授权（Token无效或过期） |
| 403 | 禁止访问（权限不足） |
| 404 | 资源不存在 |
| 500 | 服务器内部错误 |

## 使用示例

### cURL 示例

```bash
# 登录
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password123"}'

# 获取用户列表（需要替换 TOKEN）
curl -X GET http://localhost:8080/api/users \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"

# 上传文件
curl -X POST http://localhost:8080/api/upload \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -F "file=@/path/to/image.jpg"
```

### JavaScript 示例

```javascript
// 登录
const login = async () => {
  const response = await fetch('http://localhost:8080/api/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      username: 'admin',
      password: 'password123'
    })
  });
  const data = await response.json();
  return data.data.token;
};

// 获取用户列表
const getUsers = async (token) => {
  const response = await fetch('http://localhost:8080/api/users', {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  return await response.json();
};

// 上传文件
const uploadFile = async (token, file) => {
  const formData = new FormData();
  formData.append('file', file);
  
  const response = await fetch('http://localhost:8080/api/upload', {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`
    },
    body: formData
  });
  return await response.json();
};
```
