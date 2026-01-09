use crate::rest::offer::clean_old_offers::delete_all_offers;
use crate::state::{AppState, IntegrationTestGroup};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestInitializeArguments {
    pub number_of_groups: usize,
}

pub async fn test_initialize(data: web::Data<AppState>, body: String) -> HttpResponse {
    let decoded = serde_json::from_str::<TestInitializeArguments>(&body);
    let test_initialize_args = match decoded {
        Ok(t) => t,
        Err(e) => {
            log::error!("Error decoding test initialize arguments: {}", e);
            return HttpResponse::BadRequest().body(format!("Invalid format {}", e));
        }
    };

    {
        let mut lock = data.test.lock().await;
        lock.started_at = Some(chrono::Utc::now());
        lock.finished_at = None;
        lock.number_of_groups = test_initialize_args.number_of_groups;
        lock.groups.clear();
    }
    delete_all_offers(data.clone()).await;
    HttpResponse::Ok().body("New test initialized successfully")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestStartArguments {
    pub group: String,
}

pub async fn test_start(data: web::Data<AppState>, body: String) -> HttpResponse {
    let decoded = serde_json::from_str::<TestStartArguments>(&body);
    let test_start_args = match decoded {
        Ok(t) => t,
        Err(e) => {
            log::error!("Error decoding test start arguments: {}", e);
            return HttpResponse::BadRequest().body(format!("Invalid format {}", e));
        }
    };

    let mut lock = data.test.lock().await;
    {
        let entry = lock
            .groups
            .entry(test_start_args.group)
            .or_insert_with(IntegrationTestGroup::default);


        if entry.started_at.is_some() {
            return HttpResponse::BadRequest().body("Test already in progress for this group");
        }
        entry.started_at = Some(chrono::Utc::now());
        entry.finished_at = None;
    }
    if lock.groups.len() > lock.number_of_groups {
        return HttpResponse::BadRequest()
            .body("Number of test groups exceeded the initialized number");
    }

    HttpResponse::Ok().body("Test started successfully")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestFinishArguments {
    pub group: String,
    pub success: bool,
}

pub async fn test_finish(data: web::Data<AppState>, body: String) -> HttpResponse {
    let decoded = serde_json::from_str::<TestFinishArguments>(&body);
    let test_finish_args = match decoded {
        Ok(t) => t,
        Err(e) => {
            log::error!("Error decoding test finish arguments: {}", e);
            return HttpResponse::BadRequest().body(format!("Invalid format {}", e));
        }
    };

    let mut lock = data.test.lock().await;
    if lock.finished_at.is_some() {
        return HttpResponse::BadRequest().body("Test already finished");
    }
    let entry = lock.groups.get_mut(&test_finish_args.group);
    let entry = match entry {
        Some(e) => e,
        None => {
            return HttpResponse::BadRequest().body(format!(
                "No test was started for group {}",
                test_finish_args.group
            ));
        }
    };

    if entry.started_at.is_none() {
        return HttpResponse::BadRequest().body(format!(
            "No test was started for group {}",
            test_finish_args.group
        ));
    }
    if entry.finished_at.is_some() {
        return HttpResponse::BadRequest().body(format!(
            "Test already finished for this group {}",
            test_finish_args.group
        ));
    }
    entry.finished_at = Some(chrono::Utc::now());
    entry.success = Some(test_finish_args.success);

    // check if all tests are finished

    if lock.groups.len() > lock.number_of_groups {
        return HttpResponse::BadRequest()
            .body("Number of test groups exceeded the initialized number");
    }
    if lock.groups.len() == lock.number_of_groups {
        let all_finished = lock.groups.values().all(|g| g.finished_at.is_some());
        if all_finished {
            lock.finished_at = Some(chrono::Utc::now());
        }
    }

    HttpResponse::Ok().body("Test finished successfully")
}

pub async fn test_status(data: web::Data<AppState>) -> HttpResponse {
    let lock = data.test.lock().await;
    let response = serde_json::to_string(&*lock).unwrap_or_else(|_| "{}".to_string());
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}
