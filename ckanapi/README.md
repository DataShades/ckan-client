DEV not ready for use

Client for the CKAN API.

```no_run
use ckanapi::{CKAN, Action, Params};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct StatusShow {
    site_title: String,
}

fn main() {
    let mut client = CKAN::from("https://demo.ckan.org");
    let action = Action::from("status_show");

    let response: Option<StatusShow> = client.invoke(action, Params::Empty).extract();
}
```
