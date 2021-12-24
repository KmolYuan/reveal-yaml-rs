use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::{
    fs::metadata,
    time::{Duration, SystemTime},
};

const INTERVAL: Duration = Duration::from_millis(500);

fn file_date(path: &str) -> Duration {
    metadata(path)
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
}

struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<ServerEvent> for Ws {
    type Result = ();

    fn handle(&mut self, msg: ServerEvent, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, _item: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {}
}

pub(super) struct ServerMonitor {
    last: Duration,
    project: String,
    listeners: Vec<Addr<Ws>>,
}

impl ServerMonitor {
    pub(super) fn new(project: String) -> Addr<Self> {
        Self {
            last: file_date(&project),
            project,
            listeners: vec![],
        }
        .start()
    }
}

impl Actor for ServerMonitor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(INTERVAL, |act, _| {
            let last = file_date(&act.project);
            if last != act.last {
                // Broadcast
                for l in &act.listeners {
                    l.do_send(ServerEvent("changed!".to_string()));
                }
                act.last = last;
            }
        });
    }
}

impl Handler<RegisterClient> for ServerMonitor {
    type Result = ();

    fn handle(&mut self, msg: RegisterClient, _ctx: &mut Context<Self>) {
        self.listeners.push(msg.0);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct ServerEvent(String);

#[derive(Message)]
#[rtype(result = "()")]
struct RegisterClient(Addr<Ws>);

#[get("/ws/")]
pub(super) async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<ServerMonitor>>,
) -> Result<HttpResponse, Error> {
    let (addr, res) = ws::WsResponseBuilder::new(Ws, &req, stream).start_with_addr()?;
    data.do_send(RegisterClient(addr));
    Ok(res)
}
