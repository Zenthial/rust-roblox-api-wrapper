mod wrapper;

//9345226
fn main() {
    let user_id = wrapper::users::get_userid_from_username("tomspell").unwrap();
    let username = wrapper::users::get_username_from_userid(9345226).unwrap();
    let users_groups = wrapper::users::get_users_groups(9345226).unwrap();
    let user_info = wrapper::users::get_user_info(9345226).unwrap();
    let collectibles = wrapper::users::get_collectibles(9345226).unwrap();

    for collectible in collectibles.data.iter() {
        println!("{:?}", collectible);
    }
    for info in users_groups.iter() {
        println!("{:?}", info);
    }
    
    println!("{}, {}, {:?}", user_id, username, user_info);
}