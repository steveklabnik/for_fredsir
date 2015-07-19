use nickel::{Nickel, HttpRouter};

pub fn build_routes(server: &mut Nickel) {
    server.get("/bar", middleware!("This is the /bar handler"));
    server.get("/user/:userid", middleware! { |request|
        format!("This is user: {:?}", request.param("userid"))
    });
    server.get("/a/*/d", middleware!("matches /a/b/d but not /a/b/c/d"));
    server.get("/a/**/d", middleware!("This matches /a/b/d and also /a/b/c/d"));
}
