use worker::{Request, RouteContext};

pub struct Bot {
    request: Request,
    ctx: RouteContext<()>,
}

impl Bot {
    pub fn new(request: Request, ctx: RouteContext<()>) -> Bot {
        Bot{request, ctx}
    }

    pub async fn handle_interaction(&mut self) -> Result<InteractionResponse, HttpError> {

    }
}
