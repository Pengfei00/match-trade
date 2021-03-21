use actix::{Actor, StreamHandler};
use actix_web::web::Bytes;
use actix_web_actors::ws;
struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(self.text(text)),
            Ok(ws::Message::Binary(bin)) => ctx.binary(self.binary(bin)),
            _ => (),
        }
    }
}

impl Ws {
    fn text(&self,msg: String) -> String {
        "".to_string()
    }

    fn binary(&self,msg:Bytes)->Bytes {
        Bytes::from("")
    }
}