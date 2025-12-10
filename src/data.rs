use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
Product {
    id: 1,
    name: "ASUS Wireless Gaming Router".to_string(),
    price: 299.99,
    description: "Refurbished (Good) ASUS ROG Rapture WiFi 6 Wireless Gaming Router (GT-AX11000)".to_string(),
    image: "/asusrouter.jpg".to_string()
        },
Product {
    id: 2,
    name: "Samsung Galaxy Buds FE".to_string(),
    price: 129.99,
    description: "Wireless earbuds with active noise cancellation, long battery life, and a comfortable in-ear fit—perfect for commuting or workouts.".to_string(),
    image: "/galaxybuds.jpg".to_string()
},
Product {
    id: 3,
    name: "Sony WH-CH520 Wireless Headphones".to_string(),
    price: 89.99,
    description: "Lightweight Bluetooth headphones offering up to 50 hours of battery life with rich sound and crystal-clear calls.".to_string(),
    image: "/sonyheadphones.jpg".to_string()
},
Product {
    id: 4,
    name: "Apple AirTag 4-Pack".to_string(),
    price: 119.99,
    description: "Track your keys, wallet, luggage, and more using Apple’s Find My network—simple setup, reliable tracking.".to_string(),
    image: "/airtag.jpg".to_string()
},
Product {
    id: 5,
    name: "Logitech MX Master 3S Wireless Mouse".to_string(),
    price: 139.99,
    description: "The ultimate ergonomic productivity mouse featuring MagSpeed scrolling, customizable buttons, and multi-device pairing.".to_string(),
    image: "/mxmaster3s.jpg".to_string()
},
Product {
    id: 6,
    name: "Google Nest Mini (2nd Gen)".to_string(),
    price: 69.99,
    description: "A compact smart speaker powered by Google Assistant—play music, control smart home devices, and ask questions hands-free.".to_string(),
    image: "/nestmini.jpg".to_string()
},
Product {
    id: 7,
    name: "Dyson V8 Origin+ Cordless Vacuum".to_string(),
    price: 399.99,
    description: "Powerful, lightweight cordless vacuum with up to 40 minutes of fade-free suction and advanced filtration.".to_string(),
    image: "/dysonv8.jpg".to_string()
},
Product {
    id: 8,
    name: "Nintendo Switch Pro Controller".to_string(),
    price: 99.99,
    description: "A premium wireless controller offering HD Rumble, motion controls, and exceptional comfort for long gaming sessions.".to_string(),
    image: "/switchpro.jpg".to_string()
},
Product {
    id: 9,
    name: "Insignia 43\" 4K UHD Smart Fire TV".to_string(),
    price: 299.99,
    description: "Crisp 4K picture quality with built-in Fire TV for quick access to streaming apps, Alexa integration, and voice control.".to_string(),
    image: "/insignia43.jpg".to_string()
},
Product {
    id: 10,
    name: "Fitbit Charge 6 Fitness Tracker".to_string(),
    price: 199.99,
    description: "Track steps, workouts, heart rate, sleep, and more with this advanced fitness tracker featuring Google Maps integration.".to_string(),
    image: "/fitbitcharge6.jpg".to_string()
}

    ]
}