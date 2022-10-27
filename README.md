# mezmo-rs

Use Mezmo platform with Rust.

# Example

```rust
use mezmo::Logger;

fn main() {
    let logger = Logger::new("YOUR_API_KEY_HERE".to_string(), "tag1,tag2,tag3".to_string());
    logger.log("this is a log".to_string(), "INFO".to_string());    
}
```
