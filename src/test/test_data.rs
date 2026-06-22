use crate::model::model::OrderStatus;
use rand::prelude::ThreadRng;
use rand::RngExt;
use serde_json::{json, Value};

pub const AUTH_HEADER_ADMIN: &str = "Basic YWRtaW46YWRtaW4xMjM=";
pub const AUTH_HEADER_WAITER: &str = "Basic Sm9objpKb2huMTIz";
pub const AUTH_HEADER_COOK: &str = "Basic TWlrZTpNaWtlMTIz";

pub fn create_order_body() -> Value {
    let mut rng: ThreadRng = rand::rng();
    let item1: i32 = rng.random_range(1..10);
    let item2: i32 = rng.random_range(1..10);
    let item3: i32 = rng.random_range(1..10);
    let json = json!({
        "table_number": 1,
        "menu_item_ids": [item1, item2, item3]
    });
    json
}

pub fn add_order_items_body() -> Value {
    let mut rng: ThreadRng = rand::rng();
    let item1: i32 = rng.random_range(5..15);
    let item2: i32 = rng.random_range(5..15);
    let item3: i32 = rng.random_range(5..15);
    let json = json!({
        "menu_position_ids": [item1, item2, item3]
    });
    json
}

pub fn create_change_status_body(status: OrderStatus) -> Value {
    let status_body = json!({
        "status": status
    });
    status_body
}

pub fn create_menu_item_body() -> Value {
    let json = json!({
        "dish_title": "some_dish_title",
        "cost": 5000,
        "time_to_cook": 300
    });
    json
}

pub fn patch_menu_item_body() -> Value {
    let json = json!({
        "cost": 400,
        "time_to_cook": 4500,
        "is_available": false
    });
    json
}

pub const DEFAULT_MENU_RESPONSE: &str = r#"{"menu":[{"id":38,"dish_title":"Apple Pie","cost":400,"time_to_cook":1200,"is_available":true},{"id":30,"dish_title":"Beef Tacos (3 pcs)","cost":750,"time_to_cook":1080,"is_available":true},{"id":52,"dish_title":"Beef Wellington","cost":2800,"time_to_cook":3000,"is_available":true},{"id":19,"dish_title":"Bibimbap","cost":800,"time_to_cook":1500,"is_available":true},{"id":39,"dish_title":"Brownie with Ice Cream","cost":480,"time_to_cook":900,"is_available":true},{"id":2,"dish_title":"Bruschetta","cost":550,"time_to_cook":900,"is_available":true},{"id":50,"dish_title":"Cappuccino","cost":250,"time_to_cook":420,"is_available":true},{"id":23,"dish_title":"Cheeseburger Deluxe","cost":850,"time_to_cook":1320,"is_available":true},{"id":36,"dish_title":"Cheesecake","cost":450,"time_to_cook":900,"is_available":true},{"id":29,"dish_title":"Chicken Burrito","cost":700,"time_to_cook":1200,"is_available":true},{"id":27,"dish_title":"Chicken Caesar Salad","cost":650,"time_to_cook":900,"is_available":true},{"id":16,"dish_title":"Chicken Fried Rice","cost":650,"time_to_cook":1080,"is_available":true},{"id":14,"dish_title":"Chicken Parmesan","cost":950,"time_to_cook":2100,"is_available":true},{"id":3,"dish_title":"Chicken Wings (6 pcs)","cost":650,"time_to_cook":1200,"is_available":true},{"id":35,"dish_title":"Chocolate Lava Cake","cost":500,"time_to_cook":1200,"is_available":true},{"id":22,"dish_title":"Classic Burger","cost":750,"time_to_cook":1200,"is_available":true},{"id":28,"dish_title":"Club Sandwich","cost":550,"time_to_cook":900,"is_available":true},{"id":48,"dish_title":"Coffee (Hot/Iced)","cost":200,"time_to_cook":300,"is_available":true},{"id":40,"dish_title":"Crème Brûlée","cost":500,"time_to_cook":1200,"is_available":true},{"id":53,"dish_title":"Duck Confit","cost":1800,"time_to_cook":2400,"is_available":true},{"id":32,"dish_title":"Enchiladas (3 pcs)","cost":800,"time_to_cook":1500,"is_available":true},{"id":49,"dish_title":"Espresso","cost":150,"time_to_cook":300,"is_available":true},{"id":11,"dish_title":"Fettuccine Alfredo","cost":800,"time_to_cook":1200,"is_available":true},{"id":26,"dish_title":"Fish and Chips","cost":800,"time_to_cook":1500,"is_available":true},{"id":4,"dish_title":"French Fries","cost":350,"time_to_cook":720,"is_available":true},{"id":45,"dish_title":"Fresh Juice - Orange","cost":250,"time_to_cook":480,"is_available":true},{"id":41,"dish_title":"Fresh Lemonade","cost":200,"time_to_cook":300,"is_available":true},{"id":1,"dish_title":"Garlic Bread","cost":450,"time_to_cook":600,"is_available":true},{"id":21,"dish_title":"Green Curry","cost":850,"time_to_cook":1500,"is_available":true},{"id":24,"dish_title":"Grilled Salmon","cost":1100,"time_to_cook":1500,"is_available":true},{"id":37,"dish_title":"Ice Cream Sundae","cost":350,"time_to_cook":600,"is_available":true},{"id":42,"dish_title":"Iced Tea","cost":150,"time_to_cook":300,"is_available":true},{"id":12,"dish_title":"Lasagna","cost":900,"time_to_cook":2400,"is_available":true},{"id":51,"dish_title":"Lobster Thermidor","cost":2500,"time_to_cook":2700,"is_available":true},{"id":8,"dish_title":"Margherita Pizza","cost":850,"time_to_cook":1500,"is_available":true},{"id":46,"dish_title":"Milkshake - Chocolate","cost":350,"time_to_cook":600,"is_available":true},{"id":47,"dish_title":"Milkshake - Strawberry","cost":350,"time_to_cook":600,"is_available":true},{"id":33,"dish_title":"Nachos Supreme","cost":650,"time_to_cook":900,"is_available":true},{"id":5,"dish_title":"Onion Rings","cost":400,"time_to_cook":900,"is_available":true},{"id":55,"dish_title":"Osso Buco","cost":2000,"time_to_cook":3600,"is_available":true},{"id":15,"dish_title":"Pad Thai","cost":700,"time_to_cook":1200,"is_available":true},{"id":54,"dish_title":"Paella Seafood","cost":2200,"time_to_cook":2700,"is_available":true},{"id":9,"dish_title":"Pepperoni Pizza","cost":950,"time_to_cook":1500,"is_available":true},{"id":20,"dish_title":"Pho Beef Noodle Soup","cost":750,"time_to_cook":1800,"is_available":true},{"id":31,"dish_title":"Quesadilla","cost":600,"time_to_cook":900,"is_available":true},{"id":13,"dish_title":"Risotto Mushroom","cost":850,"time_to_cook":1800,"is_available":true},{"id":44,"dish_title":"Smoothie - Berry","cost":300,"time_to_cook":600,"is_available":true},{"id":43,"dish_title":"Smoothie - Mango","cost":300,"time_to_cook":600,"is_available":true},{"id":10,"dish_title":"Spaghetti Carbonara","cost":750,"time_to_cook":1200,"is_available":true},{"id":7,"dish_title":"Spring Rolls (4 pcs)","cost":500,"time_to_cook":900,"is_available":true},{"id":25,"dish_title":"Steak Frites (8oz)","cost":1500,"time_to_cook":1800,"is_available":true},{"id":6,"dish_title":"Stuffed Mushrooms","cost":600,"time_to_cook":1200,"is_available":true},{"id":17,"dish_title":"Sushi Platter (12 pcs)","cost":1200,"time_to_cook":1800,"is_available":true},{"id":18,"dish_title":"Teriyaki Chicken","cost":850,"time_to_cook":1500,"is_available":true},{"id":34,"dish_title":"Tiramisu","cost":450,"time_to_cook":900,"is_available":true}]}"#;
pub const UPDATED_MENU_RESPONSE: &str = r#"{"menu":[{"id":38,"dish_title":"Apple Pie","cost":400,"time_to_cook":1200,"is_available":true},{"id":30,"dish_title":"Beef Tacos (3 pcs)","cost":750,"time_to_cook":1080,"is_available":true},{"id":52,"dish_title":"Beef Wellington","cost":2800,"time_to_cook":3000,"is_available":true},{"id":19,"dish_title":"Bibimbap","cost":800,"time_to_cook":1500,"is_available":true},{"id":39,"dish_title":"Brownie with Ice Cream","cost":480,"time_to_cook":900,"is_available":true},{"id":2,"dish_title":"Bruschetta","cost":550,"time_to_cook":900,"is_available":true},{"id":50,"dish_title":"Cappuccino","cost":250,"time_to_cook":420,"is_available":true},{"id":23,"dish_title":"Cheeseburger Deluxe","cost":850,"time_to_cook":1320,"is_available":true},{"id":36,"dish_title":"Cheesecake","cost":450,"time_to_cook":900,"is_available":true},{"id":29,"dish_title":"Chicken Burrito","cost":700,"time_to_cook":1200,"is_available":true},{"id":27,"dish_title":"Chicken Caesar Salad","cost":650,"time_to_cook":900,"is_available":true},{"id":16,"dish_title":"Chicken Fried Rice","cost":650,"time_to_cook":1080,"is_available":true},{"id":14,"dish_title":"Chicken Parmesan","cost":950,"time_to_cook":2100,"is_available":true},{"id":3,"dish_title":"Chicken Wings (6 pcs)","cost":650,"time_to_cook":1200,"is_available":true},{"id":35,"dish_title":"Chocolate Lava Cake","cost":500,"time_to_cook":1200,"is_available":true},{"id":22,"dish_title":"Classic Burger","cost":750,"time_to_cook":1200,"is_available":true},{"id":28,"dish_title":"Club Sandwich","cost":550,"time_to_cook":900,"is_available":true},{"id":48,"dish_title":"Coffee (Hot/Iced)","cost":200,"time_to_cook":300,"is_available":true},{"id":40,"dish_title":"Crème Brûlée","cost":500,"time_to_cook":1200,"is_available":true},{"id":53,"dish_title":"Duck Confit","cost":1800,"time_to_cook":2400,"is_available":true},{"id":32,"dish_title":"Enchiladas (3 pcs)","cost":800,"time_to_cook":1500,"is_available":true},{"id":49,"dish_title":"Espresso","cost":150,"time_to_cook":300,"is_available":true},{"id":11,"dish_title":"Fettuccine Alfredo","cost":800,"time_to_cook":1200,"is_available":true},{"id":26,"dish_title":"Fish and Chips","cost":800,"time_to_cook":1500,"is_available":true},{"id":4,"dish_title":"French Fries","cost":350,"time_to_cook":720,"is_available":true},{"id":45,"dish_title":"Fresh Juice - Orange","cost":250,"time_to_cook":480,"is_available":true},{"id":41,"dish_title":"Fresh Lemonade","cost":200,"time_to_cook":300,"is_available":true},{"id":1,"dish_title":"Garlic Bread","cost":450,"time_to_cook":600,"is_available":true},{"id":21,"dish_title":"Green Curry","cost":850,"time_to_cook":1500,"is_available":true},{"id":24,"dish_title":"Grilled Salmon","cost":1100,"time_to_cook":1500,"is_available":true},{"id":37,"dish_title":"Ice Cream Sundae","cost":350,"time_to_cook":600,"is_available":true},{"id":42,"dish_title":"Iced Tea","cost":150,"time_to_cook":300,"is_available":true},{"id":12,"dish_title":"Lasagna","cost":900,"time_to_cook":2400,"is_available":true},{"id":51,"dish_title":"Lobster Thermidor","cost":2500,"time_to_cook":2700,"is_available":true},{"id":8,"dish_title":"Margherita Pizza","cost":850,"time_to_cook":1500,"is_available":true},{"id":46,"dish_title":"Milkshake - Chocolate","cost":350,"time_to_cook":600,"is_available":true},{"id":47,"dish_title":"Milkshake - Strawberry","cost":350,"time_to_cook":600,"is_available":true},{"id":33,"dish_title":"Nachos Supreme","cost":650,"time_to_cook":900,"is_available":true},{"id":5,"dish_title":"Onion Rings","cost":400,"time_to_cook":900,"is_available":true},{"id":55,"dish_title":"Osso Buco","cost":2000,"time_to_cook":3600,"is_available":true},{"id":15,"dish_title":"Pad Thai","cost":700,"time_to_cook":1200,"is_available":true},{"id":54,"dish_title":"Paella Seafood","cost":2200,"time_to_cook":2700,"is_available":true},{"id":9,"dish_title":"Pepperoni Pizza","cost":950,"time_to_cook":1500,"is_available":true},{"id":20,"dish_title":"Pho Beef Noodle Soup","cost":750,"time_to_cook":1800,"is_available":true},{"id":31,"dish_title":"Quesadilla","cost":600,"time_to_cook":900,"is_available":true},{"id":13,"dish_title":"Risotto Mushroom","cost":850,"time_to_cook":1800,"is_available":true},{"id":44,"dish_title":"Smoothie - Berry","cost":300,"time_to_cook":600,"is_available":true},{"id":43,"dish_title":"Smoothie - Mango","cost":300,"time_to_cook":600,"is_available":true},{"id":10,"dish_title":"Spaghetti Carbonara","cost":750,"time_to_cook":1200,"is_available":true},{"id":7,"dish_title":"Spring Rolls (4 pcs)","cost":500,"time_to_cook":900,"is_available":true},{"id":25,"dish_title":"Steak Frites (8oz)","cost":1500,"time_to_cook":1800,"is_available":true},{"id":6,"dish_title":"Stuffed Mushrooms","cost":600,"time_to_cook":1200,"is_available":true},{"id":17,"dish_title":"Sushi Platter (12 pcs)","cost":1200,"time_to_cook":1800,"is_available":true},{"id":18,"dish_title":"Teriyaki Chicken","cost":850,"time_to_cook":1500,"is_available":true},{"id":34,"dish_title":"Tiramisu","cost":450,"time_to_cook":900,"is_available":true},{"id":56,"dish_title":"some_dish_title","cost":400,"time_to_cook":4500,"is_available":false}]}"#;
