use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use std::sync::{Arc, Mutex};

pub static ALLOWED_DOMAINS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| HashSet::from(["http://localhost:8080"]));

static SESSION_APPS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    HashMap::from([
        (
            "http://localhost:8080".to_string(),
            "sso_consumer".to_string(),
        ),
        // (
        //     "http://consumertwo.ankuranand.in:3030".to_string(),
        //     "simple_sso_consumer".to_string(),
        // ),
    ])
});

pub type Shared<T> = Arc<Mutex<T>>;

pub struct TokenCache {
    session_app_cache: HashMap<String, HashMap<String, bool>>,
    session_user_cache: HashMap<String, String>,
    intrim_token_cache: HashMap<String, (String, String)>,
}

impl TokenCache {
    pub fn build_shared() -> Shared<Self> {
        let session_app_cache = HashMap::<String, HashMap<String, bool>>::new();
        let session_user_cache = HashMap::<String, String>::new();
        let intrim_token_cache = HashMap::<String, (String, String)>::new();

        Arc::new(Mutex::new(Self {
            session_app_cache,
            session_user_cache,
            intrim_token_cache,
        }))
    }

    pub fn store_application_in_cache(
        &mut self,
        origin: String,
        session_id: String,
        intrim_token: String,
    ) {
        let origin_name = SESSION_APPS.get(&origin).unwrap();
        match self.session_app_cache.get_mut(&session_id) {
            None => {
                let origin_map = HashMap::from([(origin_name.clone(), true)]);

                self.session_app_cache
                    .insert(session_id.clone(), origin_map);
            }
            Some(allowed_origin) => {
                allowed_origin.insert(origin_name.clone(), true);
            }
        }

        self.intrim_token_cache
            .insert(intrim_token, (session_id, origin_name.clone()));
    }

    pub fn store_user_in_cache(&mut self, session_id: String, username: String) {
        self.session_user_cache.insert(session_id, username);
    }

    pub fn get_sso_token_details(&self, sso_token: &String) -> Option<(String, String)> {
        self.intrim_token_cache
            .get(sso_token)
            .map(|(a, b)| (a.clone(), b.clone()))
    }

    pub fn is_sso_allowed(&self, session_id: &String, app_name: String) -> bool {
        match self.session_app_cache.get(session_id) {
            None=> return false,
            Some(allowed_apps)=>{
                match allowed_apps.get(&app_name) {
                    None=> return false,
                    Some(allowed)=>return allowed.to_owned()
                }
            }
        }
    }

    pub fn get_username(&self, session_id: &String)->String{
        self.session_user_cache.get(session_id).unwrap().to_owned()
    }

    pub fn remove_intrim_token(&mut self, sso_token: &String){
        self.intrim_token_cache.remove(sso_token);
    }
}
