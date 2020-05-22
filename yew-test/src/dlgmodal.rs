use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Callback,};
use stdweb::unstable::TryInto;
use stdweb::web::{event::IEvent, Element, FormData};

use crate::person::{Person, PersonFormData, PersonValidationErr};

use yew::services::{
    ConsoleService
};

///
/// DlgModal est la fenêtre DlgModale
///

#[derive(Properties, Clone, PartialEq)]
pub struct DlgModalProperties {
    #[props(required)]
    pub person: Person,
    #[props(required)]
    pub visible: bool,
    #[props(required)]
    pub on_close: Callback<bool>,
    #[props(required)]
    pub on_save: Callback<Person>
}

pub struct DlgModal {
    props: DlgModalProperties,
    error: Option<Vec<PersonValidationErr>>,
    link: ComponentLink<Self>,
}

pub enum DlgModalMsg {
    HideDlgModal,
    Save(FormData)
}

impl Component for DlgModal {
    type Message = DlgModalMsg;
    type Properties = DlgModalProperties;

    fn create(prop: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props: prop,
            error: None,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut console = ConsoleService::new();

        match msg {
            DlgModalMsg::HideDlgModal => {
                self.props.visible = false;
                self.props.on_close.emit(true);
                true
            }

            DlgModalMsg::Save(form_data) => {
                let form_data: PersonFormData = form_data.into();
                let valid = PersonFormData::validate(&form_data);

                match valid {
                    Ok(_v) => {
                        self.props.visible = false;
                        /*self.props.on_save.emit(Person {
                            id: self.props.person.id,
                            last_name: form_data.last_name,
                            first_name: form_data.first_name,
                            ..Default::default()
                        });*/
                        let pers = Person {
                            id: self.props.person.id,
                            last_name: form_data.last_name,
                            first_name: form_data.first_name,
                            ..Default::default()
                        };

                        console.log(format!("{:?} saved", pers).as_ref());
                    },
                    Err(e) => {
                        self.error = Some(e)
                    }
                }

                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.person = props.person;
        self.props.visible = props.visible;

        true
    }

    fn view(&self) -> Html {

        let visible = if self.props.visible { "is-active" } else { "" };
        let on_click = self.link.callback(|_i| DlgModalMsg::HideDlgModal);
        let on_save = self.link.callback(|e| DlgModalMsg::Save(e));
        let on_close = self.link.callback(|_i: usize| DlgModalMsg::HideDlgModal);
        let form_on_submit = self.link.callback(|e: SubmitEvent| {
            e.prevent_default();
            let form_element: Element = e.target().unwrap().try_into().unwrap();
            DlgModalMsg::Save(FormData::from_element(&form_element).unwrap())
        });

        let error = |e: &PersonValidationErr| {
            match e {
                PersonValidationErr::InvalidLastName => html! {
          <div>
            {"Last Name is required"}
          </div>
        },
                PersonValidationErr::InvalidFirstName => html! {
          <div>
            {"First Name ist required"}
          </div>
        }
            }
        };

        let errors = match self.error.as_ref() {
            None => {
                html! {}
            }

            Some(errors) => {
                html! {
          <div class="notification is-danger">
            {for errors.iter().map(error)}
          </div>
        }
            }
        };

        html! {
          <div class=("DlgModal", visible)>
            <div class="DlgModal-background"></div>
            <div class="DlgModal-card">
              <form onsubmit=form_on_submit>
                <header class="DlgModal-card-head">
                  <p class="DlgModal-card-title">{"New Item"}</p>
                  <a onclick=&on_click class="delete" aria-label="close"></a>
                </header>
                <section class="DlgModal-card-body">
                  {errors}
                  <div class="field">
                    <label class="label">{"Last Name"}</label>
                    <div class="control">
                        <input value=&self.props.person.last_name
                                name="last_name"
                                class="input"
                                autocomplete="off" />
                    </div>
                  </div>

                  <div class="field">
                    <label class="label">{"First Name"}</label>
                    <p class="control has-icons-left has-icons-right">
                      <input value=&self.props.person.first_name
                             name="first_name"
                             class="input"
                             autocomplete="off" />
                      <span class="icon is-small is-left">
                        <i class="icon ion-md-cash"></i>
                      </span>
                    </p>
                  </div>
                </section>
                <footer class="DlgModal-card-foot">
                  <button type="submit" class="button is-info">{"Save"}</button>
                  <a onclick=&on_click class="button">{"Cancel"}</a>
                </footer>
              </form>
            </div>
          </div>
        }
    }
}