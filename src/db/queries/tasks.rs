use crate::{
    db::Database,
    models::{CreateTaskDTO, TaskPoints, Tasks},
};

pub async fn _create_task(
    db: &Database,
    create_task_dto: CreateTaskDTO,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO tasks (description, points) VALUES ($1, $2)")
        .bind(create_task_dto.description)
        .bind(create_task_dto.points)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn _get_tasks(db: &Database) -> Result<Vec<Tasks>, sqlx::Error> {
    let tasks: Vec<Tasks> =
        sqlx::query_as("SELECT id, description, points, time_created FROM tasks")
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
