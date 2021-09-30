use anyhow::Result;

struct OrderId(String);

impl OrderId {
    pub fn new(value: String) -> Result<Self> {
        if value.is_empty() {
            Err(anyhow!("OrderId must not be empty"))
        } else if value.len() > 50 {
            Err(anyhow!("OrderId must not be more than 50 chars"))
        } else {
            Ok(Self(value))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
