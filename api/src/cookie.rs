const COOKIE_TOKEN_NAME: &'static str = "TOKEN";

fn set() {
    cookies.add(Cookie::new(COOKIE_TOKEN_NAME, "Token caca"));
}

fn get(
    cookies: Cookies
) {

}

fn remove() {}