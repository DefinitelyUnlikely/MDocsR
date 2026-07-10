use crate::features::users::repository::UserRepository;

pub struct UserService<U> {
    user_repo: U,
}

impl<U> UserService<U>
where
    U: UserRepository,
{
    pub fn new(user_repo: U) -> Self {
        UserService { user_repo }
    }
}
