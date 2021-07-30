// client/src/lib.rs

use seed::{prelude::*, *};
use shared::{ListPersons, Person};

struct Model {
    pub data: ListPersons,
    pub person: Person,
    pub new_person: Person,
    pub person_lastname: String,
    pub person_firstname: String,
}

impl Default for Model {
    fn default() -> Self {
        let data = ListPersons::default();
        let person = Person::default();
        let new_person = Person::default();
        let person_lastname = "".into();
        let person_firstname = "".into();

        Self {
            data,
            person,
            new_person,
            person_lastname,
            person_firstname,
        }
    }
}

#[derive(Clone, Debug)]
enum Msg {
    FetchData,
    Fetched(ListPersons),
    Click(usize),
    AddPerson,
    ModifyPerson,
    DeletePerson,
    NewFirstName(String),
    NewLastName(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchData => {
            orders.skip();
            orders.perform_cmd(async {
                let url = "https://localhost:8001/json_list";

                let response = fetch(url).await.expect("HTTP reqwest failed !");

                let list_persons = response
                    .check_status()
                    .expect("status check failed !")
                    .json::<ListPersons>()
                    .await
                    .expect("deserialisation failed !");

                Msg::Fetched(list_persons)
            });
        }

        Msg::Fetched(data) => {
            model.data = data;
        }

        //lorsqu'on clique sur une rangée de la table, les données de la Person
        // affichées dans la rangée sont placées dans la variable "person"
        // du modèle.
        //
        Msg::Click(posit) => {
            model.person = model.data.list_persons[posit].clone();
            log!(
                "Click : la variable person dans le model contient : ",
                model.person
            );
            model.person_firstname = (&model.person.prenom).to_string();
            model.person_lastname = (&model.person.nom).to_string();
        }

        // donne une nouvelle valeur String à la variable
        // person_firstname du modèle
        // suit les modifications de l'input "first_name"
        //
        Msg::NewFirstName(string) => {
            model.person_firstname = string;
        }

        // donne une nouvelle valeur String à la variable
        // person_lastname du modèle
        // suit les modifications de l'input "last_name"
        //
        Msg::NewLastName(string) => {
            model.person_lastname = string;
        }

        // ajoute une nouvelle personne à la liste des personnes de model.data
        // sur base des variables person_lastname et person_firstname du modèle
        // enregistre la nouvelle personne dans la base de donnée locale
        //
        Msg::AddPerson => {
            /*
            // on vérifie si la liste des personnes est vide ou non
            // si elle est vide, l'id de la première personne est 1
            if model.data.list_persons.is_empty() {
                let person_id = 1;
                model.new_person = Person::new(person_id as i32, model.person_firstname.clone(),
                                               model.person_lastname.clone());
            }
            else {
                // il faut utiliser l'id de la dernière personne enregistrée dans le vecteur
                // pour ne pas avoir deux fois le même id si on se base uniquement sur la
                // longueur du vecteur et qu'on a déjà effacé un enregistrement

                // variable qui enregistre la longueur du vecteur
                let len = model.data.list_persons.len();

                // on récupère la dernière personne enregistrée
                // la longueur du vecteur moins un ; le vecteur commence à 0
                let pers = &model.data.list_persons[len - 1];

                // on prend l'id de la dernière personne puis on lui ajoute 1
                // pour créer l'id de la nouvelle personne à ajouter
                let mut person_id = pers.id;
                person_id += 1;
                model.new_person = Person::new(person_id as i32, model.person_firstname.clone(),
                                               model.person_lastname.clone());
            }

            // si on utilise une indexmap :
            //model.data.list_persons.insert(person_id, model.new_person.clone());

            model.data.list_persons.push(model.new_person.clone());

            // on sauvegarde dans le local_store
            //model.data.store(); */
        }

        // Modifie une personne dans la DB locale
        //
        Msg::ModifyPerson => {
            // on va chercher les données dans les inputs et on les passe à model.person
            model.person.prenom = model.person_firstname.clone();
            model.person.nom = model.person_lastname.clone();
            /*
            // on prend la variable id de model person (i32 en usize pour data.list_person)
            // on enlève 1 à x parce que le vec commence à 0
            let mut x:usize = model.person.id as usize;
            x -= 1;

            // on remplace la Person dans le vec data.list.person par celle du model
            model.data.list_persons[x] = model.person.clone();
            model.data.store();
            */
        }
        // on efface la personne sélectionnée
        //
        Msg::DeletePerson => {
            // on prend la variable id de model person (i32 en usize pour data.list_person)
            // on enlève 1 à x parce que le vec commence à 0
            /*let mut x:usize = model.person.id as usize;
            x -= 1;

            model.data.list_persons.remove(x);
            model.data.store();*/
        }
    }
}

///
/// Montre la liste des Personnes
/// sur base du Vec<Person> compris dans la structure Data
/// (peut être chargée par le local_store
///
fn show_persons_as_rows(model: &Model) -> Vec<Node<Msg>> {
    let mut i = 0;
    let mut vec_node_pers = Vec::new();
    for pers in &model.data.list_persons {
        vec_node_pers.push(person_item(pers, i));
        i += 1;
    }
    vec_node_pers

    /*
        // on transforme l'indexmap en Vec<Person>
        //let mut vec_pers: Vec<Person> = Vec::new();

        // deux variables pour for qui viennent d'indexmap
        // un underscore pour la clé qui n'est pas utile dans le vecteur
       for (_,pers) in model.data.list_persons {
            vec_pers.push(pers);
        }
        vec_pers
    */
}

///
/// Affiche une rangée dans une table
/// avec les données d'une personne
///
/// on lui passe une Person et une position (usize)
/// lorsqu'on clique sur la rangée, on passe le message Click(position)
///
fn person_item(item: &Person , posit: usize) -> Node<Msg> {
    // le style de la rangée
    let row_style = style!["background-color" => "lightblue",
                                    "color" => "black",
                                    "font-style" => "italic",
                                    "font-size" => "150%",
                                    St::Border=> "1px solid black",];
    // la rangée elle-même
    // lorsqu'on clique sur la rangée, on passe le message Click(position)
    tr![
        td![&row_style, { item.nom.clone() }],
        td![&row_style, { item.prenom.clone() }],
        simple_ev(Ev::Click, Msg::Click(posit)),
    ]
}

///
/// la vue
///
fn view(model: &Model) -> impl IntoNodes<Msg> {
    // les différents styles utilisés
    let header_style = style!["background-color" => "yellow",
                                    "color" => "red",
                                    St::Border=> "1px solid black",
                                    St::FontSize => "120%",
                                    St::FontStyle => "bold"];
    let caption_style = style!["color"=> "green",
                                     "font-style" => "bold",
                                     "font-size" => "160%" ];
    let title_style = style![St::Color => "red",
                                   St::FontSize => "200%",
                                   St::FontStyle => "bold",
                                   St::FontStyle => "underline"];
    let table_style = style![St::AlignSelf => "center"];
    let button_style = style![St::BackgroundColor => "yellow",
                                    St::FontSize => "120%",
                                    St::FontStyle => "bold"];
    let input_style = style![St::BackgroundColor => "lightgreen",
                                   St::FontSize => "120%",
                                   St::FontStyle => "bold"];

    let persons = show_persons_as_rows(model);

    vec![
        div![
            style![
                St::Display => "flex",
                St::FlexDirection => "column";
                St::TextAlign => "center"
            ],
            h1![&title_style, "Gestion des Personnes"],
            table![
                &table_style,
                caption![&caption_style, "Liste des Personnes"],
                tr![
                    th![&header_style, "Nom : "],
                    th![&header_style, "Prénom : "],
                ],
                persons,
            ]
        ],
        div![
            form![
                attrs! { At::Id => "form"},
                input![
                    &input_style,
                    attrs! {
                        At::Id =>"input_last_name",
                        At::Value => model.person_lastname,
                        At::AutoFocus => AtValue::None,
                    },
                    input_ev(Ev::Input, Msg::NewLastName),
                ],
                input![
                    &input_style,
                    attrs! {
                        At::Id =>"input_first_name",
                        At::Value => model.person_firstname,
                        At::AutoFocus => AtValue::None,
                    },
                    input_ev(Ev::Input, Msg::NewFirstName)
                ],
                button![&button_style, "Add", simple_ev(Ev::Click, Msg::AddPerson),],
                button![
                    &button_style,
                    "Modify",
                    simple_ev(Ev::Click, Msg::ModifyPerson),
                ],
                button![
                    &button_style,
                    "Delete",
                    simple_ev(Ev::Click, Msg::DeletePerson),
                ],
            ],
            label![format!(
                "model.person : {:?},{:?},{:?}",
                model.person.id, model.person.nom, model.person.prenom
            )],
            label![format!(
                "model.person_lastname : {:?}, model.person_firstname : {:?}",
                model.person_lastname, model.person_firstname
            )]
        ],
    ]
}

///
/// Donne le modèle à utiliser
/// remplace le modèle par défaut
///
fn after_mount(_url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    let mut model = Model {
        data: ListPersons::default(),
        person: Person::default(),
        new_person: Person::default(),
        person_lastname: "".to_string(),
        person_firstname: "".to_string(),
    };
    /*
       // s'il y a des données dans le local_store
       // on les charge dans le Model
       if let Some(data) = local_store::load_data() {
           model.data = data;
       }


       // sinon on charge les données par défaut
       else {
           //let vec_pers= populate_list_persons();
           let vec_pers= Vec::new();
           model.data = Some(ListPersons{
               list_persons: vec_pers,
           })
       }
    */

    orders.send_msg(Msg::FetchData);

    log!(model.data.list_persons);
    AfterMount::new(model)
}

///
/// la fonction principale
///
#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
