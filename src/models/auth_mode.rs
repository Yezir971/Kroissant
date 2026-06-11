//! Mode d'affichage de la page d'authentification.

#[derive(Copy, Clone)]
pub enum AuthMode {
    Register,
    Login,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authmode_values() {
        let mode = AuthMode::Register;
        match mode {
            AuthMode::Register => assert!(true),
            AuthMode::Login => assert!(false),
        }
    }
}
