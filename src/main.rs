mod api;

fn main() {
    // let user_id = api::users::get_userid_from_username("tomspell").unwrap();
    // let username = api::users::get_username_from_userid(9345226).unwrap();
    // let users_groups = api::users::get_users_groups(9345226).unwrap();
    // for info in users_groups.iter() {
    //     println!("{:?}", info);
    // }

    let user_info = api::users::get_user_info(9345226).unwrap();
    println!("{:?}", user_info);
}