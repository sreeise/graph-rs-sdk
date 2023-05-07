pub enum Display {
    /// The Authorization Server SHOULD display the authentication and consent UI
    /// consistent with a full User Agent page view. If the display parameter is not
    /// specified, this is the default display mode.
    Page,
    /// The Authorization Server SHOULD display the authentication and consent UI consistent with
    /// a popup User Agent window. The popup User Agent window should be of an appropriate size
    /// for a login-focused dialog and should not obscure the entire window that it is popping
    /// up over.
    Popup,
    /// The Authorization Server SHOULD display the authentication and consent UI consistent with
    /// a device that leverages a touch interface.
    Touch,
    /// The Authorization Server SHOULD display the authentication and consent UI consistent with
    /// a "feature phone" type display.
    Wap,
}
