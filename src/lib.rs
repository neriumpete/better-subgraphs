use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Gravatar {
    id: String,
    owner: String,
    display_name: String,
    image_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    number: u64,
    hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    address: String,
    block: Block,
    event_type: String,
    params: serde_json::Value,
}

#[wasm_bindgen]
pub struct Subgraph {
    gravatars: Vec<Gravatar>,
    last_processed_block: u64,
}

#[wasm_bindgen]
impl Subgraph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Subgraph {
            gravatars: Vec::new(),
            last_processed_block: 0,
        }
    }

    // This simulates how Graph nodes call subgraphs
    #[wasm_bindgen]
    pub fn handle_event(&mut self, event_json: &str) -> Result<(), JsValue> {
        let event: Event = serde_json::from_str(event_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse event: {}", e)))?;

        // Update last processed block
        self.last_processed_block = event.block.number;

        match event.event_type.as_str() {
            "NewGravatar" => {
                let params = event.params;
                let id = params["id"].as_str().ok_or_else(|| JsValue::from_str("Missing id"))?;
                let owner = params["owner"].as_str().ok_or_else(|| JsValue::from_str("Missing owner"))?;
                let display_name = params["displayName"].as_str().ok_or_else(|| JsValue::from_str("Missing displayName"))?;
                let image_url = params["imageUrl"].as_str().ok_or_else(|| JsValue::from_str("Missing imageUrl"))?;

                self.handle_gravatar_created(id, owner, display_name, image_url)?;
            },
            "UpdatedGravatar" => {
                let params = event.params;
                let id = params["id"].as_str().ok_or_else(|| JsValue::from_str("Missing id"))?;
                let display_name = params["displayName"].as_str().ok_or_else(|| JsValue::from_str("Missing displayName"))?;
                let image_url = params["imageUrl"].as_str().ok_or_else(|| JsValue::from_str("Missing imageUrl"))?;

                self.handle_gravatar_updated(id, display_name, image_url)?;
            },
            _ => return Err(JsValue::from_str(&format!("Unknown event type: {}", event.event_type))),
        }

        web_sys::console::log_1(&JsValue::from_str(&format!(
            "Processed {} event at block {}",
            event.event_type,
            event.block.number
        )));

        Ok(())
    }

    fn handle_gravatar_created(&mut self, id: &str, owner: &str, display_name: &str, image_url: &str) -> Result<(), JsValue> {
        let gravatar = Gravatar {
            id: id.to_string(),
            owner: owner.to_string(),
            display_name: display_name.to_string(),
            image_url: image_url.to_string(),
        };
        
        self.gravatars.push(gravatar);
        web_sys::console::log_1(&JsValue::from_str(&format!("Created Gravatar for owner: {}", owner)));
        
        Ok(())
    }

    fn handle_gravatar_updated(&mut self, id: &str, display_name: &str, image_url: &str) -> Result<(), JsValue> {
        if let Some(gravatar) = self.gravatars.iter_mut().find(|g| g.id == id) {
            gravatar.display_name = display_name.to_string();
            gravatar.image_url = image_url.to_string();
            web_sys::console::log_1(&JsValue::from_str(&format!("Updated Gravatar: {}", id)));
        } else {
            return Err(JsValue::from_str(&format!("Gravatar not found: {}", id)));
        }
        
        Ok(())
    }

    #[wasm_bindgen]
    pub fn get_gravatar(&self, id: &str) -> Result<String, JsValue> {
        if let Some(gravatar) = self.gravatars.iter().find(|g| g.id == id) {
            serde_json::to_string(gravatar)
                .map_err(|e| JsValue::from_str(&format!("Failed to serialize gravatar: {}", e)))
        } else {
            Err(JsValue::from_str(&format!("Gravatar not found: {}", id)))
        }
    }

    #[wasm_bindgen]
    pub fn get_gravatars_by_owner(&self, owner: &str) -> Result<String, JsValue> {
        let owner_gravatars: Vec<&Gravatar> = self.gravatars
            .iter()
            .filter(|g| g.owner == owner)
            .collect();
            
        serde_json::to_string(&owner_gravatars)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize gravatars: {}", e)))
    }

    #[wasm_bindgen]
    pub fn get_last_processed_block(&self) -> u64 {
        self.last_processed_block
    }
}

#[wasm_bindgen]
pub fn init() {
    web_sys::console::log_1(&JsValue::from_str("Gravatar Subgraph initialized"));
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
