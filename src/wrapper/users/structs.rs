#![allow(non_snake_case)]

use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct UserInfoResponse {
    pub Id: i64,
    pub Username: String,
    pub AvatarUri: Option<String>,
    pub AvatarFinal: bool,
    pub IsOnline: bool
}

impl fmt::Debug for UserInfoResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("UserInfoResponse")
            .field("Id", &self.Id)
            .field("Username", &self.Username)
            .field("AvatarUri", &self.AvatarUri)
            .field("AvatarFinal", &self.AvatarFinal)
            .field("IsOnline", &self.IsOnline)
            .finish()
    }
}


#[derive(Deserialize)]
pub struct GroupInfo {
    pub id: i64,
    pub name: String,
    pub memberCount: i64
}

impl fmt::Debug for GroupInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("GroupInfo")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("memberCount", &self.memberCount)
            .finish()
    }
}

#[derive(Deserialize)]
pub struct RoleInfo {
    pub id: i64,
    pub name: String,
    pub rank: i64
}

impl fmt::Debug for RoleInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("RoleInfo")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("rank", &self.rank)
            .finish()
    }
}

#[derive(Deserialize)]
pub struct GroupRoleData {
    pub group: GroupInfo,
    pub role: RoleInfo
}

impl fmt::Debug for GroupRoleData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("GroupRoleData")
            .field("group", &self.group)
            .field("role", &self.role)
            .finish()
    }
}

#[derive(Deserialize)]
pub struct GroupInfoResponse {
    pub data: Vec<GroupRoleData>
}

impl fmt::Debug for GroupInfoResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("GroupInfoResponse")
            .field("data", &self.data)
            .finish()
    }
}

#[derive(Deserialize)]
pub struct UsersResponse {
    description: String,
    created: String,
    isBanned: bool,
    id: i64,
    name: String,
    displayName: String
}

impl fmt::Debug for UsersResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("UsersResponse")
            .field("description", &self.description)
            .field("created", &self.created)
            .field("isBanned", &self.isBanned)
            .field("id", &self.id)
            .field("name", &self.name)
            .field("displayName", &self.displayName)
            .finish()
    }
}

#[derive(Deserialize)]
pub struct UsernameData {
    name: String
}

impl fmt::Debug for UsernameData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("UsernameData")
            .field("name", &self.name)
            .finish()
    }
}

#[derive(Deserialize)]
pub struct UsernameResponse {
    previousPageCursor: Option<String>,
    nextPageCursor: Option<String>,
    data: Vec<UsernameData>
}

impl fmt::Debug for UsernameResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("UsernameResponse")
            .field("previousPageCursor", &self.previousPageCursor)
            .field("nextPageCursor", &self.nextPageCursor)
            .field("data", &self.data)
            .finish()
    }
}