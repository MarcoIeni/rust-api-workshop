use {
    std::{ops::Deref, time::Duration},
    wiremock::{
        matchers::{method, path, query_param},
        Mock, MockServer, ResponseTemplate,
    },
    yoda_taller::swapi::Person,
};

pub struct SwapiMock {
    server: MockServer,
}

impl Deref for SwapiMock {
    type Target = MockServer;

    fn deref(&self) -> &Self::Target {
        &self.server
    }
}

impl SwapiMock {
    pub async fn start() -> Self {
        Self {
            server: MockServer::start().await,
        }
    }
}

impl SwapiMock {
    pub async fn mock_people_query(&self, search: &str, body: serde_json::Value) {
        self.mock_people_query_response(search, ResponseTemplate::new(200).set_body_json(body))
            .await
    }

    pub async fn mock_people_query_with_delay(
        &self,
        search: &str,
        body: serde_json::Value,
        delay: Duration,
    ) {
        self.mock_people_query_response(
            search,
            ResponseTemplate::new(200)
                .set_body_json(body)
                .set_delay(delay),
        )
        .await
    }

    async fn mock_people_query_response(&self, search: &str, response: ResponseTemplate) {
        Mock::given(method("GET"))
            .and(path("/api/people/"))
            .and(query_param("search", search))
            .respond_with(response)
            .named("mock people query")
            .expect(1)
            .mount(&self.server)
            .await;
    }
}

pub fn person_query_result(person: &Person) -> serde_json::Value {
    serde_json::json!( {
        "count": 1,
        "next": null,
        "previous": null,
        "results": [
            {
                "birth_year": "896BBY",
                "created": "2014-12-15T12:26:01.042000Z",
                "edited": "2014-12-20T21:17:50.345000Z",
                "eye_color": "brown",
                "films": [
                    "http://127.0.1.1:9992/api/films/2/",
                    "http://127.0.1.1:9992/api/films/3/",
                    "http://127.0.1.1:9992/api/films/4/",
                    "http://127.0.1.1:9992/api/films/5/",
                    "http://127.0.1.1:9992/api/films/6/"
                ],
                "gender": "male",
                "hair_color": "white",
                "height": person.height,
                "homeworld": "http://127.0.1.1:9992/api/planets/28/",
                "mass": "17",
                "name": person.name,
                "skin_color": "green",
                "species": [
                    "http://127.0.1.1:9992/api/species/6/"
                ],
                "starships": [],
                "url": "http://127.0.1.1:9992/api/people/20/",
                "vehicles": []
            }
        ]
    })
}

pub fn empty_query_result() -> serde_json::Value {
    serde_json::json!( {
        "count": 0,
        "next": null,
        "previous": null,
        "results": []
    })
}
