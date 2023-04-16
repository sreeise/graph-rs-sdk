/// Indicates the type of user interaction that is required. Valid values are login, none,
/// consent, and select_account.
///
/// - **prompt=login** forces the user to enter their credentials on that request, negating single-sign on.
/// - **prompt=none** is the opposite. It ensures that the user isn't presented with any interactive prompt.
///     If the request can't be completed silently by using single-sign on, the Microsoft identity platform returns an interaction_required error.
/// - **prompt=consent** triggers the OAuth consent dialog after the user signs in, asking the user to
///     grant permissions to the app.
/// - **prompt=select_account** interrupts single sign-on providing account selection experience
///     listing all the accounts either in session or any remembered account or an option to choose to use a different account altogether.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Prompt {
    #[default]
    None,
    Login,
    Consent,
    SelectAccount,
}

impl AsRef<str> for Prompt {
    fn as_ref(&self) -> &'static str {
        match self {
            Prompt::None => "none",
            Prompt::Login => "login",
            Prompt::Consent => "consent",
            Prompt::SelectAccount => "select_account",
        }
    }
}
