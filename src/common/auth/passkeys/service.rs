use crate::features::users::db::UserRepository;

pub struct PasskeyService<U> {
    user_repo: U
}

impl<U> PasskeyService<U> 
where
    U: UserRepository
{
    pub fn new(user_repo: U) -> Self {
        PasskeyService { user_repo }
    }
}
