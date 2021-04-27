#![allow(dead_code)]
use std::error::Error;

pub mod structs;
pub mod enums;

pub fn get_userid_from_username(username: &str) -> Result<i64, Box<dyn Error>> {
    let url = format!("https://api.roblox.com/users/get-by-username?username={}", username);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::UserInfoResponse>()?;
    Ok(resp.Id)
}

pub fn get_username_from_userid(userid: i64) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.roblox.com/users/{}", userid);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::UserInfoResponse>()?;
    Ok(resp.Username)
}

pub fn get_users_groups(userid: i64) -> Result<Vec<structs::GroupRoleData>, Box<dyn Error>> {
    let url = format!("https://groups.roblox.com/v2/users/{}/groups/roles", userid);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::GroupInfoResponse>()?;
    Ok(resp.data)
}

pub fn get_user_info(userid: i64) -> Result<structs::UsersResponse, Box<dyn Error>> {
    let url = format!("https://users.roblox.com/v1/users/{}", userid);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::UsersResponse>()?;
    Ok(resp)
}

pub fn get_usernames(userid: i64) -> Result<structs::UsernameResponse, Box<dyn Error>> {
    let url = format!("https://users.roblox.com/v1/users/{}/username-history", userid);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::UsernameResponse>()?;
    Ok(resp)
}

/// i64 userid - id of the player
/// Limits limit - the specified limit of how many usernames to download. Takes an enum as only specific limit values are allowed
pub fn get_usernames_with_limit(userid: i64, limit: enums::Limits) -> Result<structs::UsernameResponse, Box<dyn Error>> {
    let url = format!("https://users.roblox.com/v1/users/{}/username-history?limit={}", userid, limit);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::UsernameResponse>()?;
    Ok(resp)
}

pub fn get_usernames_with_cursor(userid: i64, cursor: String) -> Result<structs::UsernameResponse, Box<dyn Error>> {
    let url = format!("https://users.roblox.com/v1/users/{}/username-history?limit=10&cursor={}", userid, cursor);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::UsernameResponse>()?;
    Ok(resp)
}

pub fn get_usernames_with_cursor_and_limit(userid: i64, cursor: String, limit: enums::Limits) -> Result<structs::UsernameResponse, Box<dyn Error>> {
    let url = format!("https://users.roblox.com/v1/users/{}/username-history?limit={}&cursor={}", userid, limit, cursor);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::UsernameResponse>()?;
    Ok(resp)
}

pub fn get_collectibles(userid: i64) -> Result<structs::CollectiblesResponse, Box<dyn Error>> {
    let url = format!("https://inventory.roblox.com/v1/users/{}/assets/collectibles", userid);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::CollectiblesResponse>()?;
    Ok(resp)
}