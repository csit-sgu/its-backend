CREATE VIEW aggregated_tasks AS (
    SELECT
        t.id AS task_id,
        t.taskable_type AS task_type,
        t.deadline_at AS deadline,
        t.account_id AS account_id,
        t.created_at AS created,
        ts.title AS status,
        tr.transitioned_at AS timestamp,
        obj.place_id AS place_id,
        obj.id AS object_id,
        st_latitude(p.location) AS lat,
        st_longitude(p.location) AS lon,
        r.title AS region,
        r.id AS region_id
    FROM tasks AS t
    JOIN task_transitions tr ON tr.task_id = t.id
    JOIN task_stages ts ON ts.id = tr.task_stage_id
    JOIN service_object_task sot ON sot.task_id = t.id
    JOIN service_objects obj ON obj.id = sot.service_object_id
    JOIN places p ON p.id = obj.place_id
    JOIN regions r ON r.id = p.region_id
);
