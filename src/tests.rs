#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn working_userid() {
        let user_id = wrapper::users::get_userid_from_username("tomspell").unwrap();
        assert_eq!(user_id, 9345226);
    }

    #[test]
    fn working_username() {
        let username = wrapper::users::get_username_from_userid(9345226).unwrap();
        assert_eq!(username, "tomspell");
    }

    // #[test]
    // fn server_okay() {
    //     wrapper::users::get_username_from_userid_raw_response(9345226);
    // }
}