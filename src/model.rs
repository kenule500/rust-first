use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use tower_http::classify::GrpcCode::Ok;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)] // Changed to Deserialize since it comes FROM the user
pub struct TicketDTO {
    pub title: String,
}

// region:    --- Model Controller
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }

    // Combined into the same impl block
    pub async fn create_ticket(&self, ticket_dto: TicketDTO) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        
        // Fixed ID logic: use the current length as the ID (0-based)
        let id = store.len() as u64; 
        
        let ticket = Ticket {
            id,
            title: ticket_dto.title,
        };
        
        store.push(Some(ticket.clone()));
        
        Ok(ticket)
    }

    // Fixed typo: list_tickects -> list_tickets
    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();
        
        // This is perfect. It skips None and clones the Some(Ticket)
        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}