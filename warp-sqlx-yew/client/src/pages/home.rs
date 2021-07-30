// src/pages/person_list.rs

use crate::api;
use crate::components::PersonCard;
use shared::Person;

use anyhow::Error;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::prelude::*;

pub struct PersonList {
    state : State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

struct State{
    persons: Vec<Person>,
    get_persons_error: Option<Error>,
    get_persons_loaded: bool,
}

pub enum Msg {
    GetPersons,
    GetPersonsSuccess(Vec<Person>),
    GetPersonsError(Error),
    EditPerson(i32),
    DeletePerson(i32),
}

impl Component for PersonList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let persons = vec![];
        link.send_message(Msg::GetPersons);
        Self{
            state: State {
                persons,
                get_persons_error: None,
                get_persons_loaded: false
            },
            link,
            task: None
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetPersons => {
                self.state.get_persons_loaded = false;
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<Vec<Person>>|{
                            let(_, Json(data)) = response.into_parts();
                            match data {
                                Ok(persons) => Msg::GetPersonsSuccess(persons),
                                Err(err) => Msg::GetPersonsError(err),
                            }
                        });
                self.task = Some(api::get_persons(handler));
                //self.task = Some(api::get_persons_warp(handler));
                true
            }
            Msg::GetPersonsSuccess(persons) => {
                self.state.persons = persons;
                self.state.get_persons_loaded = true;
                true
            }
            Msg::GetPersonsError(error) => {
                self.state.get_persons_error = Some(error);
                self.state.get_persons_loaded = true;
                true
            }
            Msg::EditPerson(id) => {
                unimplemented!()
            }
            Msg::DeletePerson(id) => {
                unimplemented!()
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let persons: Vec<Html> = self
            .state
            .persons
            .iter()
            .map(|person: &Person| {
                let person_id = person.id;
                html! {
                  <PersonCard person={person}
                  on_edit=self.link.callback(move |_| Msg::EditPerson(person_id))
                  on_delete=self.link.callback(move|_| Msg::DeletePerson(person_id))/>
                }
            })
            .collect();

        if !self.state.get_persons_loaded {
            return html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading ..."}</div>
                </div>
            }
        } else if let Some(_) = self.state.get_persons_error {
            return html! {
               <div>
                 <span>{"Error loading persons! :("}</span>
               </div>
            }
        } else {
            return  html! {
                <div class="person_card_list">{persons}</div>
            }
        }
    }
}



