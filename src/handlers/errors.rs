use rocket::response::content;
use rocket::Request;

#[error(404)]
fn not_found(req: &Request) -> content::HTML<String> {
    content::HTML(format!("<p>Page {} not found<p>", req.uri()))
}

