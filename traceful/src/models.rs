use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use storeful::{Context, ContextValue};
use typed_builder::TypedBuilder;
use ulid::Ulid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trace {
    pub name: String,
    pub trace_id: Ulid,
    pub context: Context,
    pub spans: Vec<Span>,
}

impl Trace {
    pub fn new(name: &str, context: Context) -> Self {
        Trace {
            name: name.into(),
            trace_id: Ulid::new(),
            context,
            spans: Vec::new(),
        }
    }

    pub fn add_span(&mut self, mut span: Span) {
        self.spans.push(span);
    }

    pub fn with_span(mut self, mut span: Span) -> Self {
        self.add_span(span);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, TypedBuilder)]
pub struct Span {
    pub name: String,
    #[builder(default = Ulid::new())]
    pub span_id: Ulid,
    #[builder(default)]
    pub parent_span_id: Option<Ulid>,
    #[builder(default)]
    pub context: Context,
    pub start_time: DateTime<Utc>,
    #[builder(default = chrono::Utc::now())]
    pub end_time: DateTime<Utc>,
    #[builder(default)]
    pub events: Vec<Event>,
    #[builder(default)]
    pub children: Vec<Span>,
}

impl Span {
    pub fn add_event(&mut self, name: &str, context: Context) {
        self.events.push(Event::new(name.into(), context));
    }

    pub fn with_event(mut self, name: &str, context: Context) -> Self {
        self.add_event(name, context);
        self
    }

    pub fn add_context(&mut self, key: &str, value: &str) {
        self.context.add_value(key, value);
    }

    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.add_context(key, value);
        self
    }

    pub fn add_child(&mut self, mut span: Span) {
        span.parent_span_id = Some(self.span_id);
        self.children.push(span);
    }

    pub fn with_child(mut self, mut span: Span) -> Self {
        span.parent_span_id = Some(self.span_id);
        self.add_child(span);
        self
    }

    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            span_id: Ulid::new(),
            parent_span_id: None,
            context: Context::default(),
            start_time: chrono::Utc::now(),
            end_time: chrono::Utc::now(),
            events: Vec::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub event_id: Ulid,
    pub name: String,
    pub context: Context,
    pub timestamp: DateTime<Utc>,
}

impl Event {
    pub fn new(name: String, context: Context) -> Self {
        Event {
            event_id: Ulid::new(),
            name,
            context,
            timestamp: chrono::Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Create the root trace with top-level attributes in the context.
        let mut trace = Trace::new("GET /api/user/profile", Context::default());

        // Main span for the HTTP request handling
        let mut main_span = Span::builder()
            .name("GET /api/user/profile".into())
            .start_time("2024-11-07T14:23:10.000Z".parse::<DateTime<Utc>>().unwrap())
            .end_time("2024-11-07T14:23:10.200Z".parse::<DateTime<Utc>>().unwrap())
            .build()
            .with_context("http.method", "GET")
            .with_context("http.url", "https://example.com/api/user/profile")
            .with_context("http.status_code", "200")
            .with_context("service.name", "API Gateway")
            .with_event("request_received", {
                let mut context = Context::default();
                context.add_value("user.id", "12345");
                context.add_value("ip_address", "192.168.1.10");
                context
            })
            .with_event("forwarding_request", {
                let mut context = Context::default();
                context.add_value("next.service", "Auth Service");
                context
            });

        // Child span: Auth Service - Validate User
        let auth_span = Span::builder()
            .name("Auth Service - Validate User".into())
            .start_time("2024-11-07T14:23:10.015Z".parse::<DateTime<Utc>>().unwrap())
            .end_time("2024-11-07T14:23:10.050Z".parse::<DateTime<Utc>>().unwrap())
            .build()
            .with_context("service.name", "Auth Service")
            .with_context("auth.method", "Token")
            .with_context("auth.result", "success")
            .with_context("user.id", "12345")
            .with_event("token_verification_start", Context::default())
            .with_event("token_verification_end", {
                let mut context = Context::default();
                context.add_value("verification_result", "valid");
                context
            });

        // Child span: User Service - Fetch Profile
        let user_service_span = Span::builder()
            .name("User Service - Fetch Profile".into())
            .start_time("2024-11-07T14:23:10.060Z".parse::<DateTime<Utc>>().unwrap())
            .end_time("2024-11-07T14:23:10.180Z".parse::<DateTime<Utc>>().unwrap())
            .build()
            .with_context("service.name", "User Service")
            .with_context("user.id", "12345")
            .with_context("db.statement", "SELECT * FROM users WHERE id = ?")
            .with_context("db.type", "sql")
            .with_context("db.instance", "user_db")
            .with_event("db_query_start", {
                let mut context = Context::default();
                context.add_value("query", "SELECT * FROM users WHERE id = ?");
                context.add_value("query_parameters", "[\"12345\"]");
                context
            })
            .with_event("db_query_end", {
                let mut context = Context::default();
                context.add_value("rows_returned", "1");
                context
            })
            .with_event("profile_data_serialization", {
                let mut context = Context::default();
                context.add_value("data_size_bytes", "2048");
                context
            });

        // Child span: API Gateway - Send Response
        let send_response_span = Span::builder()
            .name("API Gateway - Send Response".into())
            .start_time("2024-11-07T14:23:10.190Z".parse::<DateTime<Utc>>().unwrap())
            .end_time("2024-11-07T14:23:10.200Z".parse::<DateTime<Utc>>().unwrap())
            .build()
            .with_context("http.status_code", "200")
            .with_context("response.size_bytes", "2048")
            .with_context("service.name", "API Gateway")
            .with_event("response_serialization_start", Context::default())
            .with_event("response_sent", Context::default());

        // Nest child spans under the main span
        main_span.add_child(auth_span);
        main_span.add_child(user_service_span);
        main_span.add_child(send_response_span);

        // Add the main span to the trace
        trace.add_span(main_span);
    }
}
