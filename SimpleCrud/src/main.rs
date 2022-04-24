mod fruits_table;
use fruits_table::prelude::*;
mod suppliers_table;
use suppliers_table::prelude::*;

use anyhow::Result;
use chrono::Local;
use sea_orm::entity::Set;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use sea_orm::{ConnectionTrait, Database, Schema};

#[async_std::main]
async fn main() -> Result<()> {
    // Read the database environment from the .env file
    let env_database_url = include_str!("../.env").trim();
    // split the env url
    let split_url: Vec<&str> = env_database_url.split("=").collect();
    // get item with the format `database_backend://username:password@localhost/database_name`
    let database_url = split_url[1];
    let db = Database::connect(database_url).await?;
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    println!("=========================== create table ===========================");
    let create_table_op = db
        .execute(builder.build(&schema.create_table_from_entity(Fruits)))
        .await;
    println!(
        "`Created table fruits` {:?}",
        match create_table_op {
            Ok(_) => "Operation Successful".to_owned(),
            Err(e) => format!("Unsuccessful - Error {:?}", e),
        }
    );

    // Get current system time
    let now = chrono::offset::Local::now();

    // Convert system time to `NaiveDateTime` since SeaORM `DateTime` expects this;
    let naive_system_time = now.naive_local();

    let fruit_01 = FruitsActiveModel {
        name: Set("Apple".to_owned()),
        datetime_utc: Set(naive_system_time),
        unit_price: Set(2),
        sku: Set(Some("FM2022AKB40".to_owned())),
        ..Default::default()
    };

    println!("=========================== insert one ===========================");
    let fruit_insert_operation = Fruits::insert(fruit_01).exec(&db).await;
    match fruit_insert_operation {
        Ok(operation) => println!("INSERTED ONE: {:?}", operation),
        Err(e) => println!("inserted failed: {:?}", e),
    }

    let fruit_02 = FruitsActiveModel {
        name: Set("Banana".to_owned()),
        datetime_utc: Set(Local::now().naive_local()),
        unit_price: Set(2),
        sku: Set(Some("FM2022AKB41".to_owned())),
        ..Default::default()
    };

    let fruit_03 = FruitsActiveModel {
        name: Set("Pineapple".to_owned()),
        datetime_utc: Set(Local::now().naive_local()),
        unit_price: Set(8),
        sku: Set(Some("FM2022AKB42".to_owned())),
        ..Default::default()
    };

    let fruit_04 = FruitsActiveModel {
        name: Set("Mango".to_owned()),
        datetime_utc: Set(Local::now().naive_local()),
        unit_price: Set(6),
        sku: Set(Some("FM2022AKB43".to_owned())),
        ..Default::default()
    };

    println!("=========================== insert many ===========================");
    let fruit_insert_operation = Fruits::insert_many(vec![fruit_03, fruit_02, fruit_04])
        .exec(&db)
        .await;

    match fruit_insert_operation {
        Ok(operation) => println!("INSERTED MANY: {:?}", operation),
        Err(e) => println!("inserted failed: {:?}", e),
    }

    println!("=========================== find all ===========================");

    let fruits_table_rows = Fruits::find().all(&db).await;
    match fruits_table_rows {
        Ok(rows) => {
            // println!("Fruits table rows: {:?}", rows);
            for row in rows {
                println!("{:?}", row);
            }
        }
        Err(e) => println!("Failed to find rows: {:?}", e),
    }

    println!("=========================== find one ===========================");
    let fruits_by_id = Fruits::find_by_id(4).one(&db).await;
    match fruits_by_id {
        Ok(row) => println!("Fruits by id: {:?}", row),
        Err(e) => println!("Failed to find row: {:?}", e),
    }

    println!("=========================== find filter ===========================");

    let find_pineapple = Fruits::find()
        .filter(FruitsColumn::Name.contains("pineapple")) // 不区分大小写
        .one(&db)
        .await;

    match find_pineapple {
        Ok(row) => println!("Fruits filter pineapple: {:?}", row),
        Err(e) => println!("Failed to find pineapple: {:?}", e),
    }

    println!("=========================== update ===========================");

    let find_pineapple = Fruits::find()
        .filter(FruitsColumn::Name.contains("pineapple")) // 不区分大小写
        .one(&db)
        .await?;

    println!("{:?}", find_pineapple.as_ref()); // Reference the `Model` instead of owning it

    // Update the `pineaple` column with a new unit price
    if let Some(pineapple_model) = find_pineapple {
        let mut pineapple_active_model: FruitsActiveModel = pineapple_model.into();
        pineapple_active_model.unit_price = Set(10);

        let updated_pineapple_model: FruitsModel = pineapple_active_model.update(&db).await?;

        println!("UPDATED PRICE: {:?}", updated_pineapple_model);
    } else {
        println!("`Pineapple` column not found.");
    }

    println!("=========================== delete ===========================");
    // Delete the `mango` row
    let find_mango = Fruits::find()
        .filter(FruitsColumn::Name.contains("mango")) // 不区分大小写
        .one(&db)
        .await;

    if find_mango.is_ok() {
        if let Some(mango_model) = find_mango? {
            println!("Delete mango: {:?}", mango_model.delete(&db).await?);
        } else {
            println!("`Mango` column not found.");
        }
    }

    println!(
        "=========================== insert suppliers with foreign ==========================="
    );

    // Inserting Values into a Table with a Foreign Key
    let supplier_01 = SuppliersActiveModel {
        supplier_name: Set("John Doe".to_owned()),
        fruit_id: Set(1_i32),
        ..Default::default()
    };

    let supplier_02 = SuppliersActiveModel {
        supplier_name: Set("Jane Doe".to_owned()),
        fruit_id: Set(4_i32),
        ..Default::default()
    };

    let supplier_03 = SuppliersActiveModel {
        supplier_name: Set("Junior Doe".to_owned()),
        fruit_id: Set(5_i32),
        ..Default::default()
    };

    let supplier_insert_operation =
        Suppliers::insert_many(vec![supplier_01, supplier_02, supplier_03])
            .exec(&db)
            .await;

    match supplier_insert_operation {
        Ok(inserted_rows) => {
            println!("Inserted suppliers rows: {:?}", inserted_rows);
        }
        Err(e) => {
            println!("Failed to insert rows: {:?}", e);
        }
    }

    println!("=========================== select related tables ===========================");

    let who_supplies = Suppliers::find().find_with_related(Fruits).all(&db).await?;
    dbg!(who_supplies);

    Ok(())
}
