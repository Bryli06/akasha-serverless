use bot::Bot;
use commands::get_commands;
use discord::RegisteredCommand;
use worker::*;

mod utils;
mod bot;
mod handler;
mod discord;
mod commands;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .post_async("/", |req, ctx| async move {
            let mut bot = Bot::new(req, ctx);

            match bot.handle_interaction().await {
                Ok(result) => {
                    worker::console_log!("Response: {}", serde_json::to_string_pretty(&result).unwrap());
                    Response::from_json(&result)
                }
                Err(e) => {
                    worker::console_log!("Error: {}", e.to_string());
                    Response::error(e.to_string(), e.status as u16)
                }
            }
        })
        .post_async("/register", |_, ctx| async move {
            let commands = get_commands();
            let mut register: Vec<RegisteredCommand> = Vec::new();

            for command in commands.iter() {
                register.push( RegisteredCommand {
                    name: command.name(),
                    description: command.description(),
                    options: command.options()
                }); 
            }

            let client = reqwest::Client::new();
            let app_id = ctx.var("DISCORD_APPLICATION_ID")?.to_string();
            let token = ctx.var("DISCORD_TOKEN")?.to_string();

            console_log!("App id: {}\nToken: {}", app_id, token);

            let api_url = format!("https://discord.com/api/v10/applications/{}/commands", app_id);

            let json = serde_json::to_string(&register)?;
            console_log!("{}", json);

            let response = client
                .put(api_url)
                .body(json)
                .header("Authorization", format!("Bot {}", token))
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            console_log!("Response trying to update commands: {}", response);

            Response::ok(&response)
        })

        .run(req, env)
        .await
}
