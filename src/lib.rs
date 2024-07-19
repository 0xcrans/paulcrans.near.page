use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::collections::LookupMap;
use near_sdk::serde_json::json;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pages: LookupMap<String, String>,
    owner: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        let mut this = Self {
            pages: LookupMap::new(b"p"),
            owner,
        };
        let index_content = include_str!("../index.html").to_string();
        this.pages.insert(&"".to_string(), &index_content);
        this
    }

    pub fn add_page(&mut self, path: String, content: String) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only the owner can add pages");
        self.pages.insert(&path, &content);
    }

    pub fn get_page(&self, path: String) -> Option<String> {
        self.pages.get(&path)
    }

    pub fn web4_get(&self, request: near_sdk::serde_json::Value) -> near_sdk::serde_json::Value {
        let path = request["path"].as_str().unwrap_or("").trim_start_matches('/');
        
        if let Some(content) = self.pages.get(&path.to_string()) {
            json!({
                "contentType": "text/html; charset=utf-8",
                "body": content,
            })
        } else {
            json!({
                "statusCode": 404,
                "body": "404 Not Found",
            })
        }
    }
}
