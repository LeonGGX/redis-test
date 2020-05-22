// client/src/lib.rs
#[macro_use]
extern crate seed;

use seed::prelude::*;
use seed::{fetch, Request, Method};

use futures::{Future as RealFuture, TryFutureExt};
use futures_util::future::FutureExt;


use shared::models::person::{Person, ListPersons};
use seed::util::{log,};


struct Model {
   persons: Vec<Person>,
}

/*
fn get_persons_json() -> impl RealFuture<Item = Msg, Error = Msg> {
    let url = "http://127.0.0.1:8088/json";
    Request::new(url)
        .method(Method::Get)
        .fetch_json_data(|_| Msg::GetPersons)
        .map(Msg::Replace)
        .map_err(Msg::OnFetchErr)
}
*/

#[derive(Clone)]
enum Msg {
    /*GetPersons,
    Replace(Person),
    OnServerResponse(ServerResponse),
    OnFetchErr(JsValue),*/
    FetchedPersons(fetch::ResponseDataResult<ListPersons>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedPersons(Ok(mut result)) => {
            model.persons.clear();
            model.persons.append(&mut result.list_pers);
        }
        Msg::FetchedPersons(Err(reason)) => {
            log!(format!("Error fetching: {:?}", reason));
        }
    /*
        Msg::GetPersons => {
            orders.skip().perform_cmd(get_persons_json());
        }

        Msg::OnServerResponse(result) => {
            log!(format!("Response: {:?}", result));
            orders.skip();
        }

        Msg::OnFetchErr(err) => {
            error!(format!("Fetch error: {:?}", err));
            orders.skip();
        }
    */
    }
}

fn view(model: &Model) -> impl View<Msg> {
    init();
    let persons: Vec::<Node<Msg>> = model.persons.iter().map(|p| {
        li![{p.nom.clone()}]
    }).collect();

    h1![
        {"Persons"},
        ul![
            persons,
        ],
    ]
}

fn init() -> Model {
    Request::new("http://localhost:8088/json").fetch_json_data(Msg::FetchedPersons);
    Model { persons: vec![] }
}


/*
fn fetch_drills() -> impl Future<Item = Msg, Error = Msg> {
    Request::new("http://localhost:8088/json/").fetch_json_data(Msg::FetchedTasks)
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(fetch_drills());
    Model {
        persons: vec![],
    }
}
*/


// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view);//.build_and_start();
}
