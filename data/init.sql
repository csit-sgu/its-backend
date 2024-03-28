CREATE VIEW aggregated_tasks AS (
    SELECT
        t.id AS task_id,
        t.taskable_type AS task_type,
        t.deadline_at AS task_deadline,
        t.account_id AS task_account_id,
        t.created_at AS task_created_at,
        ts.title AS task_transition_title,
        tr.transitioned_at AS task_transitioned_at,
        obj.id AS object_id,
        obj.place_id AS object_place_id,
        st_latitude(p.location) AS place_lat,
        st_longitude(p.location) AS place_lon,
        r.id AS region_id,
        r.title AS region_title
    FROM tasks AS t
    JOIN task_transitions tr ON tr.task_id = t.id
    JOIN task_stages ts ON ts.id = tr.task_stage_id
    JOIN service_object_task sot ON sot.task_id = t.id
    JOIN service_objects obj ON obj.id = sot.service_object_id
    JOIN places p ON p.id = obj.place_id
    JOIN regions r ON r.id = p.region_id
);
