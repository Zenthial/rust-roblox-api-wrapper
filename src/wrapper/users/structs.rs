#![allow(non_snake_case)]

use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub struct UserInfoResponse {
    pub Id: i64,
    pub Username: String,
    pub AvatarUri: Option<String>,
    pub AvatarFinal: bool,
    pub IsOnline: bool
}

#[derive(Deserialize, Debug)]
pub struct GroupInfo {
    pub id: i64,
    pub name: String,
    pub memberCount: i64
}

#[derive(Deserialize, Debug)]
pub struct RoleInfo {
    pub id: i64,
    pub name: String,
    pub rank: i64
}

#[derive(Deserialize, Debug)]
pub struct GroupRoleData {
    pub group: GroupInfo,
    pub role: RoleInfo
}

#[derive(Deserialize, Debug)]
pub struct GroupInfoResponse {
    pub data: Vec<GroupRoleData>
}

#[derive(Deserialize, Debug)]
pub struct UsersResponse {
    pub description: String,
    pub created: String,
    pub isBanned: bool,
    pub id: i64,
    pub name: String,
    pub displayName: String
}

#[derive(Deserialize, Debug)]
pub struct UsernameData {
    pub name: String
}

#[derive(Deserialize, Debug)]
pub struct UsernameResponse {
    pub previousPageCursor: Option<String>,
    pub nextPageCursor: Option<String>,
    pub data: Vec<UsernameData>
}

#[derive(Deserialize, Debug)]
pub struct CollectiblesData {
    pub userAssetId: i64,
    pub serialNumber: Option<i64>,
    pub assetId: i64,
    pub name: String,
    pub recentAveragePrice: i64,
    pub originalPrice: Option<i64>,
    pub assetStock: Option<i64>,
    pub buildersClubMembershipType: i64
}

#[derive(Deserialize, Debug)]
pub struct CollectiblesResponse {
    pub previousPageCursor: Option<String>,
    pub nextPageCursor: Option<String>,
    pub data: Vec<CollectiblesData>
}