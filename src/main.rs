use ::entity::junction::{self, Entity as Junction};
use ::entity::sample_a::{self, Entity as SampleA};
use ::entity::sample_b::{self, Entity as SampleB};
use ::entity::sample_c::{self, Entity as SampleC};
use ::entity::sample_d::{self, Entity as SampleD};
use ::entity::sample_e::{self, Entity as SampleE};
use ::entity::sample_f::{self, Entity as SampleF};
use sea_orm::{Database, DbErr, EntityTrait, LoaderTrait, Set};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), DbErr> {
    // connnect database
    let db = Database::connect("postgres://postgres:postgres@localhost:5433/test_db").await?;
    //* One to One */
    {
        let sample_a_model1: sample_a::ActiveModel = sample_a::ActiveModel {
            value: Set("sample_a 1".to_string()),
            ..Default::default()
        };
        let sample_a_model2: sample_a::ActiveModel = sample_a::ActiveModel {
            value: Set("sample_a 2".to_string()),
            ..Default::default()
        };
        let sample_a_model3: sample_a::ActiveModel = sample_a::ActiveModel {
            value: Set("sample_a 3".to_string()),
            ..Default::default()
        };
        let sample_a_res =
            SampleA::insert_many([sample_a_model1, sample_a_model2, sample_a_model3])
                .exec(&db)
                .await?;
        println!("{:?}", sample_a_res);
        let sample_b_model1: sample_b::ActiveModel = sample_b::ActiveModel {
            value: Set("sample_b 1".to_string()),
            sample_a_id: Set(1),
            ..Default::default()
        };
        let sample_b_model2: sample_b::ActiveModel = sample_b::ActiveModel {
            value: Set("sample_b 2".to_string()),
            sample_a_id: Set(2),
            ..Default::default()
        };
        let sample_b_model3: sample_b::ActiveModel = sample_b::ActiveModel {
            value: Set("sample_b 3".to_string()),
            sample_a_id: Set(3),
            ..Default::default()
        };
        let sample_b_res =
            SampleB::insert_many([sample_b_model1, sample_b_model2, sample_b_model3])
                .exec(&db)
                .await?;
        println!("{:?}", sample_b_res);

        //エラーになる
        // let sample_b_model4: sample_b::ActiveModel = sample_b::ActiveModel {
        //     value: Set("sample_b 4".to_string()),
        //     sample_a_id: Set(4),
        //     ..Default::default()
        // };
        // let error_sample_b_res = SampleB::insert(sample_b_model4).exec(&db).await?;
        // println!("{:?}", error_sample_b_res);
        //[Error Message]
        //Error: Query(SqlxError(Database(PgDatabaseError { severity: Error, code: "23503", message: "insert or update on table \"sample_b\" violates foreign key constraint \"sample_b_sample_a_id_fkey\"", detail: Some("Key (sample_a_id)=(4) is not present in table \"sample_a\"."), hint: None, position: None, where: None, schema: Some("public"), table: Some("sample_b"), column: None, data_type: None, constraint: Some("sample_b_sample_a_id_fkey"), file: Some("ri_triggers.c"), line: Some(2608), routine: Some("ri_ReportViolation") })))
        //load_one
        let sample_bs = SampleB::find().all(&db).await?;
        let sample_as = sample_bs.load_one(SampleA, &db).await?;
        for sample_a in sample_as {
            match sample_a {
                Some(sample_a) => {
                    println!("sample_a: {:?}", sample_a);
                }
                None => {
                    println!("None");
                }
            }
        }
    }
    //* One to Many */
    {
        let sample_c_model1: sample_c::ActiveModel = sample_c::ActiveModel {
            value: Set("sample_c 1".to_string()),
            ..Default::default()
        };
        let sample_c_res = SampleC::insert(sample_c_model1).exec(&db).await?;
        println!("{:?}", sample_c_res);
        let sample_d_model1: sample_d::ActiveModel = sample_d::ActiveModel {
            value: Set("sample_d 1".to_string()),
            sample_c_id: Set(1),
            ..Default::default()
        };
        let sample_d_model2: sample_d::ActiveModel = sample_d::ActiveModel {
            value: Set("sample_d 2".to_string()),
            sample_c_id: Set(1),
            ..Default::default()
        };
        let sample_d_model3: sample_d::ActiveModel = sample_d::ActiveModel {
            value: Set("sample_d 3".to_string()),
            sample_c_id: Set(1),
            ..Default::default()
        };
        let sample_d_res =
            SampleD::insert_many([sample_d_model1, sample_d_model2, sample_d_model3])
                .exec(&db)
                .await?;
        println!("{:?}", sample_d_res);
        //エラーになる
        // let sample_d_model4 = sample_d::ActiveModel {
        //     value: Set("sample_d 4".to_string()),
        //     sample_c_id: Set(2),
        //     ..Default::default()
        // };
        // let error_sample_d_res = SampleD::insert(sample_d_model4).exec(&db).await?;
        // println!("{:?}", error_sample_d_res);
        //Error: Query(SqlxError(Database(PgDatabaseError { severity: Error, code: "23503", message: "insert or update on table \"sample_d\" violates foreign key constraint \"sample_d_sample_c_id_fkey\"", detail: Some("Key (sample_c_id)=(2) is not present in table \"sample_c\"."), hint: None, position: None, where: None, schema: Some("public"), table: Some("sample_d"), column: None, data_type: None, constraint: Some("sample_d_sample_c_id_fkey"), file: Some("ri_triggers.c"), line: Some(2608), routine: Some("ri_ReportViolation") })))
        //load_many
        let sample_c_model2: sample_c::ActiveModel = sample_c::ActiveModel {
            value: Set("sample_c 2".to_string()),
            ..Default::default()
        };
        let sample_c_res = SampleC::insert(sample_c_model2).exec(&db).await?;
        println!("{:?}", sample_c_res);
        let sample_d_model1: sample_d::ActiveModel = sample_d::ActiveModel {
            value: Set("sample_d 1".to_string()),
            sample_c_id: Set(2),
            ..Default::default()
        };
        let sample_d_model2: sample_d::ActiveModel = sample_d::ActiveModel {
            value: Set("sample_d 2".to_string()),
            sample_c_id: Set(2),
            ..Default::default()
        };
        let sample_d_res = SampleD::insert_many([sample_d_model1, sample_d_model2])
            .exec(&db)
            .await?;
        println!("{:?}", sample_d_res);
        let sample_cs = SampleC::find().all(&db).await?;
        let sample_ds_vec = sample_cs.load_many(SampleD, &db).await?;
        for sample_ds in sample_ds_vec {
            println!("sample_ds: {:?}", sample_ds);
        }
    }
    {
        let sample_e_model1: sample_e::ActiveModel = sample_e::ActiveModel {
            value: Set("sample_e 1".to_string()),
            ..Default::default()
        };
        let sample_e_model2: sample_e::ActiveModel = sample_e::ActiveModel {
            value: Set("sample_e 2".to_string()),
            ..Default::default()
        };
        let sample_e_model3: sample_e::ActiveModel = sample_e::ActiveModel {
            value: Set("sample_e 3".to_string()),
            ..Default::default()
        };
        let sample_e_res =
            SampleE::insert_many([sample_e_model1, sample_e_model2, sample_e_model3])
                .exec(&db)
                .await?;
        println!("{:?}", sample_e_res);
        let sample_f_model1: sample_f::ActiveModel = sample_f::ActiveModel {
            value: Set("sample_e 1".to_string()),
            ..Default::default()
        };
        let sample_f_model2: sample_f::ActiveModel = sample_f::ActiveModel {
            value: Set("sample_e 2".to_string()),
            ..Default::default()
        };
        let sample_f_res = SampleF::insert_many([sample_f_model1, sample_f_model2])
            .exec(&db)
            .await?;
        println!("{:?}", sample_f_res);
        let junction_model1: junction::ActiveModel = junction::ActiveModel {
            sample_e_id: Set(1),
            sample_f_id: Set(1),
            ..Default::default()
        };
        let junction_model2: junction::ActiveModel = junction::ActiveModel {
            sample_e_id: Set(2),
            sample_f_id: Set(1),
            ..Default::default()
        };
        let junction_model3: junction::ActiveModel = junction::ActiveModel {
            sample_e_id: Set(3),
            sample_f_id: Set(1),
            ..Default::default()
        };
        let junction_model4: junction::ActiveModel = junction::ActiveModel {
            sample_e_id: Set(1),
            sample_f_id: Set(2),
            ..Default::default()
        };
        let junction_model5: junction::ActiveModel = junction::ActiveModel {
            sample_e_id: Set(2),
            sample_f_id: Set(2),
            ..Default::default()
        };
        //これはエラーにならない
        // let junction_model6: junction::ActiveModel = junction::ActiveModel {
        //     sample_e_id: Set(2),
        //     sample_f_id: Set(2),
        //     ..Default::default()
        // };
        let sample_junction_res = Junction::insert_many([
            junction_model1,
            junction_model2,
            junction_model3,
            junction_model4,
            junction_model5,
        ])
        .exec(&db)
        .await?;
        println!("{:?}", sample_junction_res);
        let sample_es = SampleE::find().all(&db).await?;
        let sample_fs_vec = sample_es.load_many_to_many(SampleF, Junction, &db).await?;
        for sample_fs in sample_fs_vec {
            println!("sample_fs: {:?}", sample_fs);
        }
        let sample_fs = SampleF::find().all(&db).await?;
        let sample_es_vec = sample_fs.load_many_to_many(SampleE, Junction, &db).await?;
        for sample_es in sample_es_vec {
            println!("sample_es: {:?}", sample_es);
        }
    }
    Ok(())
}
