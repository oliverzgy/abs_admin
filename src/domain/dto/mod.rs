use crate::domain::domain::LoginCheck;
use serde::{Deserialize, Serialize};

/// IdDTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdDTO {
    pub id: Option<String>,
}

/// 资源分页DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

/// 资源添加DTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResAddDTO {
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
}

/// 资源修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResEditDTO {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub permission: Option<String>,
    pub path: Option<String>,
}

/// 登陆
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInDTO {
    pub account: String,
    pub password: String,
    //验证码，可用是短信验证码，图片验证码,二维码验证码...
    pub vcode: String,
}

/// 登陆
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAddDTO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
}

/// 用户修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserEditDTO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
}

/// 用户分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub account: Option<String>,
    pub name: Option<String>,
}

/// 角色分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RolePageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}

/// 角色添加
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleAddDTO {
    pub name: Option<String>,
    pub auths: Vec<String>,
    //父id(可空)
    pub parent_id: Option<String>,
}

/// 角色修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoleEditDTO {
    pub id: Option<String>,
    pub name: Option<String>,
    pub auths: Vec<String>,
    pub parent_id: Option<String>,
}

/// 用户角色添加
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRoleAddDTO {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
}

/// 用户角色修改
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRoleEditDTO {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
}

/// 用户角色分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRolePageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
}

/// 验证码
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CatpchaDTO {
    pub account: Option<String>,
}

/// 角色资源添加
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleResAddDTO {
    pub role: RoleAddDTO,
    //资源id集合
    pub resource_ids: Option<Vec<String>>,
}

/// 角色资源添加
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleResUpdateDTO {
    pub role: RoleEditDTO,
    //资源id集合
    pub resource_ids: Option<Vec<String>>,
}

/// 角色分页
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SysRoleResPageDTO {
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub name: Option<String>,
}
