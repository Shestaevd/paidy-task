INSERT INTO roles (description)
VALUES ('Admin'),
       ('Waiter'),
       ('Cook');

INSERT INTO users (username, password, role_id)
SELECT 'admin',
       crypt('admin123', gen_salt('bf')),
       id
FROM roles
WHERE description = 'Admin'
ON CONFLICT (username) DO NOTHING;

INSERT INTO users (username, password, role_id)
SELECT 'John',
       crypt('John123', gen_salt('bf')),
       id
FROM roles
WHERE description = 'Waiter'
ON CONFLICT (username) DO NOTHING;

INSERT INTO users (username, password, role_id)
SELECT 'Sam',
       crypt('Sam123', gen_salt('bf')),
       id
FROM roles
WHERE description = 'Waiter'
ON CONFLICT (username) DO NOTHING;

INSERT INTO users (username, password, role_id)
SELECT 'Mike',
       crypt('Mike123', gen_salt('bf')),
       id
FROM roles
WHERE description = 'Cook'
ON CONFLICT (username) DO NOTHING;

INSERT INTO users (username, password, role_id)
SELECT 'Kate',
       crypt('Kate123', gen_salt('bf')),
       id
FROM roles
WHERE description = 'Cook'
ON CONFLICT (username) DO NOTHING;

INSERT INTO menu (dish_title, cost, time_to_cook)
VALUES ('Garlic Bread', 450, 600),
       ('Bruschetta', 550, 900),
       ('Chicken Wings (6 pcs)', 650, 1200),
       ('French Fries', 350, 720),
       ('Onion Rings', 400, 900),
       ('Stuffed Mushrooms', 600, 1200),
       ('Spring Rolls (4 pcs)', 500, 900),
       ('Margherita Pizza', 850, 1500),
       ('Pepperoni Pizza', 950, 1500),
       ('Spaghetti Carbonara', 750, 1200),
       ('Fettuccine Alfredo', 800, 1200),
       ('Lasagna', 900, 2400),
       ('Risotto Mushroom', 850, 1800),
       ('Chicken Parmesan', 950, 2100),
       ('Pad Thai', 700, 1200),
       ('Chicken Fried Rice', 650, 1080),
       ('Sushi Platter (12 pcs)', 1200, 1800),
       ('Teriyaki Chicken', 850, 1500),
       ('Bibimbap', 800, 1500),
       ('Pho Beef Noodle Soup', 750, 1800),
       ('Green Curry', 850, 1500),
       ('Classic Burger', 750, 1200),
       ('Cheeseburger Deluxe', 850, 1320),
       ('Grilled Salmon', 1100, 1500),
       ('Steak Frites (8oz)', 1500, 1800),
       ('Fish and Chips', 800, 1500),
       ('Chicken Caesar Salad', 650, 900),
       ('Club Sandwich', 550, 900),
       ('Chicken Burrito', 700, 1200),
       ('Beef Tacos (3 pcs)', 750, 1080),
       ('Quesadilla', 600, 900),
       ('Enchiladas (3 pcs)', 800, 1500),
       ('Nachos Supreme', 650, 900),
       ('Tiramisu', 450, 900),
       ('Chocolate Lava Cake', 500, 1200),
       ('Cheesecake', 450, 900),
       ('Ice Cream Sundae', 350, 600),
       ('Apple Pie', 400, 1200),
       ('Brownie with Ice Cream', 480, 900),
       ('Crème Brûlée', 500, 1200),
       ('Fresh Lemonade', 200, 300),
       ('Iced Tea', 150, 300),
       ('Smoothie - Mango', 300, 600),
       ('Smoothie - Berry', 300, 600),
       ('Fresh Juice - Orange', 250, 480),
       ('Milkshake - Chocolate', 350, 600),
       ('Milkshake - Strawberry', 350, 600),
       ('Coffee (Hot/Iced)', 200, 300),
       ('Espresso', 150, 300),
       ('Cappuccino', 250, 420),
       ('Lobster Thermidor', 2500, 2700),
       ('Beef Wellington', 2800, 3000),
       ('Duck Confit', 1800, 2400),
       ('Paella Seafood', 2200, 2700),
       ('Osso Buco', 2000, 3600)
ON CONFLICT (dish_title) DO NOTHING;