use askama::Template;

#[derive(Template)]
#[template(path = "mod.rs.txt")]
pub struct ModRs {
    pub hogee: &'static str,
}
