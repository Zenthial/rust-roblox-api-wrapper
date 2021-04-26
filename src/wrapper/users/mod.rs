#![allow(dead_code)]
mod structs;

pub fn get_userid_from_username(username: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let url = format!("https://api.roblox.com/users/get-by-username?username={}", username);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::UserInfoResponse>()?;
    Ok(resp.Id)
}

pub fn get_username_from_userid(userid: i64) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.roblox.com/users/{}", userid);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::UserInfoResponse>()?;
    Ok(resp.Username)
}

pub fn get_users_groups(userid: i64) -> Result<Vec<structs::GroupRoleData>, Box<dyn std::error::Error>> {
    let url = format!("https://groups.roblox.com/v2/users/{}/groups/roles", userid);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::GroupInfoResponse>()?;
    Ok(resp.data)
}

pub fn get_user_info(userid: i64) -> Result<structs::UsersResponse, Box<dyn std::error::Error>> {
    let url = format!("https://users.roblox.com/v1/users/{}", userid);
    let resp = reqwest::blocking::get(url)?
        .json::<structs::UsersResponse>()?;
    Ok(resp)
}