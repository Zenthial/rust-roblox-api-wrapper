mod wrapper;

fn main() {
    // let user_id = wrapper::users::get_userid_from_username("tomspell").unwrap();
    // let username = wrapper::users::get_username_from_userid(9345226).unwrap();
    // let users_groups = wrapper::users::get_users_groups(9345226).unwrap();
    //let user_info = wrapper::users::get_user_info(9345226).unwrap();
    // for info in users_groups.iter() {
    //     println!("{:?}", info);
    // }

    let username_history = wrapper::users::get_usernames_with_limit(9345226, wrapper::users::enums::Limits::TwentyFive).unwrap();
    println!("{:?}", username_history);
}