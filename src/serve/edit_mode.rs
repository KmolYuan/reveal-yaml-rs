use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, StreamHandler};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, SystemTime};

fn file_date(path: &str) -> SystemTime {
    std::fs::metadata(path).unwrap().modified().unwrap()
}

struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<Event> for Ws {
    type Result = ();

    fn handle(&mut self, _msg: Event, ctx: &mut Self::Context) {
        ctx.text("changed!");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Ws {
    fn handle(&mut self, _msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {}
}

pub(super) struct Monitor {
    last: SystemTime,
    project: String,
    listeners: Vec<Addr<Ws>>,
}

impl Monitor {
    pub(super) fn new(project: String) -> Addr<Self> {
        Self {
            last: file_date(&project),
            project,
            listeners: vec![],
        }
        .start()
    }
}

impl Actor for Monitor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(Duration::from_millis(500), |act, _| {
            let last = file_date(&act.project);
            if last != act.last {
                // Broadcast
                for listener in &act.listeners {
                    listener.do_send(Event);
                }
                act.last = last;
            }
        });
    }
}

impl Handler<Client> for Monitor {
    type Result = ();

    fn handle(&mut self, msg: Client, _ctx: &mut Context<Self>) {
        self.listeners.push(msg.0);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct Event;

#[derive(Message)]
#[rtype(result = "()")]
struct Client(Addr<Ws>);

#[get("/ws/")]
pub(super) async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<Monitor>>,
) -> Result<HttpResponse, Error> {
    let (addr, res) = ws::WsResponseBuilder::new(Ws, &req, stream).start_with_addr()?;
    data.do_send(Client(addr));
    Ok(res)
}
