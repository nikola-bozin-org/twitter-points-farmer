ALTER TABLE users
ADD COLUMN finished_tasks INTEGER[] DEFAULT '{}';
