use sqlx::{SqlitePool, QueryBuilder, Sqlite};
use crate::models::{Product, ProductImage};
use crate::dto::product_dto::{CreateProductDto, ProductFiltersDto};
use crate::errors::AppError;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn find_all(pool: &SqlitePool, filters: &ProductFiltersDto) -> Result<Vec<Product>, AppError> {
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"SELECT 
                id, user_id, category_id, title, description, price, condition, 
                location, status, main_image_url, created_at, updated_at 
               FROM products"#
        );

        let mut has_where = false;
        let mut push_where_or_and = |qb: &mut QueryBuilder<Sqlite>| {
            if has_where {
                qb.push(" AND ");
            } else {
                qb.push(" WHERE ");
                has_where = true;
            }
        };

        if let Some(search) = &filters.search {
            if !search.trim().is_empty() {
                push_where_or_and(&mut query_builder);
                query_builder.push("(title LIKE ");
                query_builder.push_bind(format!("%{}%", search));
                query_builder.push(" OR description LIKE ");
                query_builder.push_bind(format!("%{}%", search));
                query_builder.push(")");
            }
        }

        if let Some(cat_id) = filters.category_id {
            push_where_or_and(&mut query_builder);
            query_builder.push("category_id = ");
            query_builder.push_bind(cat_id);
        }

        if let Some(min_p) = filters.min_price {
            push_where_or_and(&mut query_builder);
            query_builder.push("price >= ");
            query_builder.push_bind(min_p);
        }

        if let Some(max_p) = filters.max_price {
            push_where_or_and(&mut query_builder);
            query_builder.push("price <= ");
            query_builder.push_bind(max_p);
        }

        if let Some(cond) = &filters.condition {
            if !cond.trim().is_empty() {
                push_where_or_and(&mut query_builder);
                query_builder.push("condition = ");
                query_builder.push_bind(cond);
            }
        }

        if let Some(loc) = &filters.location {
            if !loc.trim().is_empty() {
                push_where_or_and(&mut query_builder);
                query_builder.push("location LIKE ");
                query_builder.push_bind(format!("%{}%", loc));
            }
        }

        if let Some(stat) = &filters.status {
            if !stat.trim().is_empty() {
                push_where_or_and(&mut query_builder);
                query_builder.push("status = ");
                query_builder.push_bind(stat);
            }
        }

        match filters.sort.as_deref() {
            Some("price_asc") => query_builder.push(" ORDER BY price ASC"),
            Some("price_desc") => query_builder.push(" ORDER BY price DESC"),
            _ => query_builder.push(" ORDER BY created_at DESC"),
        };

        let query = query_builder.build_query_as::<Product>();
        let products = query.fetch_all(pool).await?;

        Ok(products)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Product>, AppError> {
        let mut product = sqlx::query_as::<_, Product>(
            r#"SELECT 
                id, user_id, category_id, title, description, price, condition, 
                location, status, main_image_url, created_at, updated_at 
               FROM products 
               WHERE id = ?"#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        if let Some(ref mut p) = product {
            p.images = Some(Self::find_images_by_product_id(pool, id).await?);
        }

        Ok(product)
    }

    pub async fn find_by_user_id(pool: &SqlitePool, user_id: i64) -> Result<Vec<Product>, AppError> {
        let products = sqlx::query_as::<_, Product>(
            r#"SELECT 
                id, user_id, category_id, title, description, price, condition, 
                location, status, main_image_url, created_at, updated_at 
               FROM products 
               WHERE user_id = ?
               ORDER BY created_at DESC"#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(products)
    }

    pub async fn find_images_by_product_id(pool: &SqlitePool, product_id: i64) -> Result<Vec<ProductImage>, AppError> {
        let images = sqlx::query_as::<_, ProductImage>(
            r#"SELECT id, product_id, image_url, position, created_at 
               FROM product_images 
               WHERE product_id = ? 
               ORDER BY position ASC"#
        )
        .bind(product_id)
        .fetch_all(pool)
        .await?;

        Ok(images)
    }

    pub async fn add_image(pool: &SqlitePool, product_id: i64, image_url: &str) -> Result<ProductImage, AppError> {
        let images = Self::find_images_by_product_id(pool, product_id).await?;
        let position = images.len() as i64;

        let result = sqlx::query(
            r#"INSERT INTO product_images (product_id, image_url, position) VALUES (?, ?, ?)"#
        )
        .bind(product_id)
        .bind(image_url)
        .bind(position)
        .execute(pool)
        .await?
        .last_insert_rowid();

        // Si es la primera imagen, actualizamos el main_image_url del producto
        if position == 0 {
            sqlx::query("UPDATE products SET main_image_url = ? WHERE id = ?")
                .bind(image_url)
                .bind(product_id)
                .execute(pool)
                .await?;
        }

        let image = sqlx::query_as::<_, ProductImage>(
            r#"SELECT id, product_id, image_url, position, created_at FROM product_images WHERE id = ?"#
        )
        .bind(result)
        .fetch_one(pool)
        .await?;

        Ok(image)
    }

    pub async fn create(pool: &SqlitePool, dto: &CreateProductDto) -> Result<Product, AppError> {
        let status = dto.status.as_deref().unwrap_or("available");
        
        let result = sqlx::query(
            r#"INSERT INTO products (
                user_id, category_id, title, description, price, condition, location, status, main_image_url
               ) 
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(dto.user_id)
        .bind(dto.category_id)
        .bind(&dto.title)
        .bind(&dto.description)
        .bind(dto.price)
        .bind(&dto.condition)
        .bind(&dto.location)
        .bind(status)
        .bind(&dto.main_image_url)
        .execute(pool)
        .await?
        .last_insert_rowid();

        // Recuperamos el producto insertado para devolver los datos completos
        let product = Self::find_by_id(pool, result)
            .await?
            .ok_or_else(|| AppError::InternalServerError)?; // No debería pasar si acabamos de insertar

        Ok(product)
    }

    pub async fn update(pool: &SqlitePool, id: i64, dto: &crate::dto::product_dto::UpdateProductDto) -> Result<Product, AppError> {
        let status = dto.status.as_deref().unwrap_or("available");
        
        sqlx::query(
            r#"UPDATE products SET 
                category_id = ?, title = ?, description = ?, price = ?, 
                condition = ?, location = ?, status = ?, main_image_url = ?, updated_at = CURRENT_TIMESTAMP
               WHERE id = ? AND user_id = ?"#
        )
        .bind(dto.category_id)
        .bind(&dto.title)
        .bind(&dto.description)
        .bind(dto.price)
        .bind(&dto.condition)
        .bind(&dto.location)
        .bind(status)
        .bind(&dto.main_image_url)
        .bind(id)
        .bind(dto.user_id)
        .execute(pool)
        .await?;

        let product = Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Producto con ID {} no encontrado tras actualizar", id)))?;

        Ok(product)
    }

    pub async fn delete_image(pool: &SqlitePool, id: i64, product_id: i64) -> Result<(), AppError> {
        // Obtenemos la imagen para comprobar que existe y para poder borrar el archivo físico (opcional, aunque sqlite lo borra de db)
        let image = sqlx::query_as::<_, ProductImage>(
            "SELECT id, product_id, image_url, position, created_at FROM product_images WHERE id = ? AND product_id = ?"
        )
        .bind(id)
        .bind(product_id)
        .fetch_optional(pool)
        .await?;

        if let Some(img) = image {
            // Eliminar de base de datos
            sqlx::query("DELETE FROM product_images WHERE id = ?")
                .bind(id)
                .execute(pool)
                .await?;

            // Intentar eliminar el archivo físico si no es una url externa
            if !img.image_url.starts_with("http") {
                let _ = tokio::fs::remove_file(&img.image_url).await;
            }

            // Recalcular main_image_url si hemos borrado la principal
            // Si la posicion era 0, buscamos la nueva imagen en posicion 0 o la primera
            let remaining_images = Self::find_images_by_product_id(pool, product_id).await?;
            if let Some(first_img) = remaining_images.first() {
                sqlx::query("UPDATE products SET main_image_url = ? WHERE id = ?")
                    .bind(&first_img.image_url)
                    .bind(product_id)
                    .execute(pool)
                    .await?;
            } else {
                sqlx::query("UPDATE products SET main_image_url = NULL WHERE id = ?")
                    .bind(product_id)
                    .execute(pool)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
        let images = Self::find_images_by_product_id(pool, id).await?;
        for img in images {
            if !img.image_url.starts_with("http") {
                let _ = tokio::fs::remove_file(&img.image_url).await;
            }
        }
        
        sqlx::query("DELETE FROM product_images WHERE product_id = ?")
            .bind(id)
            .execute(pool)
            .await?;
            
        sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
            
        let _ = tokio::fs::remove_dir_all(&format!("uploads/products/{}", id)).await;
            
        Ok(())
    }
}
