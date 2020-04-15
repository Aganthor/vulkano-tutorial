pub struct RawVertex {
    pub vals: [f32; 3],
}

impl RawVertex {
    pub fn new(input: &str) -> RawVertex {
        let items = input.split_whitespace();
        let mut content:Vec<f32> = Vec::new();
        for item in items {
            content.push(item.parse().unwrap());
        }
        if content.len() == 2 {
            content.push(0.0);
        }
        RawVertex { vals: [*content.get(0).unwrap(), *content.get(1).unwrap(), *content.get(2).unwrap()] }
    }
}