use crate::{
    db::Database,
    models::{CreateTaskDTO, DeleteTaskDTO, PutTaskDTO, Task, TaskPoints},
};

pub async fn _create_task(
    db: &Database,
    create_task_dto: CreateTaskDTO,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO tasks (description, points, link, task_button_text) VALUES ($1, $2, $3, $4)",
    )
    .bind(create_task_dto.description)
    .bind(create_task_dto.points)
    .bind(create_task_dto.link)
    .bind(create_task_dto.task_button_text)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn _delete_task(
    db: &Database,
    delete_task_dto: DeleteTaskDTO,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(delete_task_dto.task_id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}

pub async fn _get_tasks(db: &Database) -> Result<Vec<Task>, sqlx::Error> {
    let tasks: Vec<Task> = sqlx::query_as(
        "SELECT id, description, points, link, task_button_text FROM tasks",
    )
    .fetch_all(db)
    .await?;
    Ok(tasks)
}

pub async fn _get_points_for_task(db: &Database, task_id: i32) -> Result<i32, sqlx::Error> {
    let task = sqlx::query_as::<_, TaskPoints>("SELECT points FROM tasks WHERE id = $1")
        .bind(task_id)
        .fetch_one(db)
        .await?;
    Ok(task.points)
}

pub async fn _put_task(db: &Database, put_task_dto: PutTaskDTO) -> Result<(), sqlx::Error> {
    let mut description = put_task_dto.description;
    let mut points = put_task_dto.points;
    let mut link = put_task_dto.link;
    let mut task_button_text = put_task_dto.task_button_text;

    let task = sqlx::query_as::<_, Task>(
        "SELECT id, description, points, link, task_button_text FROM tasks WHERE id = $1",
    )
    .bind(put_task_dto.task_id)
    .fetch_one(db)
    .await?;

    if description.is_none() {
        description = Some(task.description);
    }

    if points.is_none() {
        points = Some(task.points);
    }

    if link.is_none() {
        link = Some(task.link.unwrap_or_default());
    }

    if task_button_text.is_none(){
        task_button_text = Some(task.task_button_text.unwrap_or_default());
    }

    let text = task_button_text.clone();

    sqlx::query("UPDATE tasks SET description = $1, points = $2, link = $3, task_button_text = $4 WHERE id = $5")
        .bind(description)
        .bind(points)
        .bind(link)
        .bind(text)
        .bind(put_task_dto.task_id)
        .execute(db)
        .await?;

    Ok(())
}
