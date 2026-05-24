-- database/seeds.sql
-- Datos iniciales y de ejemplo para el MVP de NebriPop

-- Insertar categorías iniciales (tipo Wallapop)
INSERT INTO categories (name) VALUES
    ('Electrónica'),
    ('Hogar'),
    ('Moda'),
    ('Vehículos'),
    ('Deporte'),
    ('Libros'),
    ('Videojuegos'),
    ('Muebles'),
    ('Herramientas'),
    ('Jardín'),
    ('Bebés y niños'),
    ('Mascotas'),
    ('Coleccionismo'),
    ('Otros');

-- Usuario de prueba (No incluye datos personales reales)
-- password_hash es un string dummy, ya que el login no está implementado
INSERT INTO users (name, email, password_hash, phone, location) VALUES
    ('Usuario Demo', 'demo@nebripop.local', 'dummy_hash_no_real_password_12345', '555-0000', 'Madrid');

-- Productos de ejemplo
-- user_id = 1, categorías según su ID correspondiente (1: Electrónica, 5: Deporte, 8: Muebles, 3: Moda, 9: Herramientas, 6: Libros, 7: Videojuegos, 2: Hogar)
INSERT INTO products (user_id, category_id, title, description, price, condition, location, status) VALUES
    (1, 1, 'iPhone 12 usado', 'iPhone 12 en buen estado, batería al 85%.', 350.00, 'used', 'Madrid', 'available'),
    (1, 5, 'Bicicleta de montaña', 'Bici con poco uso, 21 marchas.', 120.00, 'good', 'Barcelona', 'available'),
    (1, 8, 'Sofá de 3 plazas', 'Sofá muy cómodo, color gris claro.', 80.00, 'good', 'Valencia', 'available'),
    (1, 3, 'Chaqueta vaquera', 'Talla M, marca conocida, sin desgaste.', 25.00, 'like_new', 'Sevilla', 'available'),
    (1, 9, 'Taladro eléctrico', 'Potente, incluye brocas básicas.', 45.00, 'used', 'Bilbao', 'available'),
    (1, 6, 'Libro de programación Rust', 'The Rust Programming Language, como nuevo.', 30.00, 'like_new', 'Zaragoza', 'available'),
    (1, 7, 'PlayStation 4', 'Incluye 2 mandos y cables.', 150.00, 'used', 'Málaga', 'available'),
    (1, 2, 'Silla de oficina', 'Ergonómica, regulable en altura.', 40.00, 'good', 'Alicante', 'available');
