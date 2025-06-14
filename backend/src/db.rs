use sqlx::{Error, PgPool};

pub async fn setup(pool: &PgPool) {
    //create an enum rating label
    create_enum_type_if_not_exists(
        pool,
        "rating",
        &["Halal", "Unconfirmed", "Unknown", "Haram"],
    )
    .await
    .unwrap();

    //create an enum rating label
    create_enum_type_if_not_exists(pool, "city", &["Delhi", "Noida"])
        .await
        .unwrap();

    create_enum_type_if_not_exists(pool, "spot_type", &["Restaurant, Hotel, Meatshop"])
        .await
        .unwrap();

    create_localities_table_if_not_exists(pool).await;
    create_places_table_if_not_exists(pool).await;
}
async fn create_localities_table_if_not_exists(pool: &PgPool) {
    //create table
    let _ = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS localities (
          id serial PRIMARY KEY,
          name TEXT NOT NULL,
          country_code TEXT NOT NULL,
          city city NOT NULL,
          latitude DOUBLE PRECISION NOT NULL,
          longitude DOUBLE PRECISION NOT NULL,
          locality_verifier TEXT NOT NULL
        );"#,
    )
    .execute(pool)
    .await;
}

async fn create_places_table_if_not_exists(pool: &PgPool) {
    //create table
    let _ = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS places (
          id serial PRIMARY KEY,
          name text NOT NULL,
          image_url text,
          halal_label rating NOT NULL,
          locality_id integer NOT NULL REFERENCES localities(id),
          address text,
          recommended bool NOT NULL,
          place_description text,
          label_description text NOT NULL,
          map_url text,
          mobile_number text,
          place_type spot_type NOT NULL
        );"#,
    )
    .execute(pool)
    .await;
}

async fn create_enum_type_if_not_exists(
    pool: &PgPool,
    enum_name: &str,
    variants: &[&str],
) -> Result<(), Error> {
    let exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM pg_type t
            JOIN pg_namespace n ON n.oid = t.typnamespace
            WHERE t.typname = $1
            AND n.nspname = 'public'
        )
        "#,
    )
    .bind(enum_name)
    .fetch_one(pool)
    .await?;

    if !exists {
        let variant_list = variants
            .iter()
            .map(|v| format!("'{}'", v))
            .collect::<Vec<_>>()
            .join(", ");

        let query = format!("CREATE TYPE {} AS ENUM ({})", enum_name, variant_list);

        sqlx::query(&query).execute(pool).await?;
    }

    Ok(())
}
